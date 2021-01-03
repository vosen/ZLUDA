##############################################################################
##
##  Makefile for Detours Test Programs.
##
##  Microsoft Research Detours Package
##
##  Copyright (c) Microsoft Corporation.  All rights reserved.
##

!include ..\common.mak

LIBS=$(LIBS) kernel32.lib

all: dirs \
    $(BIND)\dtarge$(DETOURS_BITS).dll \
    $(BIND)\dtest.exe \
!IF $(DETOURS_SOURCE_BROWSING)==1
    $(OBJD)\dtarge$(DETOURS_BITS).bsc \
    $(OBJD)\dtest.bsc \
!ENDIF
    option

clean:
    -del *~ *.obj *.sbr 2> nul
    -del $(BIND)\dtest.* $(BIND)\dtarge*.* 2> nul
    -rmdir /q /s $(OBJD) 2>nul

realclean: clean
    -rmdir /q /s $(OBJDS) 2>nul

dirs:
    @if not exist $(BIND) mkdir $(BIND) && echo.   Created $(BIND)
    @if not exist $(OBJD) mkdir $(OBJD) && echo.   Created $(OBJD)

$(OBJD)\dtarge.obj : dtarge.cpp

$(OBJD)\dtarge.res : dtarge.rc

$(BIND)\dtarge$(DETOURS_BITS).dll $(BIND)\dtarge$(DETOURS_BITS).lib: \
        $(OBJD)\dtarge.obj $(OBJD)\dtarge.res $(DEPS)
    cl /LD $(CFLAGS) \
		/Fe$(@R).dll \
		/Fd$(@R).pdb \
        $(OBJD)\dtarge.obj $(OBJD)\dtarge.res \
        /link $(LINKFLAGS) /subsystem:console \
        /export:Target0 \
        /export:Target1 \
        /export:Target2 \
        /export:Target3 \
        /export:Target4 \
        /export:Target5 \
        /export:Target6 \
        /export:Target7 \
        /export:Target8 \
        /export:Target9 \
        /export:Target10 \
        /export:Target11 \
        /export:Target12 \
        /export:Target13 \
        /export:Target14 \
        /export:Target15 \
        /export:Target16 \
        /export:TargetV \
        /export:TargetR \
        $(LIBS)

$(OBJD)\dtarge$(DETOURS_BITS).bsc : $(OBJD)\dtarge.obj
    bscmake /v /n /o $@ $(OBJD)\dtarge.sbr

$(OBJD)\dtest.obj : dtest.cpp

$(BIND)\dtest.exe : $(OBJD)\dtest.obj $(BIND)\dtarge$(DETOURS_BITS).lib $(DEPS)
    cl $(CFLAGS) /Fe$@ /Fd$(@R).pdb $(OBJD)\dtest.obj \
        /link $(LINKFLAGS) $(LIBS) $(BIND)\dtarge$(DETOURS_BITS).lib \
        /subsystem:console /entry:WinMainCRTStartup

$(OBJD)\dtest.bsc : $(OBJD)\dtest.obj
    bscmake /v /n /o $@ $(OBJD)\dtest.sbr

############################################### Install non-bit-size binaries.

!IF "$(DETOURS_OPTION_PROCESSOR)" != ""

$(OPTD)\dtarge$(DETOURS_OPTION_BITS).dll:
$(OPTD)\dtarge$(DETOURS_OPTION_BITS).pdb:

$(BIND)\dtarge$(DETOURS_OPTION_BITS).dll : $(OPTD)\dtarge$(DETOURS_OPTION_BITS).dll
    @if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\dtarge$(DETOURS_OPTION_BITS).pdb : $(OPTD)\dtarge$(DETOURS_OPTION_BITS).pdb
    @if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).

option: \
    $(BIND)\dtarge$(DETOURS_OPTION_BITS).dll \
    $(BIND)\dtarge$(DETOURS_OPTION_BITS).pdb \

!ELSE

option:

!ENDIF

##############################################################################

test: all
    $(BIND)\dtest.exe

################################################################# End of File.
