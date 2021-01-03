##############################################################################
##
##  Detours Test Program
##
##  Microsoft Research Detours Package
##
##  Copyright (c) Microsoft Corporation.  All rights reserved.
##

!include ..\common.mak

LIBS=$(LIBS) kernel32.lib

##############################################################################

all: dirs \
    $(BIND)\echofx$(DETOURS_BITS).dll \
    $(BIND)\echonul.exe \
    \
!IF $(DETOURS_SOURCE_BROWSING)==1
    $(OBJD)\echofx$(DETOURS_BITS).bsc \
    $(OBJD)\echonul.bsc \
!ENDIF
    option

##############################################################################

dirs:
    @if not exist $(BIND) mkdir $(BIND) && echo.   Created $(BIND)
    @if not exist $(OBJD) mkdir $(OBJD) && echo.   Created $(OBJD)

$(OBJD)\echofx.obj : echofx.cpp

$(OBJD)\echofx.res : echofx.rc

$(BIND)\echofx$(DETOURS_BITS).dll $(BIND)\echofx$(DETOURS_BITS).lib: \
        $(OBJD)\echofx.obj $(OBJD)\echofx.res $(DEPS) $(BIND)\echonul.lib
    cl /LD $(CFLAGS) /Fe$@ /Fd$(@R).pdb \
        $(OBJD)\echofx.obj $(OBJD)\echofx.res \
        /link $(LINKFLAGS) /subsystem:console \
        /export:DetourFinishHelperProcess,@1,NONAME \
        /export:Mine_Echo \
        $(LIBS) $(BIND)\echonul.lib

$(OBJD)\echofx$(DETOURS_BITS).bsc : $(OBJD)\echofx.obj
    bscmake /v /n /o $@ $(OBJD)\echofx.sbr

$(OBJD)\echonul.obj : echonul.cpp
$(OBJD)\main.obj : main.cpp

$(BIND)\echonul.exe $(BIND)\echonul.lib: $(OBJD)\main.obj $(OBJD)\echonul.obj
    cl $(CFLAGS) /Zl /Fe$(BIND)\echonul.exe /Fd$(@R).pdb \
        $(OBJD)\main.obj $(OBJD)\echonul.obj \
        /link $(LINKFLAGS) \
        /export:Echo \
        /subsystem:console

$(OBJD)\echonul.bsc : echonul.obj
    bscmake /v /n /o $@ echonul.sbr

##############################################################################

clean:
    -del *~ 2>nul
    -del $(BIND)\echofx*.* 2>nul
    -del $(BIND)\echonul.* 2>nul
    -rmdir /q /s $(OBJD) 2>nul

realclean: clean
    -rmdir /q /s $(OBJDS) 2>nul

############################################### Install non-bit-size binaries.

!IF "$(DETOURS_OPTION_PROCESSOR)" != ""

$(OPTD)\echofx$(DETOURS_OPTION_BITS).dll:
$(OPTD)\echofx$(DETOURS_OPTION_BITS).pdb:

$(BIND)\echofx$(DETOURS_OPTION_BITS).dll : $(OPTD)\echofx$(DETOURS_OPTION_BITS).dll
    @if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\echofx$(DETOURS_OPTION_BITS).pdb : $(OPTD)\echofx$(DETOURS_OPTION_BITS).pdb
    @if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).

option: \
    $(BIND)\echofx$(DETOURS_OPTION_BITS).dll \
    $(BIND)\echofx$(DETOURS_OPTION_BITS).pdb \

!ELSE

option:

!ENDIF

##############################################################################

test: all
    @echo -------- Should echo nothing. --------------------------------------
    -$(BIND)\echonul.exe
    @echo -------- Should echo Hello World. ----------------------------------
    -$(BIND)\withdll.exe -d:$(BIND)\echofx$(DETOURS_BITS).dll $(BIND)\echonul.exe
    @echo.

testd: all
    @echo.
    -windbg -o -g -G $(BIND)\withdll.exe -d:$(BIND)\echofx$(DETOURS_BITS).dll $(BIND)\echonul.exe
    @echo.

################################################################# End of File.
