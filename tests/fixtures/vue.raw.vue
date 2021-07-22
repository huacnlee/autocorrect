<template>
  <div :name="foo">这是html里面的正文</div>
</template>
<script>
import { defineComponent } from "@vue/composition-api";

/**
 * 第1行注释
 * 第2行注释
 */
export default defineComponent({
  // 这是setup方法
  setup() {
    const title = `这是string变量`;
  },
});
</script>
<style scoped>
/* 这是inline的css代码 */
body {
  font-size: 14px;
}
</style>