use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use autocorrect::ignorer::Ignorer;
use notify::Watcher as _;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

mod typocheck;

struct Backend {
    client: Client,
    work_dir: RwLock<PathBuf>,
    documents: Arc<RwLock<HashMap<Url, Arc<TextDocumentItem>>>>,
    ignorer: Arc<RwLock<Option<Ignorer>>>,
    diagnostics: Arc<RwLock<HashMap<Url, Vec<Diagnostic>>>>,
}

const LSP_NAME: &str = "AutoCorrect";
const DEFAULT_CONFIG_FILE: &str = ".autocorrectrc";
const DEFAULT_IGNORE_FILE: &str = ".autocorrectignore";

const DIAGNOSTIC_SOURCE: &str = "AutoCorrect";
pub(crate) const DIAGNOSTIC_SOURCE_TYPO: &str = "Typo";

fn is_config_file(path: &PathBuf) -> bool {
    path.ends_with(DEFAULT_IGNORE_FILE)
        || path.ends_with(DEFAULT_CONFIG_FILE)
        || path.ends_with(".gitignore")
}

impl Backend {
    fn work_dir(&self) -> PathBuf {
        self.work_dir.read().unwrap().clone()
    }

    fn set_work_dir(&self, work_dir: PathBuf) {
        *self.work_dir.write().unwrap() = work_dir;
    }

    fn upsert_document(&self, doc: Arc<TextDocumentItem>) {
        self.documents
            .write()
            .unwrap()
            .insert(doc.uri.clone(), doc.clone());
    }

    #[allow(unused)]
    fn get_document(&self, uri: &Url) -> Option<Arc<TextDocumentItem>> {
        self.documents.read().unwrap().get(uri).cloned()
    }

    fn remove_document(&self, uri: &Url) {
        self.documents.write().unwrap().remove(uri);
    }

    async fn lint_document(&self, document: &TextDocumentItem) {
        Self::_lint_document(&self.client, &self.ignorer, &self.diagnostics, document).await;
    }

    async fn _lint_document(
        client: &Client,
        ignorer: &Arc<RwLock<Option<Ignorer>>>,
        diagnostics: &Arc<RwLock<HashMap<Url, Vec<Diagnostic>>>>,
        document: &TextDocumentItem,
    ) {
        Self::_clear_diagnostics(client, diagnostics.clone(), &document.uri).await;
        if let Some(ignorer) = ignorer.read().unwrap().as_ref() {
            if let Ok(filepath) = document.uri.to_file_path() {
                if ignorer.is_ignored(&filepath) {
                    return;
                }
            }
        }

        let input = document.text.as_str();
        let path = document.uri.path();
        let result = autocorrect::lint_for(input, path);

        let mut new_diagnostics: Vec<Diagnostic> = result
            .lines
            .iter()
            .map(|result| {
                let addition_lines = result.old.lines().count() - 1;
                let (severity, source) = match result.severity {
                    autocorrect::Severity::Error => (
                        Some(DiagnosticSeverity::WARNING),
                        Some(DIAGNOSTIC_SOURCE.to_string()),
                    ),
                    autocorrect::Severity::Warning => (
                        Some(DiagnosticSeverity::INFORMATION),
                        Some(DIAGNOSTIC_SOURCE_TYPO.to_string()),
                    ),
                    _ => (None, None),
                };

                Diagnostic {
                    range: Range {
                        start: Position {
                            line: result.line as u32 - 1,
                            character: result.col as u32 - 1,
                        },
                        end: Position {
                            line: (result.line + addition_lines - 1) as u32,
                            character: (result.col + result.new.chars().count() - 1) as u32,
                        },
                    },
                    source,
                    severity,
                    message: result.new.clone(),
                    ..Default::default()
                }
            })
            .collect();

        let typo_diagnostics = typocheck::check_typos(input);
        new_diagnostics.extend(typo_diagnostics);

        if let Ok(mut map) = diagnostics.write() {
            map.entry(document.uri.clone())
                .and_modify(|old_diagnostics| old_diagnostics.extend_from_slice(&new_diagnostics))
                .or_insert_with(|| new_diagnostics.clone());
        }
        client
            .publish_diagnostics(document.uri.clone(), new_diagnostics, None)
            .await;
    }

