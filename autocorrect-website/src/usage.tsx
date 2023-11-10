import usage from './docs/usage.md?raw';
import { MarkdownContent } from './markdown';

export const UsagePage = () => {
  return (
    <div className="container">
      <MarkdownContent content={usage} />
    </div>
  );
};
