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
    $(BIND)\commem.exe \
!IF $(DETOURS_SOURCE_BROWSING)==1
    $(OBJD)\commem.bsc
!ENDIF

clean:
    -del *~ *.obj *.sbr 2> nul
    -del $(BIND)\commem.* 2> nul
    -rmdir /q /s $(OBJD) 2>nul

realclean: clean
    -rmdir /q /s $(OBJDS) 2>nul

dirs:
    @if not exist $(BIND) mkdir $(BIND) && echo.   Created $(BIND)
    @if not exist $(OBJD) mkdir $(OBJD) && echo.   Created $(OBJD)

$(BIND)\commem.obj : commem.cpp

$(BIND)\commem.exe : $(OBJD)\commem.obj $(DEPS)
    cl $(CFLAGS) /Fe$@ /Fd$(@R).pdb $(OBJD)\commem.obj \
        /link $(LINKFLAGS) $(LIBS) ole32.lib /subsystem:console

$(OBJD)\commem.bsc : $(OBJD)\commem.obj
    bscmake /v /n /o $@ $(OBJD)\commem.sbr

##############################################################################

test: $(BIND)\commem.exe
    @echo.
    $(BIND)\commem.exe
    @echo.

################################################################# End of File.
