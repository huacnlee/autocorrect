package org.huacnlee;

public class LintResult  {
    private long ptr;
    private LineResult[] lines;

    public LintResult(long ptr) {
        this.ptr = ptr;

        // Initialize lines
        long[] ptrs = AutoCorrect.nativeLintResultLines(this.ptr);
        this.lines = new LineResult[ptrs.length];

        for (int i = 0; i < ptrs.length; i++) {
            this.lines[i] = new LineResult(ptrs[i]);
        }
    }

    public String getFilepath() {
      return AutoCorrect.nativeLintResultString(this.ptr, "filepath");
    }

    public String getRaw() {
      return AutoCorrect.nativeLintResultString(this.ptr, "raw");
    }

    public LineResult[] getLines() {
        return this.lines;
    }
}