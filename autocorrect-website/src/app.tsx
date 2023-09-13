/// <reference lib="dom" />
/// <reference lib="dom.iterable" />
const autocorrectLib = import('@huacnlee/autocorrect');
import * as monaco from 'monaco-editor';
import examples from './examples';

import { useEffect, useRef, useState } from 'react';
import { createRoot } from 'react-dom/client';
import './style.scss';

let autocorrect: any;

let config = {
  rules: {
    fullwidth: 2,
    'halfwidth-punctuation': 2,
    spellcheck: 2,
  },
  spellcheck: {
    words: ['WebAssembly', 'Rust', 'NPM', 'Web', 'JavaScript'],
  },
};

const createMarkers = (result: any) => {
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
        endColumn: lineResult.c + lineResult.old.length + 1,
        message: `AutoCorrect: ${lineResult.new}`,
      };
    }
  );

  return markers;
};

const editorOptions: monaco.editor.IStandaloneEditorConstructionOptions = {
  theme: 'vs',
  tabSize: 2,
  useTabStops: true,
  scrollbar: {
    verticalScrollbarSize: 5,
    horizontalScrollbarSize: 5,
    useShadows: true,
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

let editor: monaco.editor.IStandaloneCodeEditor;

export const AppEditor = () => {
  const editorRef = useRef();
  const [message, showMessage] = useState('');
  const [fileType, setFileType] = useState('markdown');

  const onLint = () => {
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

    editor.setValue(example.raw);
    // @ts-ignore
    monaco.editor.setModelLanguage(editor.getModel(), fileType);
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

    const start = new Date();
    const result = autocorrect.formatFor(editor.getValue(), fileType);
    // @ts-ignore
    const duration = new Date() - start;
    showMessage(`Speed time: ${duration}ms`);
    console.log(result);

    editor.setValue(result.out);
    return false;
  };

  const initEditor = () => {
    if (!editorRef?.current) {
      return;
    }
    if (editor) {
      return;
    }

    autocorrectLib.then((ac) => {
      const loadedConfig = ac.loadConfig(JSON.stringify(config));
      console.log('Loaded config: ', loadedConfig);
      autocorrect = ac;
      // @ts-ignore
      window.autocorrect = ac;

      reloadExample();
    });

    console.log('initEditor');
    editor = monaco.editor.create(editorRef?.current, {
      value: '',
      ...editorOptions,
    });

    editor.onDidChangeModelContent(() => {
      onLint();
    });

    editor.addCommand(monaco.KeyCode.KeyZ + monaco.KeyMod.CtrlCmd, function () {
      reloadExample();
    });
  };

  useEffect(() => {
    initEditor();
    reloadExample();
  }, [editorRef]);

  return (
    <div className="app-editor-wrap">
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
      <div className="editor-wraper" ref={editorRef as any}></div>
    </div>
  );
};

document.addEventListener('DOMContentLoaded', () => {
  const appEditor = createRoot(document.getElementById('app-editor') as any);
  appEditor.render(<AppEditor />);
});
