const autocorrectLib = import('../pkg/autocorrect');
import examples from './examples';

let autocorrect;
let currentFileType = 'text';

document.addEventListener('DOMContentLoaded', () => {
  autocorrectLib.then((ac) => {
    autocorrect = ac;
    window.autocorrect = ac;

    console.log('------------', autocorrect.test_random());
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

  select.addEventListener('change', (e) => {
    currentFileType = e.target.value;
    const example = examples[currentFileType];
    input.value = example.raw;
    filename.innerHTML = `FileType: ${currentFileType}`;
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
