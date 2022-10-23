public class AutoCorrect {
    public static native String format(String input);
    public static native String formatFor(String input, String filename);
    public static native long lintFor(String input, String filename);
    public static native String lintResultGetString(long ptr, String field);
    
    static {
        System.loadLibrary("autocorrect_java");
    }

    public static void main(String[] args) {
        System.out.println("\nAutoCorrect Test");
        String output = AutoCorrect.format("Hello你好");
        System.out.printf("format: %s\n", output);

        output = AutoCorrect.formatFor("// Hello你好,这是Java注释.", "test.java");
        System.out.printf("formatFor: %s\n", output);

        long result_ptr = AutoCorrect.lintFor("// Hello你好,这是Java注释.", "test.java");
        LintResult result = new LintResult(result_ptr);
        System.out.printf("lintFor: %d\n", result_ptr);
        System.out.printf("lintFor: %d\n", result.getFilepath());
    }
}