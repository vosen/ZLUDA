##############################################################################
##
##  API Extension to Measure time slept.
##
##  Microsoft Research Detours Package
##
##  Copyright (c) Microsoft Corporation.  All rights reserved.
##

!include ..\common.mak

LIBS=$(LIBS) kernel32.lib

##############################################################################

all: dirs \
    $(BIND)\wrotei$(DETOURS_BITS).dll \
    $(BIND)\comeasy.exe \
!IF $(DETOURS_SOURCE_BROWSING)==1
    $(OBJD)\wrotei$(DETOURS_BITS).bsc \
    $(OBJD)\comeasy.bsc \
!ENDIF
    option

##############################################################################

clean:
    -del $(BIND)\wrotei*.* 2>nul
    -del $(BIND)\comeasy.* 2>nul
    -del $(BIND)\wrotei.* *~ 2>nul
    -rmdir /q /s $(OBJD) 2>nul

realclean: clean
    -rmdir /q /s $(OBJDS) 2>nul

dirs:
    @if not exist $(BIND) mkdir $(BIND) && echo.   Created $(BIND)
    @if not exist $(OBJD) mkdir $(OBJD) && echo.   Created $(OBJD)

##############################################################################

$(OBJD)\wrotei.obj : wrotei.cpp

$(OBJD)\wrotei.res : wrotei.rc

$(BIND)\wrotei$(DETOURS_BITS).dll $(BIND)\wrotei$(DETOURS_BITS).lib: \
        $(OBJD)\wrotei.obj $(OBJD)\wrotei.res $(DEPS)
    cl /LD $(CFLAGS) /Fe$(@R).dll /Fd$(@R).pdb \
        $(OBJD)\wrotei.obj $(OBJD)\wrotei.res \
        /link $(LINKFLAGS) /subsystem:console \
        /export:DetourFinishHelperProcess,@1,NONAME \
        $(LIBS) ole32.lib

$(OBJD)\wrotei$(DETOURS_BITS).bsc : $(OBJD)\wrotei.obj
    bscmake /v /n /o $@ $(OBJD)\wrotei.sbr

$(OBJD)\comeasy.obj : comeasy.cpp

$(BIND)\comeasy.exe : $(OBJD)\comeasy.obj $(DEPS)
    cl $(CFLAGS) /Fe$@ /Fd$(@R).pdb \
        $(OBJD)\comeasy.obj \
        /link $(LINKFLAGS) $(LIBS) ole32.lib \
        /subsystem:console /fixed:no

$(OBJD)\comeasy.bsc : $(OBJD)\comeasy.obj
    bscmake /v /n /o $@ $(OBJD)\comeasy.sbr

############################################### Install non-bit-size binaries.

!IF "$(DETOURS_OPTION_PROCESSOR)" != ""

$(OPTD)\wrotei$(DETOURS_OPTION_BITS).dll:
$(OPTD)\wrotei$(DETOURS_OPTION_BITS).pdb:

$(BIND)\wrotei$(DETOURS_OPTION_BITS).dll : $(OPTD)\wrotei$(DETOURS_OPTION_BITS).dll
    @if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\wrotei$(DETOURS_OPTION_BITS).pdb : $(OPTD)\wrotei$(DETOURS_OPTION_BITS).pdb
    @if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).

option: \
    $(BIND)\wrotei$(DETOURS_OPTION_BITS).dll \
    $(BIND)\wrotei$(DETOURS_OPTION_BITS).pdb \

!ELSE

option:

!ENDIF

##############################################################################

test: all
    @echo -------- Reseting test binaries to initial state. -----------------------
    $(BIND)\setdll.exe -r $(BIND)\comeasy.exe
    @echo.
    @echo -------- Should not load slept$(DETOURS_BITS).dll --------------------------------------
    $(BIND)\comeasy.exe
    @echo.
    @echo -------- Adding wrotei$(DETOURS_BITS).dll to comeasy.exe ------------------------------
    $(BIND)\setdll.exe -d:$(BIND)\wrotei$(DETOURS_BITS).dll $(BIND)\comeasy.exe
    @echo.
    @echo -------- Should load wrotei$(DETOURS_BITS).dll ----------------------------------------
    $(BIND)\comeasy.exe
    @echo.
    @echo -------- Removing wrotei$(DETOURS_BITS).dll from comeasy.exe --------------------------
    $(BIND)\setdll.exe -r $(BIND)\comeasy.exe
    @echo.
    @echo -------- Should not load wrotei$(DETOURS_BITS).dll ------------------------------------
    $(BIND)\comeasy.exe
    @echo.
    @echo -------- Should load wrotei$(DETOURS_BITS).dll dynamically using withdll.exe ----------
    $(BIND)\withdll.exe -d:$(BIND)\wrotei$(DETOURS_BITS).dll $(BIND)\comeasy.exe
    @echo.
    @echo -------- Test completed. ------------------------------------------------

################################################################# End of File.
