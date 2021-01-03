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
    $(BIND)\impmunge.exe    \
!IF $(DETOURS_SOURCE_BROWSING)==1
    $(OBJD)\impmunge.bsc
!ENDIF

##############################################################################

clean:
    -del *~ test.exe.* 2>nul
    -del $(BIND)\impmunge.* 2>nul
    -rmdir /q /s $(OBJD) 2>nul

realclean: clean
    -rmdir /q /s $(OBJDS) 2>nul

##############################################################################

dirs:
    @if not exist $(BIND) mkdir $(BIND) && echo.   Created $(BIND)
    @if not exist $(OBJD) mkdir $(OBJD) && echo.   Created $(OBJD)

$(OBJD)\impmunge.obj : impmunge.cpp

$(BIND)\impmunge.exe : $(OBJD)\impmunge.obj $(DEPS)
    cl $(CFLAGS) /Fe$@ /Fd$(@R).pdb $(OBJD)\impmunge.obj \
        /link $(LINKFLAGS) $(LIBS) imagehlp.lib /subsystem:console

$(OBJD)\impmunge.bsc : $(OBJD)\impmunge.obj
    bscmake /v /n /o $@ $(OBJD)\impmunge.sbr

##############################################################################

test: $(BIND)\impmunge.exe
    $(BIND)\impmunge.exe /m /o:test.exe.1 $(BIND)\impmunge.exe
    $(BIND)\impmunge.exe /m /l- /o:test.exe.2 test.exe.1
    $(BIND)\impmunge.exe /m /l- /o:test.exe.3 test.exe.2
    $(BIND)\impmunge.exe /m /l- /o:test.exe.4 test.exe.3
    $(BIND)\impmunge.exe /l test.exe.4
    $(BIND)\impmunge.exe /r /l- /o:test.exe.0 test.exe.4
    $(BIND)\impmunge.exe /l test.exe.0

################################################################# End of File.
