For new LLVM major versions:

 * [ ] Diff llvm/include/llvm-c/ between previous and current release.
       This requires a local repo that's in sync with the remote, and tags
       will be needed. A bare repository is fine, however.

       ```
       git clone --mirror https://github.com/llvm/llvm-project.git
       git diff llvmorg-9.0.0 llvmorg-10.0.0 -- llvm/include/llvm-c/
       ```

       Apply matching changes to Rust bindings.
 * [ ] Update `links` key in Cargo.toml for new LLVM version
 * [ ] Update usage example in README.md for new crate version
 * [ ] Update CI to refer to new version

For all versions:

 * [ ] Update `version` key in Cargo.toml for new crate version
 * [ ] Commit changes
 * [ ] Tag new version; `git tag v100.1.0`
 * [ ] Update latest branch to follow master: `git branch -f llvm-10.0 master`
 * [ ] Test and publish
 * [ ] Push changes and tags; `git push --all && git push --tags`