    async fn clear_diagnostics(&self, uri: &Url) {
        Self::_clear_diagnostics(&self.client, self.diagnostics.clone(), uri).await;
    }

    async fn _clear_diagnostics(
        client: &Client,
        diagnostics: Arc<RwLock<HashMap<Url, Vec<Diagnostic>>>>,
        uri: &Url,
    ) {
        diagnostics.write().unwrap().remove(uri);
        client.publish_diagnostics(uri.clone(), vec![], None).await;
    }

    async fn clear_all_diagnostic(&self) {
        let uris = self
            .documents
            .read()
            .unwrap()
            .keys()
            .cloned()
            .collect::<Vec<_>>();

        for uri in uris.iter() {
            self.clear_diagnostics(uri).await;
        }
    }

    async fn recheck_all_documents(
        client: &Client,
        ignorer: Arc<RwLock<Option<Ignorer>>>,
        diagnostics: Arc<RwLock<HashMap<Url, Vec<Diagnostic>>>>,
        documents: Arc<RwLock<HashMap<Url, Arc<TextDocumentItem>>>>,
    ) {
        let documents = documents
            .read()
            .unwrap()
            .values()
            .cloned()
            .collect::<Vec<_>>();
        for document in documents.iter() {
            Self::_lint_document(client, &ignorer, &diagnostics, document).await;
        }
    }

    async fn reload(&self) {
        Self::reload_config(
            self.work_dir(),
            &self.client,
            self.ignorer.clone(),
            self.diagnostics.clone(),
            self.documents.clone(),
        )
        .await;
    }

    async fn reload_config<P>(
        workdir: P,
        client: &Client,
        ignorer: Arc<RwLock<Option<Ignorer>>>,
        diagnostics: Arc<RwLock<HashMap<Url, Vec<Diagnostic>>>>,
        documents: Arc<RwLock<HashMap<Url, Arc<TextDocumentItem>>>>,
    ) where
        P: AsRef<Path>,
    {
        let workdir = workdir.as_ref();
        let conf_file = workdir.join(DEFAULT_CONFIG_FILE);
        autocorrect::config::load_file(&conf_file).ok();

        let new_ignorer = Ignorer::new(&workdir);
        ignorer.write().unwrap().replace(new_ignorer);

        Self::recheck_all_documents(client, ignorer, diagnostics, documents).await;
    }

    fn is_ignored(&self, uri: &Url) -> bool {
        if let Some(ignorer) = self.ignorer.read().unwrap().as_ref() {
            if let Ok(filepath) = uri.to_file_path() {
                return ignorer.is_ignored(&filepath);
            }
        }

        false
    }

