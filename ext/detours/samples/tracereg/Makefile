##############################################################################
##
##  Utility to registry and file access APIs.
##
##  Microsoft Research Detours Package
##
##  Copyright (c) Microsoft Corporation.  All rights reserved.
##

!include ..\common.mak

LIBS=$(LIBS) kernel32.lib advapi32.lib

all: dirs \
    $(BIND)\trcreg$(DETOURS_BITS).dll \
!IF $(DETOURS_SOURCE_BROWSING)==1
    $(OBJD)\trcreg$(DETOURS_BITS).bsc \
!ENDIF
    option

dirs:
    @if not exist $(BIND) mkdir $(BIND) && echo.   Created $(BIND)
    @if not exist $(OBJD) mkdir $(OBJD) && echo.   Created $(OBJD)

clean:
    -del *~ test.txt 2>nul
    -del $(BIND)\trcreg*.* 2>nul
    -rmdir /q /s $(OBJD) 2>nul

realclean: clean
    -rmdir /q /s $(OBJDS) 2>nul

##############################################################################

$(OBJD)\trcreg.obj : trcreg.cpp

$(OBJD)\trcreg.res : trcreg.rc

$(BIND)\trcreg$(DETOURS_BITS).dll : $(OBJD)\trcreg.obj $(OBJD)\trcreg.res $(DEPS)
    cl /LD $(CFLAGS) /Fe$@ /Fd$(@R).pdb \
        $(OBJD)\trcreg.obj $(OBJD)\trcreg.res \
        /link $(LINKFLAGS) /subsystem:console \
        /export:DetourFinishHelperProcess,@1,NONAME \
        $(LIBS)

$(OBJD)\trcreg$(DETOURS_BITS).bsc : $(OBJD)\trcreg.obj
    bscmake /v /n /o $@ $(OBJD)\trcreg.sbr

############################################### Install non-bit-size binaries.

!IF "$(DETOURS_OPTION_PROCESSOR)" != ""

$(OPTD)\trcreg$(DETOURS_OPTION_BITS).dll:
$(OPTD)\trcreg$(DETOURS_OPTION_BITS).pdb:

$(BIND)\trcreg$(DETOURS_OPTION_BITS).dll : $(OPTD)\trcreg$(DETOURS_OPTION_BITS).dll
    @if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\trcreg$(DETOURS_OPTION_BITS).pdb : $(OPTD)\trcreg$(DETOURS_OPTION_BITS).pdb
    @if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).

option: \
    $(BIND)\trcreg$(DETOURS_OPTION_BITS).dll \
    $(BIND)\trcreg$(DETOURS_OPTION_BITS).pdb \

!ELSE

option:

!ENDIF

##############################################################################

test: all
    @echo -------- Logging output to test.txt ------------
    start $(BIND)\syelogd.exe /o test.txt
    $(BIND)\sleep5.exe 1
    @echo -------- Should load trcreg$(DETOURS_BITS).dll dynamically using withdll.exe ------------
    $(BIND)\withdll -d:$(BIND)\trcreg$(DETOURS_BITS).dll $(BIND)\sleepold.exe
    @echo -------- Log from syelog -------------
    type test.txt

################################################################# End of File.
