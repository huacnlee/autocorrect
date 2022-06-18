import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';
import type { Event } from '@tauri-apps/api/event';
import { open as openDialog } from '@tauri-apps/api/dialog';

import { writeText } from '@tauri-apps/api/clipboard';
import './App.scss';
import { Button, Input, message, Select } from 'antd';
import { GitHubIcon, OpenIcon } from './icon';
import { fs } from '@tauri-apps/api';
import { demoText, fileTypes } from './config';

const fileBasename = (filename: string): string => {
  const parts = filename.split('/');
  return parts[parts.length - 1];
};

function App() {
  const [currentFileName, setCurrentFileName] = useState<string | null>(null);
  const [fileType, setFileType] = useState(fileTypes[0].value);
  const [source, setSource] = useState(demoText);
  const [output, setOutput] = useState('');

  useEffect(() => {
    // Listen file drop to open
    listen('tauri://file-drop', (event: Event<string[]>) => {
      const { payload } = event;
      const filename = payload.pop();
      openFile(filename);
    });
  });

  /**
   * OpenFile to format
   * @param fname
   * @returns
   */
  const openFile = (fname?: string) => {
    // Ignore no filename
    if (!fname) {
      return;
    }

    const extname = fname.split('.').pop() || 'txt';
    setFileType(extname);

    message.loading('文件读取中...');
    fs.readTextFile(fname)
      .then((text) => {
        message.destroy();
        message.success('文件读取完成');

        setCurrentFileName(fname);
        setSource(text);
        doFormat(text);
      })
      .catch((err) => {
        message.destroy();
        message.error(err);
      });
  };

  // show OpenFile Disalog and then openfile
  const onOpenFileClick = (e: any) => {
    e.preventDefault();
    openDialog({
      filters: [
        {
          name: 'File',
          extensions: fileTypes.map((type) => type.value),
        },
      ],
    }).then((fname) => {
      if (fname) {
        openFile(fname as any);
      }
    });
  };

  const onSaveClick = (e: any) => {
    e.preventDefault();

    if (!currentFileName) {
      return;
    }

    fs.writeTextFile(currentFileName, output)
      .then((_) => {
        message.info('Saved successfully');
      })
      .catch((err) => {
        message.error(err);
      });
  };

  const closeFile = (e: any) => {
    e.preventDefault();
    setCurrentFileName(null);
    doClear();

    return false;
  };

  // watch Source text change
  const onSourceChange = (e: any) => {
    const { value } = e.target;
    setSource(value);
    doFormat(value);
  };

  // run Format
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
    writeText(output).then(() => {
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
          <Button onClick={onOpenFileClick}>
            <OpenIcon />
          </Button>
          <Button onClick={onSaveClick} disabled={!currentFileName}>
            Save
          </Button>
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
            rel="noreferrer"
          >
            <GitHubIcon />
          </a>

          <Button onClick={doCopy} type="primary">
            Copy
          </Button>
        </div>
      </div>
      <div className="main-container">
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
      <div className="status-bar">
        {currentFileName && (
          <span className="space-x-2 current-filename">
            <span>{fileBasename(currentFileName)}</span>
            <a onClick={closeFile} className="cursor-pointer">
              &times;
            </a>
          </span>
        )}
      </div>
    </div>
  );
}

export default App;
