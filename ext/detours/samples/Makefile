##############################################################################
##
##  Makefile for Detours Test Programs.
##
##  Microsoft Research Detours Package
##
##  Copyright (c) Microsoft Corporation.  All rights reserved.
##
##  Note:
##    syelog, setdll, and withdll must be built first because a number of the
##    other samples depend on them.
##

ROOT=..
!include .\common.mak

##############################################################################

all:
    cd "$(MAKEDIR)\syelog"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS)
    cd "$(MAKEDIR)\simple"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS)
    cd "$(MAKEDIR)\slept"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS)
    cd "$(MAKEDIR)\setdll"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS)
    cd "$(MAKEDIR)\withdll"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS)
    cd "$(MAKEDIR)\cping"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS)
    cd "$(MAKEDIR)\disas"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS)
    cd "$(MAKEDIR)\dtest"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS)
    cd "$(MAKEDIR)\dumpe"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS)
    cd "$(MAKEDIR)\dumpi"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS)
    cd "$(MAKEDIR)\echo"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS)
!IF "$(DETOURS_TARGET_PROCESSOR)" != "ARM64"
    cd "$(MAKEDIR)\einst"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS)
!ENDIF
!IF "$(DETOURS_TARGET_PROCESSOR)" == "X86"
    cd "$(MAKEDIR)\excep"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS)
!ENDIF
    cd "$(MAKEDIR)\comeasy"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS)
    cd "$(MAKEDIR)\commem"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS)
!IF "$(DETOURS_TARGET_PROCESSOR)" != "ARM64"
    cd "$(MAKEDIR)\findfunc"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS)
!ENDIF
!IF "$(DETOURS_TARGET_PROCESSOR)" != "ARM" && "$(DETOURS_TARGET_PROCESSOR)" != "ARM64"
    cd "$(MAKEDIR)\member"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS)
!ENDIF
    cd "$(MAKEDIR)\region"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS)
!IF "$(DETOURS_TARGET_PROCESSOR)" == "X64" || "$(DETOURS_TARGET_PROCESSOR)" == "IA64"
    cd "$(MAKEDIR)\talloc"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS)
!ENDIF
    cd "$(MAKEDIR)\traceapi"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS)
    cd "$(MAKEDIR)\tracebld"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS)
    cd "$(MAKEDIR)\tracemem"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS)
    cd "$(MAKEDIR)\tracereg"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS)
    cd "$(MAKEDIR)\traceser"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS)
    cd "$(MAKEDIR)\tracessl"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS)
    cd "$(MAKEDIR)\tracetcp"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS)
    cd "$(MAKEDIR)\tracelnk"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS)
!IF "$(DETOURS_TARGET_PROCESSOR)" != "ARM" && "$(DETOURS_TARGET_PROCESSOR)" != "ARM64"
    cd "$(MAKEDIR)\tryman"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS)
!ENDIF
    cd "$(MAKEDIR)\impmunge"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS)
    cd "$(MAKEDIR)\dynamic_alloc"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS)
    cd "$(MAKEDIR)"

clean:
    cd "$(MAKEDIR)\syelog"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) clean
    cd "$(MAKEDIR)\simple"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) clean
    cd "$(MAKEDIR)\slept"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) clean
    cd "$(MAKEDIR)\setdll"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) clean
    cd "$(MAKEDIR)\withdll"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) clean
    cd "$(MAKEDIR)\cping"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) clean
    cd "$(MAKEDIR)\disas"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) clean
    cd "$(MAKEDIR)\dtest"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) clean
    cd "$(MAKEDIR)\dumpe"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) clean
    cd "$(MAKEDIR)\dumpi"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) clean
    cd "$(MAKEDIR)\echo"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) clean
    cd "$(MAKEDIR)\einst"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) clean
    cd "$(MAKEDIR)\excep"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) clean
    cd "$(MAKEDIR)\comeasy"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) clean
    cd "$(MAKEDIR)\commem"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) clean
    cd "$(MAKEDIR)\findfunc"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) clean
    cd "$(MAKEDIR)\member"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) clean
    cd "$(MAKEDIR)\region"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) clean
    cd "$(MAKEDIR)\talloc"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) clean
    cd "$(MAKEDIR)\traceapi"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) clean
    cd "$(MAKEDIR)\tracebld"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) clean
    cd "$(MAKEDIR)\tracemem"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) clean
    cd "$(MAKEDIR)\tracereg"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) clean
    cd "$(MAKEDIR)\traceser"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) clean
    cd "$(MAKEDIR)\tracessl"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) clean
    cd "$(MAKEDIR)\tracetcp"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) clean
    cd "$(MAKEDIR)\tracelnk"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) clean
    cd "$(MAKEDIR)\tryman"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) clean
    cd "$(MAKEDIR)\impmunge"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) clean
    cd "$(MAKEDIR)\dynamic_alloc"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) clean
    cd "$(MAKEDIR)"
    -rmdir lib32 2>nul
    -rmdir lib64 2>nul
    -rmdir include 2>nul

