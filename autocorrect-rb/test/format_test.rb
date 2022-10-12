# autocorrect-disable
require "test_helper"

class FormatTest < ActiveSupport::TestCase
  test "format" do
    assert_equal "Hello 你好。", AutoCorrect.format("Hello你好.")
  end

  test "format_for" do
    assert_equal "a = 'Hello 你好。'", AutoCorrect.format_for("a = 'Hello你好.'", "rb")
  end
end
