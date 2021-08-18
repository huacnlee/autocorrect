const autocorrect = import('../pkg/autocorrect');

document.addEventListener('DOMContentLoaded', () => {
  const input = document.getElementById('input');
  const btn = document.querySelector('#btn');
  const preview = document.querySelector('.preview');
  const message = document.querySelector('.message');

  const formatText = (e) => {
    e.preventDefault();

    autocorrect.then((autocorrect) => {
      const start = new Date();
      const newHTML = autocorrect.format_html(input.value);
      const duration = new Date() - start;
      message.innerHTML = `Speed time: ${duration}ms`;
      preview.value = newHTML;
    });

    return false;
  };

  // input.addEventListener('keyup', formatText);
  btn.addEventListener('click', formatText);
});
