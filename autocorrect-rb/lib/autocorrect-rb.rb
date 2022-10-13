begin
  # load the precompiled extension file
  ruby_version = /(\d+\.\d+)/.match(::RUBY_VERSION)
  require_relative "autocorrect/#{ruby_version}/autocorrect"
rescue LoadError
  require "autocorrect/autocorrect"
end
