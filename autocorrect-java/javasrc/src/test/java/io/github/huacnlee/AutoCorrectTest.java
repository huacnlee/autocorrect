// autocorrect-disable
package io.github.huacnlee;

import static org.junit.Assert.assertEquals;

import org.junit.Test;

/**
 * Unit test for simple App.
 */
public class AutoCorrectTest {
    @Test
    public void shouldWork() {
        assertEquals(AutoCorrect.format("Hello你好."), "Hello 你好。");
    }

    @Test
    public void shouldFormatFor() {
        assertEquals(AutoCorrect.formatFor("// Hello你好,这是Java注释.", "test.java"), "// Hello 你好，这是 Java 注释。");
    }

    @Test
    public void shouldLintFor() {
        String text = "// Hello你好,这是Java注释.\nString a = \"这是String字符串\"";
        LintResult lintResult = AutoCorrect.lintFor(text, "test.java");

        assertEquals(text, lintResult.getRaw());
        assertEquals("test.java", lintResult.getFilepath());

        LineResult[] lines = lintResult.getLines();
        assertEquals(2, lines.length);
        assertEquals("// Hello你好,这是Java注释.", lines[0].getOld());
        assertEquals("// Hello 你好，这是 Java 注释。", lines[0].getNew());
        assertEquals(1, lines[0].getLine());
        assertEquals(1, lines[0].getCol());
        assertEquals(1, lines[0].getSeverity());

        assertEquals("\"这是String字符串\"", lines[1].getOld());
        assertEquals("\"这是 String 字符串\"", lines[1].getNew());
        assertEquals(2, lines[1].getLine());
        assertEquals(12, lines[1].getCol());
    }
}
