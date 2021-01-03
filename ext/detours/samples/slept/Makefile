##############################################################################
##
##  API Extension to Measure time slept.
##
##  Microsoft Research Detours Package
##
##  Copyright (c) Microsoft Corporation.  All rights reserved.
##

!include ..\common.mak

CFLAGS = $(CFLAGS:/Od=/O2)

LIBS=$(LIBS) kernel32.lib

##############################################################################

all: dirs \
    $(BIND)\slept$(DETOURS_BITS).dll \
    $(BIND)\dslept$(DETOURS_BITS).dll \
    $(BIND)\sleepold.exe \
    $(BIND)\sleepnew.exe \
    $(BIND)\sleepbed.exe \
    \
!IF $(DETOURS_SOURCE_BROWSING)==1
    $(OBJD)\slept$(DETOURS_BITS).bsc \
    $(OBJD)\dslept$(DETOURS_BITS).bsc \
    $(OBJD)\sleepold.bsc \
    $(OBJD)\sleepnew.bsc \
    $(OBJD)\sleepbed.bsc \
!ENDIF
    option

##############################################################################

dirs:
    @if not exist $(BIND) mkdir $(BIND) && echo.   Created $(BIND)
    @if not exist $(OBJD) mkdir $(OBJD) && echo.   Created $(OBJD)

$(OBJD)\slept.obj : slept.cpp verify.cpp

$(OBJD)\slept.res : slept.rc

$(BIND)\slept$(DETOURS_BITS).dll $(BIND)\slept$(DETOURS_BITS).lib: \
        $(OBJD)\slept.obj $(OBJD)\slept.res $(DEPS)
    cl /LD $(CFLAGS) /Fe$(@R).dll /Fd$(@R).pdb \
        $(OBJD)\slept.obj $(OBJD)\slept.res\
        /link $(LINKFLAGS) /subsystem:console \
        /export:DetourFinishHelperProcess,@1,NONAME \
        /export:TimedSleepEx \
        /export:UntimedSleepEx \
        /export:GetSleptTicks \
        /export:TestTicks \
        /export:TestTicksEx \
        $(LIBS)

$(OBJD)\slept$(DETOURS_BITS).bsc : $(OBJD)\slept.obj
    bscmake /v /n /o $@ $(OBJD)\slept.sbr

$(OBJD)\dslept.obj : dslept.cpp verify.cpp

$(OBJD)\dslept.res : dslept.rc

$(BIND)\dslept$(DETOURS_BITS).dll $(BIND)\dslept$(DETOURS_BITS).lib: \
        $(OBJD)\dslept.obj $(OBJD)\dslept.res $(DEPS)
    cl /LD $(CFLAGS) /Fe$(@R).dll /Fd$(@R).pdb \
        $(OBJD)\dslept.obj $(OBJD)\dslept.res \
        /link $(LINKFLAGS) /subsystem:console \
        /export:DetourFinishHelperProcess,@1,NONAME \
        /export:TimedSleepEx \
        /export:UntimedSleepEx \
        /export:GetSleptTicks \
        $(LIBS)

$(OBJD)\dslept$(DETOURS_BITS).bsc : $(OBJD)\dslept.obj
    bscmake /v /n /o $@ $(OBJD)\dslept.sbr

$(OBJD)\sleepold.obj : sleepold.cpp verify.cpp

$(BIND)\sleepold.exe : $(OBJD)\sleepold.obj $(DEPS)
    cl $(CFLAGS) /Fe$@ /Fd$(@R).pdb $(OBJD)\sleepold.obj \
        /link $(LINKFLAGS) $(LIBS) \
        /subsystem:console /fixed:no

$(OBJD)\sleepold.bsc : $(OBJD)\sleepold.obj
    bscmake /v /n /o $@ $(OBJD)\sleepold.sbr

$(OBJD)\sleepnew.obj : sleepnew.cpp verify.cpp

$(BIND)\sleepnew.exe : $(OBJD)\sleepnew.obj $(BIND)\slept$(DETOURS_BITS).lib $(DEPS)
    cl $(CFLAGS) /Fe$@ /Fd$(@R).pdb $(OBJD)\sleepnew.obj \
        /link $(LINKFLAGS) $(LIBS) \
        /subsystem:console /fixed:no $(BIND)\slept$(DETOURS_BITS).lib

$(OBJD)\sleepnew.bsc : $(OBJD)\sleepnew.obj
    bscmake /v /n /o $@ $(OBJD)\sleepnew.sbr

$(OBJD)\sleepbed.obj : sleepbed.cpp verify.cpp

