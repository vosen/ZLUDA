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
    $(BIND)\member.exe \
!IF $(DETOURS_SOURCE_BROWSING)==1
    $(OBJD)\member.bsc
!ENDIF

clean:
    -del *~ 2> nul
    -del $(BIND)\member.* 2> nul
    -rmdir /q /s $(OBJD) 2>nul

realclean: clean
    -rmdir /q /s $(OBJDS) 2>nul

dirs:
    @if not exist $(BIND) mkdir $(BIND) && echo.   Created $(BIND)
    @if not exist $(OBJD) mkdir $(OBJD) && echo.   Created $(OBJD)

$(OBJD)\member.obj : member.cpp

$(BIND)\member.exe : $(OBJD)\member.obj $(DEPS)
    cl $(CFLAGS) /Fe$@ /Fd$(@R).pdb $(OBJD)\member.obj \
        /link $(LINKFLAGS) $(LIBS) /subsystem:console

$(OBJD)\member.bsc : $(OBJD)\member.obj
    bscmake /v /n /o $@ $(OBJD)\member.sbr

##############################################################################

test: $(BIND)\member.exe
    @echo.
    $(BIND)\member.exe
    @echo.

################################################################# End of File.
