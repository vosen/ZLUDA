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

##############################################################################

all: dirs \
    $(BIND)\withdll.exe \
!IF $(DETOURS_SOURCE_BROWSING)==1
    $(OBJD)\withdll.bsc \
!ENDIF
	option

clean:
    -del *~ 2>nul
    -del $(BIND)\withdll.* 2>nul
    -rmdir /q /s $(OBJD) 2>nul

realclean: clean
    -rmdir /q /s $(OBJDS) 2>nul

##############################################################################

dirs:
    @if not exist $(BIND) mkdir $(BIND) && echo.   Created $(BIND)
    @if not exist $(OBJD) mkdir $(OBJD) && echo.   Created $(OBJD)

$(OBJD)\withdll.obj : withdll.cpp

$(BIND)\withdll.exe : $(OBJD)\withdll.obj $(DEPS)
    cl $(CFLAGS) /Fe$@ /Fd$(@R).pdb $(OBJD)\withdll.obj \
        /link $(LINKFLAGS) $(LIBS) /subsystem:console

$(OBJD)\withdll.bsc : $(OBJD)\withdll.obj
    bscmake /v /n /o $@ $(OBJD)\withdll.sbr

############################################### Install non-bit-size binaries.

option:

##############################################################################

test: all
    $(BIND)\withdll.exe -d:$(BIND)\slept$(DETOURS_BITS).dll $(BIND)\sleepold.exe
    $(BIND)\withdll.exe -v -d:$(BIND)\slept$(DETOURS_BITS).dll $(BIND)\sleepold.exe

debug: all
    windbg  -c ".srcfix;l+s;l+t" -o \
		$(BIND)\withdll.exe -d:$(BIND)\slept$(DETOURS_BITS).dll $(BIND)\sleepold.exe

################################################################# End of File.
