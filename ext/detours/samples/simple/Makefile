##############################################################################
##
##  API Extention to Measure time slept.
##
##  Microsoft Research Detours Package
##
##  Copyright (c) Microsoft Corporation.  All rights reserved.
##

!include ..\common.mak

LIBS=$(LIBS) kernel32.lib

##############################################################################

all: dirs \
    $(BIND)\simple$(DETOURS_BITS).dll \
    $(BIND)\sleep5.exe \
    \
!IF $(DETOURS_SOURCE_BROWSING)==1
    $(OBJD)\simple$(DETOURS_BITS).bsc \
    $(OBJD)\sleep5.bsc \
!ENDIF
    option

##############################################################################

dirs:
    @if not exist $(BIND) mkdir $(BIND) && echo.   Created $(BIND)
    @if not exist $(OBJD) mkdir $(OBJD) && echo.   Created $(OBJD)

$(OBJD)\simple.obj : simple.cpp

$(OBJD)\simple.res : simple.rc

$(BIND)\simple$(DETOURS_BITS).dll $(BIND)\simple$(DETOURS_BITS).lib: \
        $(OBJD)\simple.obj $(OBJD)\simple.res $(DEPS)
    cl /LD $(CFLAGS) /Fe$(@R).dll /Fd$(@R).pdb \
        $(OBJD)\simple.obj $(OBJD)\simple.res \
        /link $(LINKFLAGS) /subsystem:console \
        /export:DetourFinishHelperProcess,@1,NONAME \
        /export:TimedSleepEx \
        $(LIBS)

$(OBJD)\simple$(DETOURS_BITS).bsc : $(OBJD)\simple.obj
    bscmake /v /n /o $@ $(OBJD)\simple.sbr

$(OBJD)\sleep5.obj : sleep5.cpp

$(BIND)\sleep5.exe : $(OBJD)\sleep5.obj $(DEPS)
    cl $(CFLAGS) /Fe$@ /Fd$(@R).pdb $(OBJD)\sleep5.obj \
        /link $(LINKFLAGS) $(LIBS) \
        /subsystem:console

$(OBJD)\sleep5.bsc : $(OBJD)\sleep5.obj
    bscmake /v /n /o $@ $(OBJD)\sleep5.sbr

##############################################################################

clean:
    -del *~ 2>nul
    -del $(BIND)\simple*.* 2>nul
    -del $(BIND)\sleep5.* 2>nul
    -rmdir /q /s $(OBJD) 2>nul

realclean: clean
    -rmdir /q /s $(OBJDS) 2>nul

############################################### Install non-bit-size binaries.

!IF "$(DETOURS_OPTION_PROCESSOR)" != ""

$(OPTD)\simple$(DETOURS_OPTION_BITS).dll:
$(OPTD)\simple$(DETOURS_OPTION_BITS).pdb:

$(BIND)\simple$(DETOURS_OPTION_BITS).dll : $(OPTD)\simple$(DETOURS_OPTION_BITS).dll
    @if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\simple$(DETOURS_OPTION_BITS).pdb : $(OPTD)\simple$(DETOURS_OPTION_BITS).pdb
    @if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).

option: \
    $(BIND)\simple$(DETOURS_OPTION_BITS).dll \
    $(BIND)\simple$(DETOURS_OPTION_BITS).pdb \

!ELSE

option:

!ENDIF

##############################################################################

test: all
    @echo -------- Reseting test binaries to initial state. ---------------------
    $(BIND)\setdll.exe -r $(BIND)\sleep5.exe
    @echo.
    @echo -------- Should not load simple$(DETOURS_BITS).dll -----------------------------------
    $(BIND)\sleep5.exe
    @echo.
    @echo -------- Adding simple$(DETOURS_BITS).dll to sleep5.exe ------------------------------
    $(BIND)\setdll.exe -d:$(BIND)\simple$(DETOURS_BITS).dll $(BIND)\sleep5.exe
    @echo.
    @echo -------- Should load simple$(DETOURS_BITS).dll statically ----------------------------
    $(BIND)\sleep5.exe
    @echo.
    @echo -------- Removing simple$(DETOURS_BITS).dll from sleep5.exe --------------------------
    $(BIND)\setdll.exe -r $(BIND)\sleep5.exe
    @echo.
    @echo -------- Should not load simple$(DETOURS_BITS).dll -----------------------------------
    $(BIND)\sleep5.exe
    @echo.
    @echo -------- Should load simple$(DETOURS_BITS).dll dynamically using withdll.exe----------
    $(BIND)\withdll.exe -d:$(BIND)\simple$(DETOURS_BITS).dll $(BIND)\sleep5.exe
    @echo.

debug: all
    windbg -o $(BIND)\withdll.exe -d:$(BIND)\simple$(DETOURS_BITS).dll $(BIND)\sleep5.exe


################################################################# End of File.
