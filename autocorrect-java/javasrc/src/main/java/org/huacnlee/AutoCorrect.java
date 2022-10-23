public class AutoCorrect {
    public static native String format(String input);
    public static native String formatFor(String input, String filename);
    
    static {
        System.loadLibrary("autocorrect_java");
    }

    public static void main(String[] args) {
        System.out.println("\nAutoCorrect Test");
        String output = AutoCorrect.format("Hello你好");
        System.out.printf("format: %s\n", output);

        output = AutoCorrect.formatFor("// Hello你好,这是Java注释.", "test.java");
        System.out.printf("formatFor: %s\n", output);
    }
}