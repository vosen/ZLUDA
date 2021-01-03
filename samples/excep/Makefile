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
    $(BIND)\excep.exe \
!IF $(DETOURS_SOURCE_BROWSING)==1
    $(OBJD)\excep.bsc
!ENDIF

clean:
    -del *~ 2>nul
    -del $(BIND)\excep.* 2>nul
    -rmdir /q /s $(OBJD) 2>nul

realclean: clean
    -rmdir /q /s $(OBJDS) 2>nul

dirs:
    @if not exist $(BIND) mkdir $(BIND) && echo.   Created $(BIND)
    @if not exist $(OBJD) mkdir $(OBJD) && echo.   Created $(OBJD)

$(OBJD)\excep.obj : excep.cpp
$(OBJD)\firstexc.obj : firstexc.cpp

$(BIND)\excep.exe : $(OBJD)\excep.obj $(OBJD)\firstexc.obj $(DEPS)
    cl $(CFLAGS) /Fe$@ /Fd$(@R).pdb $(OBJD)\excep.obj $(OBJD)\firstexc.obj \
        /link $(LINKFLAGS) $(LIBS) /subsystem:console /entry:WinMainCRTStartup

$(OBJD)\excep.bsc : $(OBJD)\excep.obj
    bscmake /v /n /o $@ $(OBJD)\excep.sbr

##############################################################################

test: $(BIND)\excep.exe
    $(BIND)\excep.exe

################################################################# End of File.
