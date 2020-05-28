use super::*;
use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use markup5ever_rcdom::{Handle, NodeData, RcDom};

pub fn format_html(raw_html: &str) -> String {
  let mut dom = parse_document(RcDom::default(), Default::default())
    .from_utf8()
    .read_from(&mut raw_html.as_bytes())
    .unwrap();

  traverse_nodes(&mut dom.document);

  String::from("")
}

// traverse nodes to format
fn traverse_nodes(handle: &Handle) {
  let node = handle;
  match node.data {
    NodeData::Text { ref contents } => {
      let text = format(&contents.borrow());
      println!("{}", text)
    }
    _ => {}
  }

  for mut child in node.children.borrow().iter() {
    traverse_nodes(&child);
  }
}

#[test]
fn test_format_html() {
  let html = r#"
  <article>
    <h1>这是Heading标题</h1>
    <div class="content">
      <p>你好Rust世界</p>
      <p>这是第二行p标签</p>
    </div>
  </article>
  "#;

  format_html(html);
}
