// autocorrect: false
export const demoText = `基于Rust编写的工具,用于「自动纠正」或「检查并建议」文案，给CJK（中文、日语、韩语）与英文混写的场景,补充正确的空格,同时尝试以安全的方式自动纠正标点符号等等.

支持各种类型源代码文件,能自动识别文件名,并准确找到字符串、注释做自动纠正.`;
// autocorrect: true

/**
 * FileTypes supported
 */
export const fileTypes = [
  {
    name: 'Plain Text',
    value: 'txt',
  },
  {
    name: 'Markdown',
    value: 'md',
  },
  {
    name: 'HTML / Vue',
    value: 'html',
  },
  {
    name: 'CSS / SCSS / LESS',
    value: 'css',
  },
  {
    name: 'JavaScript',
    value: 'js',
  },
  {
    name: 'TypeScript',
    value: 'ts',
  },
  {
    name: 'JSON',
    value: 'json',
  },
  {
    name: 'YAML',
    value: 'yaml',
  },
  {
    name: 'XML',
    value: 'xml',
  },
  {
    name: 'Go',
    value: 'go',
  },
  {
    name: 'Rust',
    value: 'rs',
  },
  {
    name: 'Python',
    value: 'py',
  },
  {
    name: 'Ruby',
    value: 'rb',
  },
  {
    name: 'Java',
    value: 'java',
  },
  {
    name: 'PHP',
    value: 'php',
  },
  {
    name: 'C#',
    value: 'cs',
  },
  {
    name: 'Objective-C',
    value: 'objective_c',
  },
  {
    name: 'Strings',
    value: 'strings',
  },
  {
    name: 'Swift',
    value: 'swift',
  },
  {
    name: 'Kotlin',
    value: 'kt',
  },
  {
    name: 'Dart',
    value: 'dart',
  },
  {
    name: 'Scala',
    value: 'scala',
  },
  {
    name: 'LaTex',
    value: 'tex',
  },
  {
    name: 'Gettext',
    value: 'po',
  },
];
