import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import './App.css';
import { Button, Input, message, Select } from 'antd';
import { GitHubIcon } from './icon';

// autocorrect: false
const demoText = `基于Rust编写的工具,用于「自动纠正」或「检查并建议」文案，给CJK（中文、日语、韩语）与英文混写的场景,补充正确的空格,同时尝试以安全的方式自动纠正标点符号等等.

支持各种类型源代码文件,能自动识别文件名,并准确找到字符串、注释做自动纠正.`;
// autocorrect: true

const fileTypes = [
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

function App() {
  const [fileType, setFileType] = useState(fileTypes[0].value);
  const [source, setSource] = useState(demoText);
  const [output, setOutput] = useState('');

  const onSourceChange = (e: any) => {
    const { value } = e.target;
    setSource(value);
    doFormat(value);
  };

  const doFormat = (value: string) => {
    invoke('format_for', {
      text: value,
      filename: fileType,
    })
      .then((out: any) => {
        setOutput(out);
        message.destroy();
      })
      .catch((msg) => {
        message.error(msg);
      });
  };

  const onFileTypeChange = (value: string) => {
    setFileType(value);
    doFormat(source);
  };

  const doClear = () => {
    setOutput('');
    setSource('');
  };

  const doCopy = () => {
    navigator.clipboard.writeText(output).then(() => {
      message.info('复制成功');
    });
  };

  useEffect(() => {
    doFormat(source);
  });

  return (
    <div className="space-y-6 text-left App">
      <div className="flex items-center justify-between toolbar">
        <div className="flex items-center space-x-4">
          <Select
            showSearch
            className="w-52"
            defaultValue={fileType}
            onChange={onFileTypeChange}
          >
            {fileTypes.map((item) => (
              <Select.Option key={item.value} value={item.value}>
                {item.name}
              </Select.Option>
            ))}
          </Select>
          <Button onClick={doClear}>Clear</Button>
        </div>
        <div className="flex items-center space-x-6">
          <a
            href="https://github.com/huacnlee/autocorrect"
            target="_blank"
            className="text-gray-700"
            rel="noreferrer"
          >
            <GitHubIcon />
          </a>

          <Button onClick={doCopy} type="primary">
            Copy
          </Button>
        </div>
      </div>
      <div className="absolute flex space-x-6 left-4 right-4 top-14 bottom-4">
        <Input.TextArea
          className="block w-full h-full"
          value={source}
          placeholder="Input source text here..."
          onChange={onSourceChange}
        />

        <Input.TextArea
          value={output}
          className="block w-full h-full outline-none bg-gray-50"
          placeholder="Formatted text will appear here..."
          readOnly
        />
      </div>
    </div>
  );
}

export default App;
