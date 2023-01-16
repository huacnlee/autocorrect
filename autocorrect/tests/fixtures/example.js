// 第1行注释
// 第2行注释
function helloWorld(a) {
  const a = '第1个';
  const b = "第2个" + "第3个";
  const re = /包含#regexp测试/;
  const re1 = new RegExp("RegExp不处理");
  const re2 = new RegExp('不处理RegExp');
  const str_literal = `这个${foo}不会处理`;
  
  /**
   * Hello你好
   * 这是第2行
   */
  const c = `这是string第1行
  这是string第2行`;
  
  // autocorrect-disable
  const disable_1 = "这行将会disable掉";
  const disable_2 = "这行将也会disable掉";
  
  return <>
  <div className="react-name">
      <List renderItem={(item) => (
          <Item className="list-item">
          <span>nested项</span>
          <span>{item}</span>
          </Item>
      )} />
      <h1>Hello你好<strong>你好foo世界</strong></h1>
      外部HTML结果
      <div>{ a && t("这里string也要处理")}</div>
  </div>
  </>
  }