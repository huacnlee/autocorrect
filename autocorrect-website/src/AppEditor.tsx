import Editor from '@monaco-editor/react';
import * as monaco from 'monaco-editor';
import { useEffect, useState } from 'react';
import examples from './examples';
import './style.scss';
export const autocorrectLib = import('@huacnlee/autocorrect');

export let config = {
  rules: {
    spellcheck: 2,
  },
  spellcheck: {
    words: ['WebAssembly', 'Rust', 'NPM', 'Web', 'JavaScript'],
  },
};

export const createMarkers = (result: any) => {
  const markers: monaco.editor.IMarkerData[] = result.lines.map(
    (lineResult: any) => {
      return {
        severity:
          lineResult.severity === 1
            ? monaco.MarkerSeverity.Warning
            : monaco.MarkerSeverity.Info,
        startLineNumber: lineResult.l,
        startColumn: lineResult.c,
        endLineNumber: lineResult.l,
        endColumn: lineResult.c + lineResult.old.length,
        message: `AutoCorrect: ${lineResult.new}`,
      };
    }
  );

  return markers;
};

autocorrectLib.then((ac) => {
  const loadedConfig = ac.loadConfig(JSON.stringify(config));
  // @ts-ignore
  window.autocorrect = ac;
});

const editorOptions: monaco.editor.IStandaloneEditorConstructionOptions = {
  theme: window.matchMedia('(prefers-color-scheme: dark)').matches
    ? 'vs-dark'
    : 'vs',
  tabSize: 2,
  useTabStops: false,
  scrollbar: {
    verticalScrollbarSize: 5,
    horizontalScrollbarSize: 5,
    useShadows: false,
  },
  renderLineHighlight: 'none',
  minimap: {
    enabled: false,
  },
  formatOnPaste: true,
  unicodeHighlight: {
    ambiguousCharacters: false,
  },
};

export const AppEditor = () => {
  const [monaco, setMonaco] = useState<any>();
  const [editor, setEditor] = useState<monaco.editor.IStandaloneCodeEditor>();
  const [message, showMessage] = useState('');
  const [fileType, setFileType] = useState('markdown');

  // @ts-ignore
  const autocorrect = window.autocorrect;

  useEffect(() => {
    if (monaco) {
      // Get media query is dark mode
      const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
      // Watch dark mode change
      mediaQuery.addEventListener('change', (e) => {
        const newTheme = e.matches ? 'vs-dark' : 'vs';
        monaco?.editor?.setTheme(newTheme);
      });

      monaco.editor.defineTheme('vs-dark', {
        base: 'vs-dark',
        inherit: true,
        rules: [],
        colors: {
          'editor.background': '#030712',
        },
      });
    }
  }, [monaco]);

  const onLint = () => {
    if (!editor) {
      return;
    }
    if (!autocorrect) {
      return;
    }

    const start = new Date();
    const result = autocorrect.lintFor(editor.getValue(), fileType);
    // @ts-ignore
    const duration = new Date() - start;
    showMessage(`Speed time: ${duration}ms`);

    monaco.editor.setModelMarkers(
      // @ts-ignore
      editor.getModel(),
      'autocorrect',
      createMarkers(result)
    );

    return false;
  };

  const reloadExample = () => {
    loadExampleByFileType(fileType);
  };

  const loadExampleByFileType = (fileType: string) => {
    if (!editor) {
      return;
    }

    // @ts-ignore
    const example = examples[fileType];
    // @ts-ignore
    monaco.editor.setModelLanguage(editor.getModel(), fileType);
    editor.setValue(example.raw);
  };

  const FileTypeOptions = () => {
    return (
      <>
        {Object.keys(examples).map((key) => {
          // @ts-ignore
          const item = examples[key];
          return (
            <option key={key} value={key}>
              {item.title}
            </option>
          );
        })}
      </>
    );
  };

  const onChangeFileType = (e: any) => {
    const fileType = e.target?.value;
    setFileType(fileType);
    loadExampleByFileType(fileType);
  };

  const formatText = (e: any) => {
    e.preventDefault();
    if (!editor) {
      return;
    }

    const start = new Date();
    const result = autocorrect.formatFor(editor.getValue(), fileType);
    // @ts-ignore
    const duration = new Date() - start;
    showMessage(`Speed time: ${duration}ms`);
    console.log(result);

    editor.setValue(result.out);
    return false;
  };

  const onEditorMounted = (
    editor: monaco.editor.IStandaloneCodeEditor,
    monaco: any
  ) => {
    setEditor(editor);
    setMonaco(monaco);
  };

  useEffect(() => {
    editor?.onDidChangeModelContent(() => {
      onLint();
    });

    reloadExample();
  }, [editor]);

  return (
    <div className="p-4 text-left app-editor-wrap">
      <div className="flex items-center justify-between mx-auto mb-4">
        <select
          onChange={onChangeFileType}
          value={fileType}
          className="min-w-[250px]"
        >
          <FileTypeOptions />
        </select>

        <div className="flex flex-wrap items-center justify-between space-y-6 lg:flex-nowrap lg:space-x-6 lg:space-y-0">
          <span className="message">{message}</span>
          <div className="flex gap-3">
            <button onClick={reloadExample}>Reset</button>
            <button className="btn-primary" onClick={formatText}>
              Format
            </button>
          </div>
        </div>
      </div>
      <div className="editor-wraper absolute bottom-4 left-4 right-4 top-[110px]">
        <Editor
          defaultLanguage="markdown"
          theme={editorOptions.theme}
          options={editorOptions}
          onMount={onEditorMounted}
        />
      </div>
    </div>
  );
};
