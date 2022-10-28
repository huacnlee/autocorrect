// autocorrect-disable
package org.example;

import io.github.huacnlee.AutoCorrect;

public class Main {
    public static void main(String[] args) {
        String output = AutoCorrect.format("Hello你好.");
        System.out.println(output);

        var configStr = "{}";
        AutoCorrect.loadConfig(configStr);
    }
}