/// <reference lib="dom" />
/// <reference lib="dom.iterable" />
const autocorrectLib = import('@huacnlee/autocorrect');
import * as monaco from 'monaco-editor';
import examples from './examples';

import './style.scss';

let autocorrect: any;
let currentFileType = 'text';

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

document.addEventListener('DOMContentLoaded', () => {
  const input = document.querySelector('.editor-wraper') as HTMLElement;
  if (!editor) {
    editor = monaco.editor.create(input, {
      value: '',
      ...editorOptions,
    });
  }
  editor.onDidChangeModelContent(() => {
    lintText();
  });

  editor.addCommand(monaco.KeyCode.KeyZ + monaco.KeyMod.CtrlCmd, function () {
    reloadExample();
  });

  const btn = document.querySelector('#btn') as HTMLElement;
  const btnReset = document.querySelector('#btnReset') as HTMLElement;
  const message = document.querySelector('.message') as HTMLElement;
  const select = document.querySelector('#filetype') as any;

  const reloadExample = () => {
    loadExampleByFileType(currentFileType);
  };

  const loadExampleByFileType = (fileType: string) => {
    currentFileType = fileType;

    // @ts-ignore
    const example = examples[fileType];

    editor.setValue(example.raw);
    // @ts-ignore
    monaco.editor.setModelLanguage(editor.getModel(), fileType);
  };

  const options = Object.keys(examples).map((key) => {
    // @ts-ignore
    const example = examples[key];
    return "<option value='" + key + "'>" + example.title + '</option>';
  });

  autocorrectLib.then((ac) => {
    const loadedConfig = ac.loadConfig(JSON.stringify(config));
    console.log('Loaded config: ', loadedConfig);
    autocorrect = ac;
    // @ts-ignore
    window.autocorrect = ac;

    reloadExample();
  });

  select.innerHTML = options.join('');
  select.value = 'javascript';

  select.addEventListener('change', (e: any) => {
    loadExampleByFileType(e.target.value);
  });

  const formatText = (e: any) => {
    e.preventDefault();

    const start = new Date();
    const result = autocorrect.formatFor(editor.getValue(), currentFileType);
    // @ts-ignore
    const duration = new Date() - start;
    message.innerHTML = `Speed time: ${duration}ms`;
    console.log(result);

    editor.setValue(result.out);
    return false;
  };

  const lintText = () => {
    if (!autocorrect) {
      return;
    }

    const start = new Date();
    const result = autocorrect.lintFor(editor.getValue(), currentFileType);
    // @ts-ignore
    const duration = new Date() - start;
    message.innerHTML = `Speed time: ${duration}ms`;

    monaco.editor.setModelMarkers(
      // @ts-ignore
      editor.getModel(),
      'autocorrect',
      createMarkers(result)
    );

    return false;
  };

  // input.addEventListener('keyup', formatText);
  btn.addEventListener('click', formatText);
  btnReset.addEventListener('click', () => {
    reloadExample();
  });

  loadExampleByFileType('javascript');
});

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
