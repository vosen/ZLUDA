##############################################################################
##
##  Samples README File
##
##  Microsoft Research Detours Package
##
##  Copyright (c) Microsoft Corporation.  All rights reserved.
##

This README file describes how to set up your build environment, build
samples, and run tests.

BUILD ENVIRONMENT:
==================
We assume that you have a version of the Visual Studio IDE installed.  You can
download a free copy of the Visual Studio IDE from
https://visualstudio.microsoft.com.  During Visual Studio installation, make
sure that C/C++ tools are installed and that the Windows SDK is installed.

Clone the Detours git repo to a directory on your machine.  Choose a directory
that does not have spaces in the full path name.

BUILDING:
=========
Open a Developer Command Prompt for VS.  Note there are several different
flavors of the command prompt for different target architectures.  The
default Visual Studio Command prompt targets x86.  To target x64, choose
the "X64 Native Tools Command Prompt for VS"

Change directory to the samples directory for your git repo.  To build the
samples, type "nmake".

Note that you must build setdll and syslog in order to use many of the
other sample programs.

INSTALLING AND BUILDING VIA VCPKG:
==================================
You can download and install detours using the vcpkg(https://github.com/Microsoft/vcpkg) dependency manager:

    git clone https://github.com/Microsoft/vcpkg.git
    cd vcpkg
    ./bootstrap-vcpkg.sh
    ./vcpkg integrate install
    vcpkg install detours
    
The detours port in vcpkg is kept up to date by Microsoft team members and community contributors. If the version is out of date, please create an issue or pull request(https://github.com/Microsoft/vcpkg) on the vcpkg repository.

TESTING:
========
Each of the sample directories has a test, which can be invoked by typing
"nmake test", to demonstrate the usage of the sample.  With very few
exceptions, all of the executables also accept a "/?" command to display a
usage message.

To run all sample tests, change directory to the samples directory and type
"nmake test".  Note that some samples are architecture-specific.  Tests for
those samples be run only on supported architectures and will be skipped on
other architectures.

COMMENTS:
=========
The trace* samples log their output through the syelogd.exe daemon and hook
CreateProcessW to load themselves into any child processes.  For example,
typing "withdll -d:traceapi.dll cmd.exe" will create a command shell under
which all processes log their API calls through traceapi.dll.
