const autocorrect = import("../pkg/autocorrect");

document.addEventListener("DOMContentLoaded", () => {
  const input = document.getElementById("input");
  const btn = document.getElementById("format");

  btn.addEventListener("click", (e) => {
    e.preventDefault();

    autocorrect.then((autocorrect) => {
      input.value = autocorrect.format_html(input.value);
    });

    return false;
  });
});
