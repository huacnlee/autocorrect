package org.huacnlee;

import static org.junit.Assert.assertEquals;
import org.huacnlee.AutoCorrect;
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
        assertEquals(AutoCorrect.format("hello你好"), "hello 你好");
    }
}
