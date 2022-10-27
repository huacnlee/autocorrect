# autocorrect-disable
require "test_helper"

class FormatTest < ActiveSupport::TestCase
  test "format" do
    assert_equal "Hello 你好。", AutoCorrect.format("Hello你好.")
  end

  test "format_for" do
    assert_equal "a = 'Hello 你好。'", AutoCorrect.format_for("a = 'Hello你好.'", "rb")
  end

  test "lint_for" do
    result = AutoCorrect.lint_for("a = 'Hello你好.'", "test.rb")
    assert_equal(
      {
        "filepath" => "test.rb",
        "lines" => [
          {
            "line" => 1, "col" => 5, "new" => "'Hello 你好。'", "old" => "'Hello你好.'", "severity" => 1
          }
        ],
        "error" => ""
      }, result
    )
  end

  test "load_config" do
    AutoCorrect.load_config(%({ textRules: { "你好hello": 0 } }))
    assert_equal AutoCorrect.format("Hello你好."), "Hello 你好。"
    assert_equal AutoCorrect.format("你好hello."), "你好hello."
  end

  test "Ignorer" do
    ignorer = AutoCorrect::Ignorer.new("../")
    assert ignorer.ignored?("README.md")
    assert ignorer.ignored?("src/lib.rs")
    assert ignorer.ignored?("target/foo/bar")
    assert !ignorer.ignored?("Cagro.toml")
    assert !ignorer.ignored?("autocorrect-rb.gemspec")
  end
end
