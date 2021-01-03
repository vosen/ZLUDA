---
name: Bug Report
about: Report a bug in Detours
title: "<header>: Problem"
labels: 'bug'
assignees: ''

---

**Describe the bug**
A clear and concise description of what the bug is. Please check that you've
read the guidelines for submitting a bug report in the 
[Bug Reports](https://github.com/microsoft/Detours/wiki/FAQ#bug-reports) section
of the FAQ.

**Command-line test case**
```
C:\Temp>type repro.cpp
#include <iostream>
#include <windows.h>
#include <detours.h>

void main() {
    // Replace this program with one demonstrating your actual bug report,
    // along with the following compilation command. Please leave compiler
    // version banners in the output (don't use /nologo), and include output
    // of your test program, if any.
    std::cout << "Test Case Result: ";
    if (DetourIsHelperProcess()) {
        std::cout << "Fail\n";
    } else {
        std::cout << "Pass\n";
    }
}

C:\Temp>cl.exe /EHsc /W4 /WX .\repro.cpp -I. ..\lib.X64\detours.lib
Microsoft (R) C/C++ Optimizing Compiler Version 19.27.29111 for x64
Copyright (C) Microsoft Corporation.  All rights reserved.

repro.cpp
Microsoft (R) Incremental Linker Version 14.27.29111.0
Copyright (C) Microsoft Corporation.  All rights reserved.

/out:repro.exe
repro.obj
..\lib.X64\detours.lib

C:\Temp>.\repro.exe
Test Case Result: Pass
```

**Expected behavior**
A clear and concise description of what you expected to happen.
Alternatively, include `static_assert` or `assert` lines in your
test case above whose failure clearly indicates the problem.

**Detours version**
* Option 1: Release version
  * Displayed on the releases page: https://github.com/microsoft/Detours/releases/
  * Example:
    ```
    Version 4.0.1 of Detours
    ```

* Option 2: git commit hash
  * Example:
    ```
    https://github.com/microsoft/Detours/commit/2195148
    ```

**Additional context**
Add any other context about the problem here.
