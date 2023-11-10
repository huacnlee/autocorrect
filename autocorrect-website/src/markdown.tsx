import hljs from 'highlight.js';
import { Marked } from 'marked';
import { gfmHeadingId } from 'marked-gfm-heading-id';
import { markedHighlight } from 'marked-highlight';
import { TableOfContents } from './toc';

import bash from 'highlight.js/lib/languages/bash';
import diff from 'highlight.js/lib/languages/diff';
import java from 'highlight.js/lib/languages/java';
import javascript from 'highlight.js/lib/languages/javascript';
import python from 'highlight.js/lib/languages/python';
import ruby from 'highlight.js/lib/languages/ruby';
import rust from 'highlight.js/lib/languages/rust';
import shell from 'highlight.js/lib/languages/shell';

import 'github-markdown-css/github-markdown.css';
import './markdown.scss';

hljs.registerLanguage('javascript', javascript);
hljs.registerLanguage('bash', bash);
hljs.registerLanguage('shell', shell);
hljs.registerLanguage('rust', rust);
hljs.registerLanguage('ruby', ruby);
hljs.registerLanguage('python', python);
hljs.registerLanguage('java', java);
hljs.registerLanguage('diff', diff);

const marked = new Marked(
  markedHighlight({
    langPrefix: 'hljs language-',
    highlight(code, lang) {
      const language = hljs.getLanguage(lang) ? lang : 'plaintext';
      return hljs.highlight(code, { language }).value;
    },
  })
);

marked.use(gfmHeadingId());

/**
 * https://marked.js.org/using_pro#use
 * @param source Markdown source
 * @returns HTML string
 */
const markdown = (source: string) => {
  return marked.parse(source, {
    gfm: true,
  });
};

export const MarkdownContent = ({ content }: { content: string }) => {
  const html = markdown(content);
  return (
    <div className="relative block md:flex">
      <article
        className="markdown-body"
        dangerouslySetInnerHTML={{ __html: html }}
      ></article>
      <TableOfContents className="sticky" />
    </div>
  );
};
