use super::*;
use html5ever::serialize;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::{parse_document, ParseOpts};
use markup5ever_rcdom::{Handle, NodeData, RcDom, SerializableHandle};

/// Format a html content.
///
/// Example:
///
/// ```
/// let html = r#"
/// <article>
///   <h1>这是Heading标题</h1>
///   <div class="content">
///     <p>你好Rust世界<strong>Bold文本</strong></p>
///     <p>这是第二行p标签</p>
///   </div>
/// </article>
/// "#;
///
/// autocorrect.format_html(html)
/// ```
pub fn format_html(html_str: &str) -> String {
  let opts = ParseOpts {
    tree_builder: TreeBuilderOpts {
      drop_doctype: true,
      ..Default::default()
    },
    ..Default::default()
  };

  let mut dom = parse_document(RcDom::default(), opts)
    .from_utf8()
    .read_from(&mut html_str.as_bytes())
    .unwrap();

  traverse_nodes(&mut dom.document);

  let mut bytes = vec![];
  let document: SerializableHandle = dom.document.clone().into();
  serialize(&mut bytes, &document, Default::default()).unwrap();
  let result = String::from_utf8(bytes).unwrap();
  return String::from(result.as_str());
}

// traverse nodes to format
fn traverse_nodes(handle: &Handle) {
  let node = handle;
  match node.data {
    NodeData::Text { ref contents } => {
      let mut text = contents.borrow_mut();
      let new_text = format(&text);
      if text.len() == 0 {
        return;
      }

      text.clear();
      text.push_slice(&new_text);
      // println!("{}", text)
    }
    _ => {}
  }

  for child in node.children.borrow_mut().iter() {
    traverse_nodes(&child);
  }
}

#[test]
fn test_format_html() {
  let html = r#"
  <article>
    <h1>这是Heading标题</h1>
    <div class="content">
      <p>你好Rust世界<strong>Bold文本</strong></p>
      <p>这是第二行p标签</p>
    </div>
  </article>
  "#;

  let expected = r#"
  <article>
    <h1>这是 Heading 标题</h1>
    <div class="content">
      <p>你好 Rust 世界<strong>Bold 文本</strong></p>
      <p>这是第二行 p 标签</p>
    </div>
  </article>
  "#;

  assert_html_eq!(expected, format_html(html))
}
