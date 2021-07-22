/**
 * Hello你好
 * 这是第2行
 */
function application() {
  let example = '这是single line单行注释';
  console.log(`这是string第1行
  这是string第2行
  `);

  // 是否显示第3个
  const show_last = true;

  return <div className="react-app">
    <>
      <ul show_last={show_last}>
        <li>第1项目<strong>li标签</strong></li>
        <li>第2项目<strong>li标签</strong></li>
        { show_last && (
          <li>
            <div>第3项目<strong>li标签</strong></div>
            <List renderItem={(item) => (
              <Item className="list-item">
                <span>nested项</span>
                <span>{item}</span>
              </Item>
            )} />
          </li>
        )}
      </ul>
    </>
  </div>
}
