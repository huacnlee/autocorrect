public class AutoCorrect {
    public static native String format(String input);
    
    static {
        System.loadLibrary("autocorrect_java");
    }

    public static void main(String[] args) {
        System.out.println("\nAutoCorrect Test");
        String output = AutoCorrect.format("hello你好");
        System.out.printf("format: %s\n", output);
    }
}