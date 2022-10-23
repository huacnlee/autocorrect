import static org.junit.Assert.assertEquals;
import org.junit.Test;

/**
 * Unit test for simple App.
 */
public class AutoCorrectTest 
{
    /**
     * Rigorous Test :-)
     */
    @Test
    public void shouldAnswerWithTrue()
    {
        assertEquals(AutoCorrect.format("Hello你好."), "Hello 你好。");
        assertEquals(AutoCorrect.formatFor("// Hello你好,这是Java注释.", "test.java"), "// Hello 你好，这是 Java 注释。");
    }
}
