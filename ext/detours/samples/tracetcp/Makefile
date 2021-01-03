##############################################################################
##
##  Utility to trace WinSock TCP APIs.
##
##  Microsoft Research Detours Package
##
##  Copyright (c) Microsoft Corporation.  All rights reserved.
##

!include ..\common.mak

LIBS=$(LIBS) kernel32.lib ws2_32.lib

##############################################################################

all: dirs \
    $(BIND)\trctcp$(DETOURS_BITS).dll \
!IF $(DETOURS_SOURCE_BROWSING)==1
    $(OBJD)\trctcp$(DETOURS_BITS).bsc \
!ENDIF
    option

##############################################################################

dirs:
    @if not exist $(BIND) mkdir $(BIND) && echo.   Created $(BIND)
    @if not exist $(OBJD) mkdir $(OBJD) && echo.   Created $(OBJD)

$(OBJD)\trctcp.obj: trctcp.cpp

$(OBJD)\trctcp.res: trctcp.rc

$(BIND)\trctcp$(DETOURS_BITS).dll: $(OBJD)\trctcp.obj $(OBJD)\trctcp.res $(DEPS)
    cl /LD $(CFLAGS) /Fe$@ /Fd$(@R).pdb \
        $(OBJD)\trctcp.obj $(OBJD)\trctcp.res \
        /link $(LINKFLAGS) /subsystem:console \
        /export:DetourFinishHelperProcess,@1,NONAME \
        $(LIBS)

$(OBJD)\trctcp$(DETOURS_BITS).bsc : $(OBJD)\trctcp.obj
    bscmake /v /n /o $@ $(OBJD)\trctcp.sbr

##############################################################################

clean:
    -del *~ test.txt 2>nul
    -del $(BIND)\trctcp*.* 2>nul
    -rmdir /q /s $(OBJD) 2>nul

realclean: clean
    -rmdir /q /s $(OBJDS) 2>nul

############################################### Install non-bit-size binaries.

!IF "$(DETOURS_OPTION_PROCESSOR)" != ""

$(OPTD)\trctcp$(DETOURS_OPTION_BITS).dll:
$(OPTD)\trctcp$(DETOURS_OPTION_BITS).pdb:

$(BIND)\trctcp$(DETOURS_OPTION_BITS).dll : $(OPTD)\trctcp$(DETOURS_OPTION_BITS).dll
    @if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\trctcp$(DETOURS_OPTION_BITS).pdb : $(OPTD)\trctcp$(DETOURS_OPTION_BITS).pdb
    @if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).

option: \
    $(BIND)\trctcp$(DETOURS_OPTION_BITS).dll \
    $(BIND)\trctcp$(DETOURS_OPTION_BITS).pdb \

!ELSE

option:

!ENDIF

##############################################################################

test: all
    @echo -------- Logging output to test.txt ------------
    start $(BIND)\syelogd.exe /o test.txt
    $(BIND)\sleep5.exe 1
    @echo -------- Should load trctcp$(DETOURS_BITS).dll dynamically using withdll.exe ------------
    @echo.
    @echo ** NOTE NOTE NOTE NOTE NOTE NOTE NOTE NOTE NOTE NOTE NOTE **
    @echo **
    @echo ** Close the Internet Explorer window to continue test.
    @echo **
    @echo ** NOTE NOTE NOTE NOTE NOTE NOTE NOTE NOTE NOTE NOTE NOTE **
    @echo.
    $(BIND)\withdll -d:$(BIND)\trctcp$(DETOURS_BITS).dll \
        "c:\program files\Internet Explorer\iexplore.exe" "http://www.microsoft.com"
    @echo -------- Log from syelog -------------
    type test.txt

debug: all
    windbg -g -G -o $(BIND)\withdll -d:$(BIND)\trctcp$(DETOURS_BITS).dll \
        "c:\program files\Internet Explorer\iexplore.exe" "http://www.microsoft.com"

################################################################# End of File.
