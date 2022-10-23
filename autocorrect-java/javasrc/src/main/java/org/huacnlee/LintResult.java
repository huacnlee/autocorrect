public class LintResult implements AutoCloseable {
    private long ptr;

    LintResult(long ptr) {
      this.ptr = ptr;
    }

    public String getFilepath() {
      return AutoCorrect.lintResultGetString(this.ptr, "filepath");
    }

    public String getRaw() {
      return AutoCorrect.lintResultGetString(this.ptr, "raw");
    }

}