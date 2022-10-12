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
end
