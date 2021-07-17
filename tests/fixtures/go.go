package main

import (
	"fmt"
)

// WithContext创建基于ctx的db
// 第2行注释
func hello(name string) {
	a := "第1个"
	b := `
多行string
第2行
`
	fmt.Println("Hello你好：" + name)
	return
}
