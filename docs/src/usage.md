# Usage

> [!WARNING]  
> This version of ZLUDA is under heavy development and will likely not work with your application yet. In the meantime, you are encouraged to try it and report results.

## How to get it

ZLUDA evolves quickly. Download the [most recent pre-release version](https://github.com/vosen/ZLUDA/releases). Periodically, we mark a pre-release version as stable, but you don't have to wait for that.

## Usage

### Windows
You should have a recent AMD GPU driver ("AMD Software: Adrenalin Edition") installed.\
To run your application either:
* (_Recommended_) Copy all ZLUDA files (including `nvcuda.dll`) from `zluda` (if you downloaded a zip package) or `target\release` (if you built from sources) into a path which your application uses to load CUDA. Paths vary application to application, but usually it's the directory where the .exe file is located
* Use ZLUDA launcher:
    ```
    <ZLUDA_DIRECTORY>\zluda_with.exe -- <APPLICATION> <APPLICATION_ARGUMENTS>
    ```
    ZLUDA launcher is known to be buggy and incomplete, but it's less invasive

### Linux

Run your application like this:
* Recommended method
    ```
    LD_LIBRARY_PATH="<ZLUDA_DIRECTORY>:$LD_LIBRARY_PATH" <APPLICATION> <APPLICATION_ARGUMENTS>
    ```

    where `<ZLUDA_DIRECTORY>` is the directory which contains ZLUDA-provided `libcuda.so`: `zluda` if you downloaded a prebuilt package or `target/release` if you built from sources.

* Alternative method
    ```
    LD_PRELOAD="<ZLUDA_DIRECTORY>/zluda_preload" <APPLICATION> <APPLICATION_ARGUMENTS>
    ```

    where `<ZLUDA_DIRECTORY>` is the directory which contains ZLUDA-provided `libcuda.so`: `zluda` if you downloaded a prebuilt package or `target/release` if you built from sources.



### macOS

Not supported