realclean:
    cd "$(MAKEDIR)\syelog"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) realclean
    cd "$(MAKEDIR)\simple"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) realclean
    cd "$(MAKEDIR)\slept"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) realclean
    cd "$(MAKEDIR)\setdll"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) realclean
    cd "$(MAKEDIR)\withdll"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) realclean
    cd "$(MAKEDIR)\cping"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) realclean
    cd "$(MAKEDIR)\disas"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) realclean
    cd "$(MAKEDIR)\dtest"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) realclean
    cd "$(MAKEDIR)\dumpe"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) realclean
    cd "$(MAKEDIR)\dumpi"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) realclean
    cd "$(MAKEDIR)\echo"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) realclean
    cd "$(MAKEDIR)\einst"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) realclean
    cd "$(MAKEDIR)\excep"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) realclean
    cd "$(MAKEDIR)\comeasy"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) realclean
    cd "$(MAKEDIR)\commem"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) realclean
    cd "$(MAKEDIR)\findfunc"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) realclean
    cd "$(MAKEDIR)\member"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) realclean
    cd "$(MAKEDIR)\region"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) realclean
    cd "$(MAKEDIR)\talloc"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) realclean
    cd "$(MAKEDIR)\traceapi"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) realclean
    cd "$(MAKEDIR)\tracebld"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) realclean
    cd "$(MAKEDIR)\tracemem"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) realclean
    cd "$(MAKEDIR)\tracereg"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) realclean
    cd "$(MAKEDIR)\traceser"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) realclean
    cd "$(MAKEDIR)\tracessl"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) realclean
    cd "$(MAKEDIR)\tracetcp"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) realclean
    cd "$(MAKEDIR)\tracelnk"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) realclean
    cd "$(MAKEDIR)\tryman"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) realclean
    cd "$(MAKEDIR)\impmunge"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) realclean
    cd "$(MAKEDIR)\dynamic_alloc"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) realclean
    cd "$(MAKEDIR)"
    -rmdir lib32 2>nul
    -rmdir lib64 2>nul
    -rmdir include 2>nul

test:
    cd "$(MAKEDIR)\syelog"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) test
    cd "$(MAKEDIR)\simple"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) test
!IF "$(DETOURS_TARGET_PROCESSOR)" != "ARM64"
    cd "$(MAKEDIR)\slept"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) test
    cd "$(MAKEDIR)\setdll"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) test
    cd "$(MAKEDIR)\withdll"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) test
!ENDIF
!IF "$(DETOURS_TARGET_PROCESSOR)" == "X86"
    cd "$(MAKEDIR)\cping"
#   @$(MAKE) /NOLOGO /$(MAKEFLAGS) test
!ENDIF
    cd "$(MAKEDIR)\disas"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) test
!IF "$(DETOURS_TARGET_PROCESSOR)" != "ARM64"
    cd "$(MAKEDIR)\dtest"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) test
!ENDIF
    cd "$(MAKEDIR)\dumpe"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) test
    cd "$(MAKEDIR)\dumpi"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) test
    cd "$(MAKEDIR)\echo"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) test
!IF "$(DETOURS_TARGET_PROCESSOR)" != "ARM64"
    cd "$(MAKEDIR)\einst"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) test
!ENDIF
!IF "$(DETOURS_TARGET_PROCESSOR)" == "X86"
    cd "$(MAKEDIR)\excep"
#   @$(MAKE) /NOLOGO /$(MAKEFLAGS) test
!ENDIF
!IF "$(DETOURS_TARGET_PROCESSOR)" != "ARM64"
    cd "$(MAKEDIR)\comeasy"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) test

    cd "$(MAKEDIR)\commem"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) test
    cd "$(MAKEDIR)\findfunc"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) test
    cd "$(MAKEDIR)\member"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) test
    cd "$(MAKEDIR)\region"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) test
!ENDIF
!IF "$(DETOURS_TARGET_PROCESSOR)" == "X64" || "$(DETOURS_TARGET_PROCESSOR)" == "IA64"
    cd "$(MAKEDIR)\talloc"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) test
!ENDIF
!IF "$(DETOURS_TARGET_PROCESSOR)" != "ARM64"
    cd "$(MAKEDIR)\traceapi"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) test
    cd "$(MAKEDIR)\tracebld"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) test
    cd "$(MAKEDIR)\tracemem"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) test
    cd "$(MAKEDIR)\tracereg"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) test
    cd "$(MAKEDIR)\traceser"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) test
!ENDIF
#    cd "$(MAKEDIR)\tracessl"
#    @$(MAKE) /NOLOGO /$(MAKEFLAGS) test
#    cd "$(MAKEDIR)\tracetcp"
#    @$(MAKE) /NOLOGO /$(MAKEFLAGS) test
!IF "$(DETOURS_TARGET_PROCESSOR)" != "ARM64"
    cd "$(MAKEDIR)\tracelnk"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) test
!ENDIF
    cd "$(MAKEDIR)\impmunge"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) test
    cd "$(MAKEDIR)\dynamic_alloc"
    @$(MAKE) /NOLOGO /$(MAKEFLAGS) test
    cd "$(MAKEDIR)"

##
################################################################# End of File.
