const autocorrectLib = import('../pkg/autocorrect');
import examples from './examples';

let autocorrect;
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

const selectFileType = (fileType) => {
  const example = examples[fileType];
  input.value = example.raw;
  filename.innerHTML = `FileType: ${fileType}`;
};

document.addEventListener('DOMContentLoaded', () => {
  autocorrectLib.then((ac) => {
    const loadedConfig = ac.loadConfig(config);
    console.log('Loaded config: ', loadedConfig);
    autocorrect = ac;
    window.autocorrect = ac;
  });

  const input = document.getElementById('input');
  const btn = document.querySelector('#btn');
  const btnLint = document.querySelector('#btn-lint');
  const preview = document.querySelector('.preview');
  const message = document.querySelector('.message');
  const select = document.querySelector('#filetype');
  const output = document.querySelector('#output');
  const filename = document.querySelector('#filename');

  const options = [];
  Object.keys(examples).forEach((key) => {
    options.push(
      "<option value='" + key + "'>" + examples[key].title + '</option>'
    );
  });
  select.innerHTML = options.join('');
  select.value = 'html';
  selectFileType('html');

  select.addEventListener('change', (e) => {
    currentFileType = e.target.value;
    selectFileType(currentFileType);
  });

  const formatText = (e) => {
    e.preventDefault();

    const start = new Date();
    const result = autocorrect.formatFor(input.value, currentFileType);
    const duration = new Date() - start;
    message.innerHTML = `Speed time: ${duration}ms`;
    console.log(result);
    preview.value = result.out;

    return false;
  };

  const lintText = (e) => {
    e.preventDefault();

    const start = new Date();
    const result = autocorrect.lintFor(input.value, currentFileType);
    const duration = new Date() - start;
    message.innerHTML = `Speed time: ${duration}ms`;
    preview.value = JSON.stringify(result, null, 2);

    return false;
  };

  // input.addEventListener('keyup', formatText);
  btn.addEventListener('click', formatText);
  btnLint.addEventListener('click', lintText);
});
