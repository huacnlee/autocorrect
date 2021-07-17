package main

import (
	"fmt"
)

// 这种 *.corrected.* 文件是正确的
// 用于验证 autocorrect --lint 直接输出的时候，stdout 是否正常
func hello(name string) {
	a := "第 1 个"
	b := `多行 string
第 2 行
`
	fmt.Println("Hello 你好：" + name)
	return
}
