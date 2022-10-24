package io.github.huacnlee;

/**
 * AutoCorrect Ignorer to read .autocorrectignore and .gitignore from workdir
 *
 * Give you a isIgnored method to check file ignore status.
 */
public class Ignorer {
    private final long ptr;

    /**
     * Create a Ignorer
     * @param workdir location of .autocorrectignore and .gitignore
     */
    public Ignorer(String workdir) {
        this.ptr = AutoCorrect.nativeNewIgnorer(workdir);
    }

    /**
     * Check filepath is ignored
     * @param filepath
     * @return
     */
    public boolean isIgnored(String filepath) {
        return AutoCorrect.nativeIgnorerIsIgnored(this.ptr, filepath);
    }
}