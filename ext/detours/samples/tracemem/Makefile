##############################################################################
##
##  Utility to trace HeapAlloc APIs.
##
##  Microsoft Research Detours Package
##
##  Copyright (c) Microsoft Corporation.  All rights reserved.
##

!include ..\common.mak

LIBS=$(LIBS) kernel32.lib

all: dirs \
    $(BIND)\trcmem$(DETOURS_BITS).dll \
!IF $(DETOURS_SOURCE_BROWSING)==1
    $(OBJD)\trcmem$(DETOURS_BITS).bsc \
!ENDIF
    option

clean:
    -del *~ test.txt 2>nul
    -del $(BIND)\trcmem*.* 2>nul
    -rmdir /q /s $(OBJD) 2>nul

dirs:
    @if not exist $(BIND) mkdir $(BIND) && echo.   Created $(BIND)
    @if not exist $(OBJD) mkdir $(OBJD) && echo.   Created $(OBJD)

realclean: clean
    -rmdir /q /s $(OBJDS) 2>nul

##############################################################################

$(OBJD)\trcmem.obj : trcmem.cpp

$(OBJD)\trcmem.res : trcmem.rc

$(BIND)\trcmem$(DETOURS_BITS).dll : $(OBJD)\trcmem.obj $(OBJD)\trcmem.res $(DEPS)
    cl /LD $(CFLAGS) /Fe$@ /Fd$(@R).pdb \
        $(OBJD)\trcmem.obj $(OBJD)\trcmem.res \
        /link $(LINKFLAGS) /subsystem:console \
        /export:DetourFinishHelperProcess,@1,NONAME \
        $(LIBS)

$(OBJD)\trcmem$(DETOURS_BITS).bsc : $(OBJD)\trcmem.obj
    bscmake /v /n /o $@ $(OBJD)\trcmem.sbr

############################################### Install non-bit-size binaries.

!IF "$(DETOURS_OPTION_PROCESSOR)" != ""

$(OPTD)\trcmem$(DETOURS_OPTION_BITS).dll:
$(OPTD)\trcmem$(DETOURS_OPTION_BITS).pdb:

$(BIND)\trcmem$(DETOURS_OPTION_BITS).dll : $(OPTD)\trcmem$(DETOURS_OPTION_BITS).dll
    @if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\trcmem$(DETOURS_OPTION_BITS).pdb : $(OPTD)\trcmem$(DETOURS_OPTION_BITS).pdb
    @if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).

option: \
    $(BIND)\trcmem$(DETOURS_OPTION_BITS).dll \
    $(BIND)\trcmem$(DETOURS_OPTION_BITS).pdb \

!ELSE

option:

!ENDIF

##############################################################################

test: all
    @echo -------- Logging output to test.txt ------------
    start $(BIND)\syelogd.exe /o test.txt
    $(BIND)\sleep5.exe 1
    @echo -------- Should load trcmem$(DETOURS_BITS).dll dynamically using withdll.exe ------------
    $(BIND)\withdll -d:$(BIND)\trcmem$(DETOURS_BITS).dll $(BIND)\sleepold.exe
    @echo -------- Log from syelog -------------
    type test.txt

################################################################# End of File.
