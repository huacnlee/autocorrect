# AutoCorrect for Java

The Java version of [AutoCorrect](https://github.com/huacnlee/autocorrect) built for Java.

- Rust - [autocorrect](https://github.com/huacnlee/autocorrect)
- Ruby - [autocorrect-rb](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-rb)
- Go - [autocorrect-go](https://github.com/longbridgeapp/autocorrect)
- Python - [autocorrect-py](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-py)
- Node.js - [autocorrect-node](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-node)
- JavaScript (Browser) - [autocorrect-wasm](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-wasm)
- Java - [autocorrect-java](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-java)

## Installation

```bash

```

## Usage

```js
import io.github.huacnlee.AutoCorrect;

public static void main(String[] args) {
    String output = AutoCorrect.format("Hello你好");
    System.out.println(output);
    // Hello 你好

    output = AutoCorrect.formatFor("// Hello你好,这是Java注释.", "test.java");
    System.out.println(output);
    // // Hello 你好，这是 Java 注释。

    LintResult result = AutoCorrect.lintFor("// Hello你好,这是Java注释.", "test.java");
    System.out.printf("LintResult.raw: %s\n", result.getRaw());
    System.out.printf("LintResult.filepath: %s\n", result.getFilepath());

    for (LineResult line : result.getLines()) {
        System.out.printf("LineResult: (%d,%d) severity: %d\n", line.getLine(), line.getCol(), line.getSeverity());
        System.out.printf("LineResult old -> new:\n%s\n%s\n", line.getOld(), line.getNew());
    }
}
```
