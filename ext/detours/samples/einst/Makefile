##############################################################################
##
##  Makefile for Detours Test Programs.
##
##  Microsoft Research Detours Package
##
##  Copyright (c) Microsoft Corporation.  All rights reserved.
##

!include ..\common.mak

# ARM64 does not like base addresses below 4GB.
# Append two extra zeros for it.
#
!if "$(DETOURS_TARGET_PROCESSOR)" == "ARM64"
EDLL1X_BASE=0x710000000
EDLL2X_BASE=0x720000000
EDLL3X_BASE=0x730000000
!else
EDLL1X_BASE=0x7100000
EDLL2X_BASE=0x7200000
EDLL3X_BASE=0x7300000
!endif

LIBS=$(LIBS) kernel32.lib user32.lib

all: dirs \
    $(BIND)\edll1x$(DETOURS_BITS).dll \
    $(BIND)\edll2x$(DETOURS_BITS).dll \
    $(BIND)\edll3x$(DETOURS_BITS).dll \
    $(BIND)\einst.exe \
    \
!IF $(DETOURS_SOURCE_BROWSING)==1
    $(OBJD)\edll1x$(DETOURS_BITS).bsc \
    $(OBJD)\edll2x$(DETOURS_BITS).bsc \
    $(OBJD)\edll3x$(DETOURS_BITS).bsc \
    $(OBJD)\einst.bsc \
!ENDIF
	option

clean:
    -del *~ 2>nul
    -del $(BIND)\edll1x*.* 2>nul
    -del $(BIND)\edll2x*.* 2>nul
    -del $(BIND)\edll3x*.* 2>nul
    -del $(BIND)\einst.* 2>nul
    -rmdir /q /s $(OBJD) 2>nul

realclean: clean
    -rmdir /q /s $(OBJDS) 2>nul

dirs:
    @if not exist $(BIND) mkdir $(BIND) && echo.   Created $(BIND)
    @if not exist $(OBJD) mkdir $(OBJD) && echo.   Created $(OBJD)

##############################################################################

$(OBJD)\einst.obj : einst.cpp

$(BIND)\einst.exe : $(OBJD)\einst.obj $(DEPS)
    cl $(CFLAGS) /Fe$@ /Fd$(@R).pdb $(OBJD)\einst.obj \
        /link $(LINKFLAGS) $(LIBS) \
        $(BIND)\edll1x$(DETOURS_BITS).lib $(BIND)\edll2x$(DETOURS_BITS).lib $(BIND)\edll3x$(DETOURS_BITS).lib \
        /subsystem:console /entry:WinMainCRTStartup

$(OBJD)\einst.bsc : $(OBJD)\einst.obj
    bscmake /v /n /o $@ $(OBJD)\einst.sbr

$(OBJD)\edll1x.obj : edll1x.cpp

$(BIND)\edll1x$(DETOURS_BITS).dll : $(OBJD)\edll1x.obj $(DEPS)
    cl $(CFLAGS) /Fe$@ /Fd$(@R).pdb \
        $(OBJD)\edll1x.obj /LD \
        /link $(LINKFLAGS) $(LIBS) \
        /subsystem:windows \
        /base:$(EDLL1X_BASE)

$(OBJD)\edll1x$(DETOURS_BITS).bsc : $(OBJD)\edll1x.obj
    bscmake /v /n /o $@ $(OBJD)\edll1x.sbr

$(OBJD)\edll2x.obj : edll2x.cpp

$(BIND)\edll2x$(DETOURS_BITS).dll : $(OBJD)\edll2x.obj $(DEPS)
    cl $(CFLAGS) /Fe$@ /Fd$(@R).pdb \
        $(OBJD)\edll2x.obj /LD \
        /link $(LINKFLAGS) $(LIBS) \
        /subsystem:console \
        /base:$(EDLL2X_BASE)

$(OBJD)\edll2x$(DETOURS_BITS).bsc : $(OBJD)\edll2x.obj
    bscmake /v /n /o $@ $(OBJD)\edll2x.sbr

$(OBJD)\edll3x.obj : edll3x.cpp

$(BIND)\edll3x$(DETOURS_BITS).dll : $(OBJD)\edll3x.obj $(DEPS)
    cl $(CFLAGS) /Fe$@ /Fd$(@R).pdb \
        $(OBJD)\edll3x.obj /LD \
        /link $(LINKFLAGS) $(LIBS) \
        /subsystem:console \
        /base:$(EDLL3X_BASE)

$(OBJD)\edll3x$(DETOURS_BITS).bsc : $(OBJD)\edll3x.obj
    bscmake /v /n /o $@ $(OBJD)\edll3x.sbr

############################################### Install non-bit-size binaries.

!IF "$(DETOURS_OPTION_PROCESSOR)" != ""

$(OPTD)\edll1x$(DETOURS_OPTION_BITS).dll:
$(OPTD)\edll1x$(DETOURS_OPTION_BITS).pdb:
$(OPTD)\edll2x$(DETOURS_OPTION_BITS).dll:
$(OPTD)\edll2x$(DETOURS_OPTION_BITS).pdb:
$(OPTD)\edll3x$(DETOURS_OPTION_BITS).dll:
$(OPTD)\edll3x$(DETOURS_OPTION_BITS).pdb:

$(BIND)\edll1x$(DETOURS_OPTION_BITS).dll : $(OPTD)\edll1x$(DETOURS_OPTION_BITS).dll
	@if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\edll1x$(DETOURS_OPTION_BITS).pdb : $(OPTD)\edll1x$(DETOURS_OPTION_BITS).pdb
	@if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\edll2x$(DETOURS_OPTION_BITS).dll : $(OPTD)\edll2x$(DETOURS_OPTION_BITS).dll
	@if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\edll2x$(DETOURS_OPTION_BITS).pdb : $(OPTD)\edll2x$(DETOURS_OPTION_BITS).pdb
	@if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\edll3x$(DETOURS_OPTION_BITS).dll : $(OPTD)\edll3x$(DETOURS_OPTION_BITS).dll
	@if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\edll3x$(DETOURS_OPTION_BITS).pdb : $(OPTD)\edll3x$(DETOURS_OPTION_BITS).pdb
	@if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).

option: \
	$(BIND)\edll1x$(DETOURS_OPTION_BITS).dll \
	$(BIND)\edll1x$(DETOURS_OPTION_BITS).pdb \
	$(BIND)\edll2x$(DETOURS_OPTION_BITS).dll \
	$(BIND)\edll2x$(DETOURS_OPTION_BITS).pdb \
	$(BIND)\edll3x$(DETOURS_OPTION_BITS).dll \
	$(BIND)\edll3x$(DETOURS_OPTION_BITS).pdb \

!ELSE

option:

!ENDIF

##############################################################################

test: all
    $(BIND)\einst.exe

################################################################# End of File.