    fn watch_config(&self) -> anyhow::Result<()> {
        let work_dir = self.work_dir().clone();
        let ignorer = self.ignorer.clone();
        let client = self.client.clone();
        let documents = self.documents.clone();
        let diagnostics = self.diagnostics.clone();
        let (tx, rx) = smol::channel::bounded(100);

        let mut watcher =
            notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
                if let Ok(event) = &res {
                    if !event.paths.iter().any(|p| is_config_file(p)) {
                        return;
                    }

                    match event.kind {
                        notify::EventKind::Create(_)
                        | notify::EventKind::Modify(_)
                        | notify::EventKind::Remove(_) => {
                            if let Err(err) = tx.send_blocking(res) {
                                eprintln!("Failed to send theme event: {:?}", err);
                            }
                        }
                        _ => {}
                    }
                }
            })?;

        smol::spawn(async move {
            if let Err(err) = watcher.watch(&work_dir, notify::RecursiveMode::Recursive) {
                client
                    .log_message(
                        MessageType::ERROR,
                        format!("Failed to watch root directory: {:?}", err),
                    )
                    .await;
            }

            while let Ok(Ok(event)) = rx.recv().await {
                let paths = event.paths;
                let changed_file = paths
                    .iter()
                    .flat_map(|p| if is_config_file(p) { Some(p) } else { None })
                    .next();

                if let Some(changed_file) = changed_file {
                    client
                        .log_message(
                            MessageType::INFO,
                            format!("{:?} config file changed, reload config.", changed_file),
                        )
                        .await;
                    Backend::reload_config(
                        &work_dir,
                        &client,
                        ignorer.clone(),
                        diagnostics.clone(),
                        documents.clone(),
                    )
                    .await;
                }
            }
        })
        .detach();

        Ok(())
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        if let Some(root_uri) = params.root_uri {
            let root_path = root_uri.to_file_path().unwrap();
            self.set_work_dir(root_path.clone());
            self.client
                .log_message(
                    MessageType::INFO,
                    format!("root_uri: {}\n", root_path.display()),
                )
                .await;

            let ignorer = Ignorer::new(&root_path);
            self.ignorer.write().unwrap().replace(ignorer);
            self.reload().await;

            if let Err(err) = self.watch_config() {
                self.client
                    .log_message(
                        MessageType::ERROR,
                        format!("Failed to watch root directory: {:?}", err),
                    )
                    .await;
            }
        }

        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: LSP_NAME.into(),
                version: Some(env!("CARGO_PKG_VERSION").into()),
            }),
            capabilities: ServerCapabilities {
                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(true)),
                    }),
                    file_operations: None,
                }),
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        change: Some(TextDocumentSyncKind::FULL),
                        ..Default::default()
                    },
                )),
                document_formatting_provider: Some(OneOf::Left(false)),

                code_action_provider: Some(CodeActionProviderCapability::Options(
                    CodeActionOptions {
                        code_action_kinds: Some(vec![
                            CodeActionKind::QUICKFIX,
                            CodeActionKind::SOURCE_FIX_ALL,
                        ]),
                        ..Default::default()
                    },
                )),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized!\n")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        self.client
            .log_message(MessageType::INFO, "server shutdown!\n")
            .await;
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let DidOpenTextDocumentParams { text_document } = params;
        self.clear_diagnostics(&text_document.uri).await;
        self.client
            .log_message(
                MessageType::INFO,
                format!(
                    "did_open {}, workdir: {:?}\n",
                    text_document.uri,
                    self.work_dir()
                ),
            )
            .await;

        self.upsert_document(Arc::new(text_document.clone()));
        if !self.is_ignored(&text_document.uri) {
            self.lint_document(&text_document).await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let DidCloseTextDocumentParams { text_document } = params;
        self.remove_document(&text_document.uri);
        self.clear_diagnostics(&text_document.uri).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let DidChangeTextDocumentParams {
            text_document,
            content_changes,
        } = params;
        let VersionedTextDocumentIdentifier { uri, version } = text_document;

        self.clear_diagnostics(&uri).await;
        if self.is_ignored(&uri) {
            return;
        }

        assert_eq!(content_changes.len(), 1);
        let change = content_changes.into_iter().next().unwrap();
        assert!(change.range.is_none());

        self.client
            .log_message(MessageType::INFO, format!("did_change {}", uri))
            .await;

        let updated_doc =
            TextDocumentItem::new(uri.clone(), "".to_string(), version, change.text.clone());
        self.upsert_document(Arc::new(updated_doc.clone()));

        self.lint_document(&updated_doc).await;
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let DidSaveTextDocumentParams { text_document, .. } = params;
        self.client
            .log_message(
                MessageType::INFO,
                format!("did_save {}\n", text_document.uri),
            )
            .await;

        if is_config_file(&text_document.uri.to_file_path().unwrap()) {
            self.clear_all_diagnostic().await;
            self.client
                .log_message(MessageType::INFO, "reload config\n")
                .await;
            self.reload().await;
        }
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let DocumentFormattingParams { text_document, .. } = params;

        if self.is_ignored(&text_document.uri) {
            return Ok(None);
        }

        self.client
            .log_message(
                MessageType::INFO,
                format!("formatting {}\n", text_document.uri),
            )
            .await;

        if let Some(document) = self.get_document(&text_document.uri) {
            self.clear_diagnostics(&text_document.uri).await;
            let input = document.text.as_str();

            self.client
                .log_message(MessageType::INFO, format!("before: {}", input))
                .await;
            let result = autocorrect::format_for(input, document.uri.path());
            self.client
                .log_message(MessageType::INFO, format!("after: {}", result.out))
                .await;
            let range = Range::new(
                Position::new(0, 0),
                Position {
                    line: u32::MAX,
                    character: u32::MAX,
                },
            );
            return Ok(Some(vec![TextEdit::new(range, result.out)]));
        }

        Ok(None)
    }

    async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        let CodeActionParams {
            text_document,
            context,
            ..
        } = params;

        if self.is_ignored(&text_document.uri) {
            return Ok(None);
        }

        let mut response = CodeActionResponse::new();

        // let mut all_changes = vec![];
        for diagnostic in context.diagnostics.iter() {
            let suggestions = diagnostic
                .data
                .as_ref()
                .and_then(|data| serde_json::from_value::<Vec<String>>(data.clone()).ok())
                .unwrap_or(vec![diagnostic.message.clone()]);

            // if let Some(suggestion) = suggestions.first() {
            //     all_changes.push((
            //         text_document.uri.clone(),
            //         vec![TextEdit::new(diagnostic.range, suggestion.clone())],
            //     ));
            // }

            for suggestion in suggestions.iter() {
                let title = if diagnostic.source == Some(DIAGNOSTIC_SOURCE.to_string()) {
                    Some("AutoCorrect Fix".to_string())
                } else if diagnostic.source == Some(DIAGNOSTIC_SOURCE_TYPO.to_string()) {
                    Some(format!("Suggest: {}", suggestion))
                } else {
                    None
                };
                let Some(title) = title else {
                    continue;
                };

                let action = CodeAction {
                    title,
                    kind: Some(CodeActionKind::QUICKFIX),
                    diagnostics: Some(vec![diagnostic.clone()]),
                    edit: Some(WorkspaceEdit {
                        changes: Some(
                            vec![(
                                text_document.uri.clone(),
                                vec![TextEdit {
                                    range: diagnostic.range,
                                    new_text: suggestion.clone(),
                                }],
                            )]
                            .into_iter()
                            .collect(),
                        ),
                        ..Default::default()
                    }),
                    is_preferred: Some(true),
                    ..Default::default()
                };
                response.push(CodeActionOrCommand::CodeAction(action));
            }
        }

        // if !all_changes.is_empty() {
        //     let fix_all_action = CodeAction {
        //         title: "AutoCorrect All".into(),
        //         kind: Some(CodeActionKind::SOURCE_FIX_ALL),
        //         diagnostics: None,
        //         edit: Some(WorkspaceEdit {
        //             changes: Some(all_changes.into_iter().collect()),
        //             ..Default::default()
        //         }),
        //         ..Default::default()
        //     };

        //     response.push(CodeActionOrCommand::CodeAction(fix_all_action.clone()))
        // }

        return Ok(Some(response));
    }
}

pub async fn start() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend {
        client,
        work_dir: RwLock::new(PathBuf::new()),
        documents: Arc::new(RwLock::new(HashMap::new())),
        ignorer: Arc::new(RwLock::new(None)),
        diagnostics: Arc::new(RwLock::new(HashMap::new())),
    });

    Server::new(stdin, stdout, socket).serve(service).await;
}
