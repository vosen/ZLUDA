##############################################################################
##
##  Makefile for Detours Test Programs - Dump Imports
##
##  Microsoft Research Detours Package
##
##  Copyright (c) Microsoft Corporation.  All rights reserved.
##

!include ..\common.mak

LIBS=$(LIBS) kernel32.lib

all: dirs \
    $(BIND)\dumpi.exe \
!IF $(DETOURS_SOURCE_BROWSING)==1
    $(OBJD)\dumpi.bsc \
!ENDIF

clean:
    -del *~ 2>nul
    -del $(BIND)\dumpi.* 2>nul
    -rmdir /q /s $(OBJD) 2>nul

realclean: clean
    -rmdir /q /s $(OBJDS) 2>nul

dirs:
    @if not exist $(BIND) mkdir $(BIND) && echo.   Created $(BIND)
    @if not exist $(OBJD) mkdir $(OBJD) && echo.   Created $(OBJD)

$(OBJD)\dumpi.obj : dumpi.cpp

$(BIND)\dumpi.exe : $(OBJD)\dumpi.obj $(DEPS)
    cl $(CFLAGS) /Fe$@ /Fd$(@R).pdb $(OBJD)\dumpi.obj \
        /link $(LINKFLAGS) $(LIBS) \
        /subsystem:console

$(OBJD)\dumpi.bsc : $(OBJD)\dumpi.obj
    bscmake /v /n /o $@ $(OBJD)\dumpi.sbr

##############################################################################

test: $(BIND)\dumpi.exe
    $(BIND)\dumpi.exe $(BIND)\slept.dll $(BIND)\sleepold.exe

################################################################# End of File.
