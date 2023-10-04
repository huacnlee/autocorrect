const USE_CASES = [
  {
    name: 'MDN Web Docs',
    url: 'https://developer.mozilla.org',
    className: 'dark:invert-1',
    image:
      'https://github.com/huacnlee/autocorrect/assets/5518/72fd3ebf-43e4-4cd2-b0ab-08a6700f63b2',
  },
  {
    name: 'Apache APISIX',
    url: 'https://apisix.apache.org',
    image:
      'https://github.com/huacnlee/autocorrect/assets/5518/2b286775-e619-4d7e-9c81-37479b93b18e',
  },
  {
    name: 'Longbridge',
    url: 'https://open.longportapp.com',
    className: 'scale-[1.2]',
    image:
      'https://github.com/huacnlee/autocorrect/assets/5518/4d43712d-30ac-4bad-95bc-9b571cd43660',
  },
  {
    name: 'JuiceFS',
    url: 'https://juicefs.com',
    image:
      'https://github.com/huacnlee/autocorrect/assets/5518/788634f6-080f-4c0e-8e13-0da22a5d3ae7',
  },
  {
    name: 'Lowcode Engine',
    url: 'https://lowcode-engine.cn',
    className: 'scale-[0.8]',
    image:
      'https://github.com/huacnlee/autocorrect/assets/5518/89c60e49-cda2-4e9a-a739-ef4d769c7457',
  },
  {
    name: 'Envd',
    url: 'https://envd.tensorchord.ai',
    className: 'scale-[1.2]',
    image:
      'https://github.com/huacnlee/autocorrect/assets/5518/bd055496-62aa-4d6e-accb-775c26014f15',
  },
];

export const Welcome = () => {
  return (
    <div className="container">
      <article className="mb-10 space-y-6 text-center">
        <div className="space-y-3">
          <div className="flex items-center justify-center space-x-2">
            <a href="https://github.com/huacnlee/autocorrect/actions?query=workflow%3ACI">
              <img
                src="https://github.com/huacnlee/autocorrect/workflows/CI/badge.svg"
                alt="Go"
              />
            </a>
            <a href="https://github.com/huacnlee/autocorrect/releases">
              <img
                src="https://img.shields.io/github/v/release/huacnlee/autocorrect?label=CLI&color=blue"
                alt="GitHub release (latest by date)"
              />
            </a>
            <a href="https://hub.docker.com/r/huacnlee/autocorrect">
              <img
                src="https://img.shields.io/docker/v/huacnlee/autocorrect?label=Docker&color=blue"
                alt="Docker Image Version (latest server)"
              />
            </a>
            <a href="https://crates.io/crates/autocorrect">
              <img
                src="https://img.shields.io/crates/v/autocorrect?color=1t&label=Crate"
                alt="Crates.io"
              />
            </a>
            <a href="https://www.npmjs.com/package/@huacnlee/autocorrect">
              <img
                src="https://img.shields.io/npm/v/@huacnlee/autocorrect?color=1t&label=NPM"
                alt="NPM"
              />
            </a>
            <a href="https://pypi.org/project/autocorrect-py/">
              <img
                src="https://img.shields.io/pypi/v/autocorrect-py?color=1&label=PyPI"
                alt="PyPI version"
              />
            </a>
            <a href="https://rubygems.org/gems/autocorrect-rb">
              <img
                src="https://img.shields.io/gem/v/autocorrect-rb?color=1&label=Gem"
                alt="Gem Version"
              />
            </a>
            <a href="https://repo1.maven.org/maven2/io/github/huacnlee/autocorrect-java/">
              <img
                alt="Maven Central"
                src="https://img.shields.io/maven-central/v/io.github.huacnlee/autocorrect-java?color=1&label=Maven"
              />
            </a>
          </div>
        </div>
        <div className="intro">
          <p>
            A linter and formatter for help you improve copywriting, to correct
            spaces, punctuations between CJK (Chinese, Japanese, Korean).
          </p>
        </div>
        <div className="flex flex-col justify-center space-y-2 text-center md:flex-row md:space-y-0 md:space-x-6">
          <a
            href="https://github.com/huacnlee/autocorrect/releases/latest"
            target="_blank"
            className="btn btn-download"
          >
            <div className="text-lg">Download</div>
            <div className="text-xs text-gray-200">macOS / Windows / Linux</div>
          </a>
          <a
            href="vscode:extension/huacnlee.autocorrect"
            className="btn btn-install"
          >
            <div className="text-lg">Install Extension</div>
            <div className="text-xs text-gray-200">Visual Studio Code</div>
          </a>
          <a
            href="https://plugins.jetbrains.com/plugin/20244-autocorrect"
            target="_blank"
            className="btn btn-install2"
          >
            <div className="text-lg">Install Plugin</div>
            <div className="text-xs text-gray-200">Intellij Platform</div>
          </a>
        </div>
        <div className="codeblock-wrap">
          <pre className="codeblock">
            <span>brew install autocorrect</span>
          </pre>
          <div className="text-gray-200">Or just install via this:</div>
          <pre className="codeblock">
            <span>curl -sSL https://git.io/JcGER | sh</span>
          </pre>
        </div>
        <div className="intro">
          <p>
            Like Eslint, Rubocop, Gofmt ..., AutoCorrect allow us to checking
            soure code, and output as colorized diff with corrected suggest. You
            can intergrating to CI (GitLab CI, GitHub Action, Travis CI....) for
            use to checking the contents in source code. Recognize the file
            name, and find out the strings and the comment part.
          </p>
          <p>
            基于 Rust 编写的 CLI 工具，用于「自动纠正」或「检查并建议」文案，给
            CJK（中文、日语、韩语）与英文混写的场景，补充正确的空格，同时尝试以安全的方式自动纠正标点符号等等。
          </p>
          <p>
            类似 ESlint、Rubocop、Gofmt 等工具，AutoCorrect 可以用于 CI
            环境，它提供 Lint
            功能能便捷的检测出项目中有问题的文案，起到统一规范的作用。
          </p>
          <p>
            支持各种类型源代码文件，能自动识别文件名，并准确找到字符串、注释做自动纠正。
          </p>
        </div>

        <p>
          <img
            className="w-full max-w-4xl mx-auto"
            src="https://user-images.githubusercontent.com/5518/191890126-4e0c99dc-91ce-4262-a774-3813a636eea1.png"
          />

          <img
            className="w-full max-w-4xl mx-auto"
            src="https://user-images.githubusercontent.com/5518/192738752-89a9e4f5-75cb-40af-b84d-04889d22e834.png"
          />
        </p>

        <div className="intro">
          <h2 className="text-lg">Use cases</h2>
          <div
            className="flex flex-wrap items-center place-content-center gap-4 lg:gap-6 py-8 dark:invert-[0.2]"
            style={{ colorScheme: 'revert' }}
          >
            {USE_CASES.map((item) => {
              return (
                <a href={item.url} target="_blank" title={item.name}>
                  <img
                    src={item.image}
                    className={`${item.className} h-6 lg:h-8`}
                  />
                </a>
              );
            })}
          </div>
        </div>
      </article>
    </div>
  );
};
