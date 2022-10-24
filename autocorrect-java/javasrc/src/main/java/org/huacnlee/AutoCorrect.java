// autocorrect-disable
package org.huacnlee;

public class AutoCorrect {
    /**
     * AutoCorrect format a plain text
     *
     * @param text plain text
     * @return Formatted text
     */
    public static native String format(String text);

    /**
     * AutoCorrect format text by type
     *
     * @param text     Raw text
     * @param filename Filename or filetype
     * @return Formatted text
     */
    public static native String formatFor(String text, String filename);

    /**
     * AutoCorrect lint text by type
     *
     * @param text     Raw text
     * @param filename Filename or filetype
     * @return LintResult
     */
    static native long nativeLintFor(String text, String filename);

    /**
     * Get string from Native LineResult by ptr and field
     *
     * @param ptr
     * @param field
     * @return
     */
    static native String nativeLineResultString(long ptr, String field);

    /**
     * Get long from Native LineResult by ptr and field
     *
     * @param ptr
     * @param field fieldName
     * @return
     */
    static native long nativeLineResultLong(long ptr, String field);

    /**
     * Get long[] (ptrs) from Native LineResult#lines
     *
     * @param ptr
     * @return
     */
    static native long[] nativeLintResultLines(long ptr);

    /**
     * Get string from Native LintResult by ptr and field
     *
     * @param ptr
     * @param field
     * @return
     */
    static native String nativeLintResultString(long ptr, String field);

    public static LintResult lintFor(String text, String filepath) {
        long ptr = AutoCorrect.nativeLintFor(text, filepath);
        return new LintResult(ptr);
    }

    static {
        System.loadLibrary("autocorrect_java");
    }

    public static void main(String[] args) {
        System.out.println("\nAutoCorrect Test");
        String output = AutoCorrect.format("Hello你好");
        System.out.printf("format: %s\n", output);

        output = AutoCorrect.formatFor("// Hello你好,这是Java注释.", "test.java");
        System.out.printf("formatFor: %s\n", output);

        LintResult result = AutoCorrect.lintFor("// Hello你好,这是Java注释.", "test.java");
        System.out.printf("LintResult.raw: %s\n", result.getRaw());
        System.out.printf("LintResult.filepath: %s\n", result.getFilepath());

        for (LineResult line : result.getLines()) {
            System.out.printf("LineResult: (%d,%d) severity: %d\n", line.getLine(), line.getCol(), line.getSeverity());
            System.out.printf("LineResult old -> new:\n%s\n%s\n", line.getOld(), line.getNew());
        }
    }
}