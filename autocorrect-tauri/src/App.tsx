import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import './App.css';

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
    name: 'HTML',
    value: 'html',
  },
  {
    name: 'CSS',
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
];

function App() {
  const [message, setMessage] = useState({ title: '', type: 'info' });
  const [fileType, setFileType] = useState(fileTypes[0].value);
  const [source, setSource] = useState('');
  const [output, setOutput] = useState('');

  const showMessage = (msg: string, type?: string) => {
    setMessage({ title: msg, type: type || 'info' });

    setTimeout(() => {
      setMessage({ title: '', type: 'info' });
    }, 5000);
  };

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
        showMessage('执行成功');
      })
      .catch((msg) => {
        showMessage(msg, 'error');
      });
  };

  const onFileTypeChange = (e: any) => {
    const { value } = e.target;
    setFileType(value);
    doFormat(source);
  };

  const doClear = () => {
    setOutput('');
    setSource('');
  };

  const doCopy = () => {
    navigator.clipboard.writeText(output).then(() => {
      showMessage('复制成功');
    });
  };

  return (
    <div className="App text-left space-y-6">
      <div className="toolbar flex justify-between items-center">
        <div className="flex items-center space-x-4">
          <select
            className="dropdown"
            value={fileType}
            onChange={onFileTypeChange}
          >
            {fileTypes.map((item) => (
              <option key={item.value} value={item.value}>
                {item.name}
              </option>
            ))}
          </select>
          <button onClick={doClear} className="btn">
            Clear
          </button>
        </div>
        <div>
          <button onClick={doCopy} className="btn">
            Copy
          </button>
        </div>
      </div>
      <div className="flex absolute left-4 right-4 top-16 bottom-12 space-x-6 items-center">
        <textarea
          className="textarea block h-full w-full"
          value={source}
          placeholder="Input source text here..."
          onChange={onSourceChange}
        ></textarea>

        <textarea
          value={output}
          className="textarea block h-full w-full"
          placeholder="Formatted text will appear here..."
          readOnly
        ></textarea>
      </div>
      <div className="flex h-8 absolute bottom-0 left-0 right-0 px-4">
        <span
          className={`text-sm ${
            message.type === 'error' ? 'text-red-600' : 'text-green-600'
          }`}
        >
          {message.title}
        </span>
      </div>
    </div>
  );
}

export default App;
