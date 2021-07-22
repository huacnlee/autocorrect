/**
 * Hello 你好
 * 这是第 2 行
 */
function application() {
  let example = '这是 single line 单行注释';
  console.log(`这是 string 第 1 行
  这是 string 第 2 行
  `);

  // 是否显示第 3 个
  const show_last = true;

  return <div className="react-app">
    <ul>
      <li>第 1 项目<strong>li 标签</strong></li>
      <li>第 2 项目<strong>li 标签</strong></li>
      { show_last && (
        <li>第 3 项目<strong>li 标签</strong></li>
      )}
    </ul>
  </div>
}