$(BIND)\sleepbed.exe : $(OBJD)\sleepbed.obj $(DEPS)
    cl $(CFLAGS) /Fe$@ /Fd$(@R).pdb $(OBJD)\sleepbed.obj \
        /link $(LINKFLAGS) $(LIBS) \
        /subsystem:console /fixed:no

$(OBJD)\sleepbed.bsc : $(OBJD)\sleepbed.obj
    bscmake /v /n /o $@ $(OBJD)\sleepbed.sbr

##############################################################################

clean:
    -del *~ 2>nul
    -del $(BIND)\slept*.* 2>nul
    -del $(BIND)\dslept*.* 2>nul
    -del $(BIND)\sleepold.* 2>nul
    -del $(BIND)\sleepnew.* 2>nul
    -del $(BIND)\sleepbed.* 2>nul
    -rmdir /q /s $(OBJD) 2>nul

realclean: clean
    -rmdir /q /s $(OBJDS) 2>nul

############################################### Install non-bit-size binaries.

!IF "$(DETOURS_OPTION_PROCESSOR)" != ""

$(OPTD)\slept$(DETOURS_OPTION_BITS).dll:
$(OPTD)\slept$(DETOURS_OPTION_BITS).pdb:
$(OPTD)\dslept$(DETOURS_OPTION_BITS).dll:
$(OPTD)\dslept$(DETOURS_OPTION_BITS).pdb:

$(BIND)\slept$(DETOURS_OPTION_BITS).dll: $(OPTD)\slept$(DETOURS_OPTION_BITS).dll
    @if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\slept$(DETOURS_OPTION_BITS).pdb: $(OPTD)\slept$(DETOURS_OPTION_BITS).pdb
    @if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\dslept$(DETOURS_OPTION_BITS).dll: $(OPTD)\dslept$(DETOURS_OPTION_BITS).dll
    @if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\dslept$(DETOURS_OPTION_BITS).pdb: $(OPTD)\dslept$(DETOURS_OPTION_BITS).pdb
    @if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).

option: \
    $(BIND)\slept$(DETOURS_OPTION_BITS).dll \
    $(BIND)\slept$(DETOURS_OPTION_BITS).pdb \
    $(BIND)\dslept$(DETOURS_OPTION_BITS).dll \
    $(BIND)\dslept$(DETOURS_OPTION_BITS).pdb \

!ELSE

option:

!ENDIF

##############################################################################

skype: all
    start windbg -G -o $(BIND)\withdll.exe -d:$(BIND)\slept$(DETOURS_BITS).dll "C:\Program Files (x86)\Skype\Phone\Skype.exe"

test: all
    @echo -------- Reseting test binaries to initial state. -----------------------
    $(BIND)\setdll.exe -r $(BIND)\sleepold.exe
    @echo.
    @echo -------- Should load detour self ----------------------------------------
    $(BIND)\sleepbed.exe
    @echo.
    @echo -------- Should load slept$(DETOURS_BITS).dll statically -------------------------------
    $(BIND)\sleepnew.exe
    @echo.
    @echo -------- Should not load slept$(DETOURS_BITS).dll --------------------------------------
    $(BIND)\sleepold.exe
    @echo.
    @echo -------- Adding slept$(DETOURS_BITS).dll to sleepold.exe -------------------------------
    $(BIND)\setdll.exe -d:$(BIND)\slept$(DETOURS_BITS).dll $(BIND)\sleepold.exe
    @echo.
    @echo -------- Should load slept$(DETOURS_BITS).dll statically -------------------------------
    $(BIND)\sleepold.exe
    @echo.
    @echo -------- Replacing slept$(DETOURS_BITS).dll with dslept$(DETOURS_BITS).dll in sleepold.exe ------------
    $(BIND)\setdll.exe -r $(BIND)\sleepold.exe
    $(BIND)\setdll.exe -d:$(BIND)\dslept$(DETOURS_BITS).dll $(BIND)\sleepold.exe
    @echo.
    @echo -------- Should load dslept$(DETOURS_BITS).dll instead of slept$(DETOURS_BITS).dll --------------------
    $(BIND)\sleepold.exe
    @echo.
    @echo -------- Removing dslept$(DETOURS_BITS).dll from sleepold.exe --------------------------
    $(BIND)\setdll.exe -r $(BIND)\sleepold.exe
    @echo.
    @echo -------- Should not load dslept$(DETOURS_BITS).dll or slept$(DETOURS_BITS).dll ------------------------
    $(BIND)\sleepold.exe
    @echo.
    @echo -------- Should load slept$(DETOURS_BITS).dll dynamically using withdll.exe ------------
    $(BIND)\withdll.exe -d:$(BIND)\slept$(DETOURS_BITS).dll $(BIND)\sleepold.exe
    @echo.
    @echo -------- Test completed. ------------------------------------------------

################################################################# End of File.
