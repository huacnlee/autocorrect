<template>
  <div :name="foo">这是 html 里面的正文</div>
</template>
<script>
import { defineComponent } from "@vue/composition-api";

/**
 * 第 1 行注释
 * 第 2 行注释
 */
export default defineComponent({
  // 这是 setup 方法
  setup() {
    const title = `这是 string 变量`;
  },
});
</script>
<style scoped>
/* 这是 inline 的 css 代码 */
body {
  font-size: 14px;
}
</style>
