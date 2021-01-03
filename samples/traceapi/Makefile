##############################################################################
##
##  Utility to trace Win32 APIs.
##
##  Microsoft Research Detours Package
##
##  Copyright (c) Microsoft Corporation.  All rights reserved.
##

!include ..\common.mak

LIBS=$(LIBS) kernel32.lib gdi32.lib user32.lib shell32.lib advapi32.lib ole32.lib ws2_32.lib

##############################################################################

all: dirs \
    $(BIND)\trcapi$(DETOURS_BITS).dll \
    $(BIND)\testapi.exe \
!IF $(DETOURS_SOURCE_BROWSING)==1
    $(OBJD)\trcapi$(DETOURS_BITS).bsc \
    $(OBJD)\testapi.bsc \
!ENDIF
    option

##############################################################################

clean:
    -del *~ test.txt 2>nul
    -del $(BIND)\trcapi*.* $(BIND)\testapi.* 2>nul
    -rmdir /q /s $(OBJD) 2>nul

realclean: clean
    -rmdir /q /s $(OBJDS) 2>nul

dirs:
    @if not exist $(BIND) mkdir $(BIND) && echo.   Created $(BIND)
    @if not exist $(OBJD) mkdir $(OBJD) && echo.   Created $(OBJD)

$(OBJD)\trcapi.obj : trcapi.cpp _win32.cpp

$(OBJD)\trcapi.res : trcapi.rc

$(BIND)\trcapi$(DETOURS_BITS).dll: $(OBJD)\trcapi.obj $(OBJD)\trcapi.res $(DEPS)
    cl /LD $(CFLAGS) /Fe$@ /Fd$(@R).pdb \
        $(OBJD)\trcapi.obj $(OBJD)\trcapi.res \
        /link $(LINKFLAGS) /release /subsystem:console \
        /export:DetourFinishHelperProcess,@1,NONAME \
        $(LIBS)

$(OBJD)\trcapi$(DETOURS_BITS).bsc : $(OBJD)\trcapi.obj
    bscmake /v /n /o $@ $(OBJD)\trcapi.sbr

$(OBJD)\testapi.obj : testapi.cpp trcapi.cpp _win32.cpp

$(BIND)\testapi.exe : $(OBJD)\testapi.obj $(DEPS)
    cl $(CFLAGS) /Fe$@ /Fd$(@R).pdb $(OBJD)\testapi.obj \
        /link $(LINKFLAGS) $(LIBS) \
        /subsystem:console /fixed:no

$(OBJD)\testapi.bsc : $(OBJD)\testapi.obj
    bscmake /v /n /o $@ $(OBJD)\testapi.sbr

############################################### Install non-bit-size binaries.

!IF "$(DETOURS_OPTION_PROCESSOR)" != ""

$(OPTD)\trcapi$(DETOURS_OPTION_BITS).dll:
$(OPTD)\trcapi$(DETOURS_OPTION_BITS).pdb:

$(BIND)\trcapi$(DETOURS_OPTION_BITS).dll : $(OPTD)\trcapi$(DETOURS_OPTION_BITS).dll
    @if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\trcapi$(DETOURS_OPTION_BITS).pdb : $(OPTD)\trcapi$(DETOURS_OPTION_BITS).pdb
    @if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).

option: \
    $(BIND)\trcapi$(DETOURS_OPTION_BITS).dll \
    $(BIND)\trcapi$(DETOURS_OPTION_BITS).pdb \

!ELSE

option:

!ENDIF

##############################################################################

test: all
    @echo -------- Logging output to test.txt ------------
    start $(BIND)\syelogd.exe /o test.txt
    $(BIND)\sleep5.exe 1
    @echo -------- Should load trcapi$(DETOURS_BITS).dll dynamically using withdll.exe ------------
    $(BIND)\withdll -d:$(BIND)\trcapi$(DETOURS_BITS).dll $(BIND)\sleepold.exe
    @echo -------- Log from syelog -------------
    type test.txt

debug: all
    @echo -------- Logging output to test.txt ------------
    start $(BIND)\syelogd.exe /o test.txt
    $(BIND)\sleep5.exe 1
    @echo -------- Should load trcapi$(DETOURS_BITS).dll dynamically using withdll.exe ------------
    windbg -o -g -G $(BIND)\withdll -d:$(BIND)\trcapi$(DETOURS_BITS).dll $(BIND)\sleepold.exe
    @echo -------- Log from syelog -------------
    type test.txt

calc: all
    @echo -------- Logging output to test.txt ------------
    start $(BIND)\syelogd.exe /q /o test.txt
    $(BIND)\sleep5.exe 1
    @echo -------- Should load trcapi$(DETOURS_BITS).dll dynamically using withdll.exe ------------
    $(BIND)\withdll -d:$(BIND)\trcapi$(DETOURS_BITS).dll $(SYSTEMROOT)\System32\calc.exe
    @echo -------- Log from syelog -------------
    wc test.txt

explorer: all
    @echo -------- Logging output to test.txt ------------
    start $(BIND)\syelogd.exe /q /o test.txt
    $(BIND)\sleep5.exe 1
    @echo -------- Should load trcapi$(DETOURS_BITS).dll dynamically using withdll.exe ------------
    $(BIND)\withdll -d:$(BIND)\trcapi$(DETOURS_BITS).dll $(SYSTEMROOT)\explorer.exe
    @echo -------- Log from syelog -------------
    wc test.txt

wordpad: all
    @echo -------- Logging output to test.txt ------------
    start $(BIND)\syelogd.exe /q /o test.txt
    $(BIND)\sleep5.exe 1
    @echo -------- Should load trcapi$(DETOURS_BITS).dll dynamically using withdll.exe ------------
    $(BIND)\withdll -d:$(BIND)\trcapi$(DETOURS_BITS).dll $(PROGRAMFILES)\Windows NT\Accessories\wordpad.exe
    @echo -------- Log from syelog -------------
    wc test.txt

notepad: all
    @echo -------- Logging output to test.txt ------------
    start $(BIND)\syelogd.exe /q /o test.txt
    $(BIND)\sleep5.exe 1
    @echo -------- Should load trcapi$(DETOURS_BITS).dll dynamically using withdll.exe ------------
    $(BIND)\withdll -d:$(BIND)\trcapi$(DETOURS_BITS).dll $(SYSTEMROOT)\notepad.exe
    @echo -------- Log from syelog -------------
    wc test.txt

ie: all
    @echo -------- Logging output to test.txt ------------
    start $(BIND)\syelogd.exe /q /o test.txt
    $(BIND)\sleep5.exe 1
    @echo -------- Should load trcapi$(DETOURS_BITS).dll dynamically using withdll.exe ------------
    windbg -g -G -o $(BIND)\withdll -d:$(BIND)\trcapi$(DETOURS_BITS).dll "$(PROGRAMFILES)\Internet Explorer\iexplore.exe"
    @echo -------- Log from syelog -------------
    wc test.txt

ws: all
    $(BIND)\withdll -d:$(BIND)\trcapi$(DETOURS_BITS).dll $(BIND)\WebServiceTester.exe

################################################################# End of File.
