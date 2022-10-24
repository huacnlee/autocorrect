package io.github.huacnlee;

public class LineResult {
    private long ptr;

    public LineResult(long ptr) {
        this.ptr = ptr;
    }

    public long getLine() {
        return AutoCorrect.nativeLineResultLong(this.ptr, "line");
    }

    public long getCol() {
        return AutoCorrect.nativeLineResultLong(this.ptr, "col");
    }

    public long getSeverity() {
        return AutoCorrect.nativeLineResultLong(this.ptr, "severity");
    }

    public String getNew() {
        return AutoCorrect.nativeLineResultString(this.ptr, "new");
    }

    public String getOld() {
        return AutoCorrect.nativeLineResultString(this.ptr, "old");
    }
}
