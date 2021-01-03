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
    $(BIND)\dumpe.exe \
!IF $(DETOURS_SOURCE_BROWSING)==1
    $(OBJD)\dumpe.bsc
!ENDIF

clean:
    -del *~ 2>nul
    -del $(BIND)\dumpe.* 2>nul
    -rmdir /q /s $(OBJD) 2>nul

realclean: clean
    -rmdir /q /s $(OBJDS) 2>nul

dirs:
    @if not exist $(BIND) mkdir $(BIND) && echo.   Created $(BIND)
    @if not exist $(OBJD) mkdir $(OBJD) && echo.   Created $(OBJD)

$(OBJD)\dumpe.obj : dumpe.cpp

$(BIND)\dumpe.exe : $(OBJD)\dumpe.obj $(DEPS)
    cl $(CFLAGS) /Fe$@ /Fd$(@R).pdb $(OBJD)\dumpe.obj \
        /link $(LINKFLAGS) $(LIBS) \
        /subsystem:console

$(OBJD)\dumpe.bsc : $(OBJD)\dumpe.obj
    bscmake /v /n /o $@ $(OBJD)\dumpe.sbr

##############################################################################

test: $(BIND)\dumpe.exe
    $(BIND)\dumpe.exe $(BIND)\slept.dll

testx: $(BIND)\dumpe.exe
    cd $(MAKEDIR)\..\..\src
    nmake
    cd $(MAKEDIR)
    if exist $(SYSTEMROOT)\system32\browseui.dll $(BIND)\dumpe.exe browseui.dll

################################################################# End of File.
