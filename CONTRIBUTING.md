## Dependencies

Development builds of ZLUDA requires following dependencies:

* CMake
* Python 3

Additionally repository have to be clone with Git submodules initalized. If you cloned the repo without initalizing submodules, do this:
```
git submodule update --init --recursive
```

## Tests

Tests should be executed with `--workspace` option to test non-default targets:
```
cargo test --workspace
```
