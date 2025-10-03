use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

mod typocheck;

struct Backend {
    client: Client,
    work_dir: RwLock<PathBuf>,
    documents: RwLock<HashMap<Url, Arc<TextDocumentItem>>>,
    ignorer: RwLock<Option<autocorrect::ignorer::Ignorer>>,
    diagnostics: RwLock<HashMap<Url, Vec<Diagnostic>>>,
}

const LSP_NAME: &str = "AutoCorrect";
const DEFAULT_CONFIG_FILE: &str = ".autocorrectrc";
const DEFAULT_IGNORE_FILE: &str = ".autocorrectignore";

const DIAGNOSTIC_SOURCE: &str = "AutoCorrect";
pub(crate) const DIAGNOSTIC_SOURCE_SPELLCHECK: &str = "Typo";

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
        self.clear_diagnostics(&document.uri).await;

        let input = document.text.as_str();
        let path = document.uri.path();
        let result = autocorrect::lint_for(input, path);

        let mut diagnostics: Vec<Diagnostic> = result
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
                        Some(DIAGNOSTIC_SOURCE_SPELLCHECK.to_string()),
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
        diagnostics.extend(typo_diagnostics);

        self.send_diagnostics(document, diagnostics).await;
    }

    async fn send_diagnostics(&self, document: &TextDocumentItem, diagnostics: Vec<Diagnostic>) {
        if let Ok(mut map) = self.diagnostics.write() {
            map.entry(document.uri.clone())
                .and_modify(|old_diagnostics| old_diagnostics.extend_from_slice(&diagnostics))
                .or_insert_with(|| diagnostics.clone());
        }
        self.client
            .publish_diagnostics(document.uri.clone(), diagnostics, None)
            .await;
    }

    async fn clear_diagnostics(&self, uri: &Url) {
        self.diagnostics.write().unwrap().remove(uri);
        self.client
            .publish_diagnostics(uri.clone(), vec![], None)
            .await;
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

    fn reload_config(&self) {
        let conf_file = self.work_dir().join(DEFAULT_CONFIG_FILE);
        autocorrect::config::load_file(&conf_file.to_string_lossy()).ok();

        let ignorer = autocorrect::ignorer::Ignorer::new(&self.work_dir().to_string_lossy());
        self.ignorer.write().unwrap().replace(ignorer);
    }

    fn is_ignored(&self, uri: &Url) -> bool {
        if let Some(ignorer) = self.ignorer.read().unwrap().as_ref() {
            if let Ok(filepath) = uri.to_file_path() {
                return ignorer.is_ignored(&filepath.to_string_lossy());
            }
        }

        false
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

            let ignorer = autocorrect::ignorer::Ignorer::new(&root_path.to_string_lossy());
            self.ignorer.write().unwrap().replace(ignorer);
        }

        self.reload_config();

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

        if self.is_ignored(&text_document.uri) {
            return;
        }

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

        self.lint_document(&text_document).await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let DidCloseTextDocumentParams { text_document } = params;

        if self.is_ignored(&text_document.uri) {
            return;
        }

        self.client
            .log_message(
                MessageType::INFO,
                format!("did_close {}\n", text_document.uri),
            )
            .await;

        self.remove_document(&text_document.uri);
        self.clear_diagnostics(&text_document.uri).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let DidChangeTextDocumentParams {
            text_document,
            content_changes,
        } = params;
        let VersionedTextDocumentIdentifier { uri, version } = text_document;

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

        if text_document.uri.path().ends_with(DEFAULT_CONFIG_FILE)
            || text_document.uri.path().ends_with(DEFAULT_IGNORE_FILE)
        {
            self.clear_all_diagnostic().await;
            self.client
                .log_message(MessageType::INFO, "reload config\n")
                .await;
            self.reload_config();
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

        let all_diagnostics = self
            .diagnostics
            .read()
            .unwrap()
            .get(&text_document.uri)
            .cloned();

        let mut show_fix_all = false;

        let all_changes = all_diagnostics.map(|diagnostics| {
            let mut map = HashMap::new();

            if !show_fix_all {
                show_fix_all = diagnostics.iter().any(|diagnostic| {
                    diagnostic.source == Some(DIAGNOSTIC_SOURCE.to_string())
                        || diagnostic.source == Some(DIAGNOSTIC_SOURCE_SPELLCHECK.to_string())
                });
            }

            map.insert(
                text_document.uri.clone(),
                diagnostics
                    .iter()
                    .map(|diagnostic| TextEdit {
                        range: diagnostic.range,
                        new_text: diagnostic.message.clone(),
                    })
                    .collect(),
            );
            map
        });

        for diagnostic in context.diagnostics.iter() {
            let suggestions = diagnostic
                .data
                .as_ref()
                .and_then(|data| serde_json::from_value::<Vec<String>>(data.clone()).ok())
                .unwrap_or(vec![diagnostic.message.clone()]);

            for suggestion in suggestions.iter() {
                let action = CodeAction {
                    title: if diagnostic.source == Some(DIAGNOSTIC_SOURCE.to_string()) {
                        "AutoCorrect Fix".to_string()
                    } else {
                        format!("Typo Fix: {}", suggestion)
                    },
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

            let all_changes = suggestions.iter().fold(
                all_changes.clone().unwrap_or_default(),
                |mut acc, suggestion| {
                    acc.entry(text_document.uri.clone())
                        .or_insert_with(Vec::new)
                        .push(TextEdit {
                            range: diagnostic.range,
                            new_text: suggestion.clone(),
                        });
                    acc
                },
            );

            let fix_all_action = CodeAction {
                title: "AutoCorrect All".into(),
                kind: Some(CodeActionKind::SOURCE_FIX_ALL),
                diagnostics: None,
                edit: Some(WorkspaceEdit {
                    changes: Some(all_changes),
                    ..Default::default()
                }),
                ..Default::default()
            };

            if show_fix_all {
                response.push(CodeActionOrCommand::CodeAction(fix_all_action.clone()))
            }
        }
        return Ok(Some(response));
    }
}

pub async fn start() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend {
        client,
        work_dir: RwLock::new(PathBuf::new()),
        documents: RwLock::new(HashMap::new()),
        ignorer: RwLock::new(None),
        diagnostics: RwLock::new(HashMap::new()),
    });

    Server::new(stdin, stdout, socket).serve(service).await;
}
