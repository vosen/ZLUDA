##############################################################################
##
##  Utility to trace WinSock SSL APIs.
##
##  Microsoft Research Detours Package
##
##  Copyright (c) Microsoft Corporation.  All rights reserved.
##

!include ..\common.mak

LIBS=$(LIBS) kernel32.lib ws2_32.lib secur32.lib

##############################################################################

all: dirs \
    $(BIND)\trcssl$(DETOURS_BITS).dll \
!IF $(DETOURS_SOURCE_BROWSING)==1
    $(OBJD)\trcssl$(DETOURS_BITS).bsc \
!ENDIF
    option

##############################################################################

dirs:
    @if not exist $(BIND) mkdir $(BIND) && echo.   Created $(BIND)
    @if not exist $(OBJD) mkdir $(OBJD) && echo.   Created $(OBJD)

$(OBJD)\trcssl.obj : trcssl.cpp

$(OBJD)\trcssl.res : trcssl.rc

$(BIND)\trcssl$(DETOURS_BITS).dll : $(OBJD)\trcssl.obj $(OBJD)\trcssl.res $(DEPS)
    cl /LD $(CFLAGS) /Fe$@ /Fd$(@R).pdb \
        $(OBJD)\trcssl.obj $(OBJD)\trcssl.res \
        /link $(LINKFLAGS) /subsystem:console \
        /export:DetourFinishHelperProcess,@1,NONAME \
        $(LIBS)

$(OBJD)\trcssl$(DETOURS_BITS).bsc : $(OBJD)\trcssl.obj
    bscmake /v /n /o $@ $(OBJD)\trcssl.sbr

##############################################################################

clean:
    -del *~ test.txt 2>nul
    -del $(BIND)\trcssl*.* 2>nul
    -rmdir /q /s $(OBJD) 2>nul

realclean: clean
    -rmdir /q /s $(OBJDS) 2>nul

############################################### Install non-bit-size binaries.

!IF "$(DETOURS_OPTION_PROCESSOR)" != ""

$(OPTD)\trcssl$(DETOURS_OPTION_BITS).dll:
$(OPTD)\trcssl$(DETOURS_OPTION_BITS).pdb:

$(BIND)\trcssl$(DETOURS_OPTION_BITS).dll : $(OPTD)\trcssl$(DETOURS_OPTION_BITS).dll
    @if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\trcssl$(DETOURS_OPTION_BITS).pdb : $(OPTD)\trcssl$(DETOURS_OPTION_BITS).pdb
    @if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).

option: \
    $(BIND)\trcssl$(DETOURS_OPTION_BITS).dll \
    $(BIND)\trcssl$(DETOURS_OPTION_BITS).pdb \

!ELSE

option:

!ENDIF

##############################################################################

test: all
    @echo -------- Logging output to test.txt ------------
    start $(BIND)\syelogd.exe /o test.txt
    $(BIND)\sleep5.exe 1
    @echo -------- Should load trcssl$(DETOURS_BITS).dll dynamically using withdll.exe ------------
    @echo.
    @echo ** NOTE NOTE NOTE NOTE NOTE NOTE NOTE NOTE NOTE NOTE NOTE **
    @echo **
    @echo ** Close the Internet Explorer window to continue test.
    @echo **
    @echo ** NOTE NOTE NOTE NOTE NOTE NOTE NOTE NOTE NOTE NOTE NOTE **
    @echo.
    $(BIND)\withdll -d:$(BIND)\trcssl$(DETOURS_BITS).dll \
        "c:\program files\Internet Explorer\iexplore.exe" "https://www.microsoft.com"
    @echo -------- Log from syelog -------------
    type test.txt

################################################################# End of File.
