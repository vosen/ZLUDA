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
    $(BIND)\region.exe \
!IF $(DETOURS_SOURCE_BROWSING)==1
    $(OBJD)\region.bsc
!ENDIF

clean:
    -del *~ 2> nul
    -del $(BIND)\region.* 2> nul
    -rmdir /q /s $(OBJD) 2>nul

realclean: clean
    -rmdir /q /s $(OBJDS) 2>nul

dirs:
    @if not exist $(BIND) mkdir $(BIND) && echo.   Created $(BIND)
    @if not exist $(OBJD) mkdir $(OBJD) && echo.   Created $(OBJD)

$(OBJD)\region.obj : region.cpp

$(BIND)\region.exe : $(OBJD)\region.obj $(DEPS)
    cl $(CFLAGS) /Fe$@ /Fd$(@R).pdb $(OBJD)\region.obj \
        /link $(LINKFLAGS) $(LIBS) /subsystem:console

$(OBJD)\region.bsc : $(OBJD)\region.obj
    bscmake /v /n /o $@ $(OBJD)\region.sbr

##############################################################################

test: $(BIND)\region.exe
    @echo.
    $(BIND)\region.exe
    @echo.

################################################################# End of File.
