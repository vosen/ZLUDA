##############################################################################
##
##  Utility to trace serial (COM1, COM2, etc.) APIs.
##
##  Microsoft Research Detours Package
##
##  Copyright (c) Microsoft Corporation.  All rights reserved.
##

!include ..\common.mak

LIBS=$(LIBS) kernel32.lib

all: dirs \
    $(BIND)\trcser$(DETOURS_BITS).dll \
!IF $(DETOURS_SOURCE_BROWSING)==1
    $(OBJD)\trcser$(DETOURS_BITS).bsc \
!ENDIF
    option

dirs:
    @if not exist $(BIND) mkdir $(BIND) && echo.   Created $(BIND)
    @if not exist $(OBJD) mkdir $(OBJD) && echo.   Created $(OBJD)

clean:
    -del *~ test.txt 2>nul
    -del $(BIND)\trcser*.* 2>nul
    -rmdir /q /s $(OBJD) 2>nul

realclean: clean
    -rmdir /q /s $(OBJDS) 2>nul

##############################################################################

$(OBJD)\trcser.obj: trcser.cpp

$(OBJD)\trcser.res: trcser.rc

$(BIND)\trcser$(DETOURS_BITS).dll: $(OBJD)\trcser.obj $(OBJD)\trcser.res $(DEPS)
    cl /LD $(CFLAGS) /Fe$@ /Fd$(@R).pdb \
        $(OBJD)\trcser.obj $(OBJD)\trcser.res \
        /link $(LINKFLAGS) /subsystem:console \
        /export:DetourFinishHelperProcess,@1,NONAME \
        $(LIBS)

$(OBJD)\trcser$(DETOURS_BITS).bsc : $(OBJD)\trcser.obj
    bscmake /v /n /o $@ $(OBJD)\trcser.sbr

############################################### Install non-bit-size binaries.

!IF "$(DETOURS_OPTION_PROCESSOR)" != ""

$(OPTD)\trcser$(DETOURS_OPTION_BITS).dll:
$(OPTD)\trcser$(DETOURS_OPTION_BITS).pdb:

$(BIND)\trcser$(DETOURS_OPTION_BITS).dll : $(OPTD)\trcser$(DETOURS_OPTION_BITS).dll
    @if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\trcser$(DETOURS_OPTION_BITS).pdb : $(OPTD)\trcser$(DETOURS_OPTION_BITS).pdb
    @if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).

option: \
    $(BIND)\trcser$(DETOURS_OPTION_BITS).dll \
    $(BIND)\trcser$(DETOURS_OPTION_BITS).pdb \

!ELSE

option:

!ENDIF

##############################################################################

test: all
    @echo -------- Logging output to test.txt ------------
    start $(BIND)\syelogd.exe /o test.txt
    $(BIND)\sleep5.exe 1
    @echo -------- Should load trcser$(DETOURS_BITS).dll dynamically using withdll.exe ------------
    $(BIND)\withdll -d:$(BIND)\trcser$(DETOURS_BITS).dll $(BIND)\sleepold.exe
    @echo -------- Log from syelog -------------
    type test.txt

################################################################# End of File.
