# Development Guide

## Release a new version

1. Run `FROM= TO= make version` to update all version numbers in the project.

   - The `FROM` is the current version number, the `TO` is the new version number.
   - When you run the command, it will update the version number in the `Cargo.toml`, `package.json`, `gemspec` ...
   - Sometimes, this command may modify a wrong file, you need to `git diff` check the changed files and fix it manually.

1. Create a new tag with the new version number, e.g. `git tag v0.1.0`.
1. Push the tag to the remote repository.
1. Then the GitHub Actions will automatically build and publish the new version to the GitHub Package Registry.
