const autocorrectLib = import('../pkg/autocorrect');
import * as monaco from 'monaco-editor';
import examples from './examples';

let autocorrect: any;
let currentFileType = 'text';

let config = `{
  "spellcheck": {
    "mode": 1,
    "words": [
      "WebAssembly",
      "Rust",
      "NPM",
      "Web"
    ]
  }
}`;

const editorOptions: any = {
  lineNumbers: 'on',
  scrollbar: {
    verticalScrollbarSize: 5,
    horizontalScrollbarSize: 5,
  },
  minimap: {
    enabled: false,
  },
};

autocorrectLib.then((ac) => {
  const loadedConfig = ac.loadConfig(config);
  console.log('Loaded config: ', loadedConfig);
  autocorrect = ac;
  // @ts-ignore
  window.autocorrect = ac;
});

document.addEventListener('DOMContentLoaded', () => {
  const input = document.getElementById('input') as HTMLElement;
  const preview = document.querySelector('.preview') as HTMLElement;

  const btn = document.querySelector('#btn') as HTMLElement;
  const btnLint = document.querySelector('#btn-lint') as HTMLElement;
  const message = document.querySelector('.message') as HTMLElement;
  const select = document.querySelector('#filetype') as any;
  const output = document.querySelector('#output') as HTMLElement;
  const filename = document.querySelector('#filename') as HTMLElement;

  const selectFileType = (fileType) => {
    currentFileType = fileType;

    const example = examples[fileType];
    editor.setValue(example.raw);
    filename.innerHTML = `FileType: ${fileType}`;
  };

  const editor = monaco.editor.create(input, {
    value: '',
    ...editorOptions,
  });

  const previewEditor = monaco.editor.create(preview, {
    value: '',
    ...editorOptions,
  });

  const options = Object.keys(examples).map((key) => {
    return "<option value='" + key + "'>" + examples[key].title + '</option>';
  });

  select.innerHTML = options.join('');
  select.value = 'html';
  selectFileType('html');

  select.addEventListener('change', (e) => {
    selectFileType(e.target.value);
  });

  const formatText = (e) => {
    e.preventDefault();

    const start = new Date();
    const result = autocorrect.formatFor(editor.getValue(), currentFileType);
    // @ts-ignore
    const duration = new Date() - start;
    message.innerHTML = `Speed time: ${duration}ms`;
    console.log(result);
    previewEditor.setValue(result.out);

    return false;
  };

  const lintText = (e) => {
    e.preventDefault();

    const start = new Date();
    const result = autocorrect.lintFor(editor.getValue(), currentFileType);
    // @ts-ignore
    const duration = new Date() - start;
    message.innerHTML = `Speed time: ${duration}ms`;
    previewEditor.setValue(JSON.stringify(result, null, 2));

    return false;
  };

  // input.addEventListener('keyup', formatText);
  btn.addEventListener('click', formatText);
  btnLint.addEventListener('click', lintText);
});
