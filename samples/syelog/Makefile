##############################################################################
##
##  Makefile for Detours.
##
##  Microsoft Research Detours Package
##
##  Copyright (c) Microsoft Corporation.  All rights reserved.
##
##############################################################################

TARGETOS=WINNT
!include ..\common.mak

LIBS=$(LIBS) kernel32.lib

##############################################################################

all: dirs \
    $(INCD)\syelog.h        \
    $(LIBD)\syelog.lib  \
    $(BIND)\syelogd.exe \
    \
    $(BIND)\sltest.exe  \
    $(BIND)\sltestp.exe     \
    \
!IF $(DETOURS_SOURCE_BROWSING)==1
    $(OBJD)\syelogd.bsc \
    $(OBJD)\sltest.bsc  \
    $(OBJD)\sltestp.bsc     \
!ENDIF

##############################################################################
##
clean:
    -del *~ test.txt 2> nul
    -del $(INCD)\syelog.* 2>nul
    -del $(LIBD)\syelog.* 2>nul
    -del $(BIND)\syelogd.* 2>nul
    -del $(BIND)\sltest.* 2>nul
    -del $(BIND)\sltestp.* 2>nul
    -rmdir /q /s $(OBJD) 2>nul

realclean: clean
    -rmdir /q /s $(OBJDS) 2>nul

##############################################################################

dirs:
    @if not exist $(INCD) mkdir $(INCD) && echo.   Created $(INCD)
    @if not exist $(LIBD) mkdir $(LIBD) && echo.   Created $(LIBD)
    @if not exist $(BIND) mkdir $(BIND) && echo.   Created $(BIND)
    @if not exist $(OBJD) mkdir $(OBJD) && echo.   Created $(OBJD)

$(OBJD)\syelog.obj : syelog.cpp syelog.h
$(OBJD)\syelogd.obj: syelogd.cpp syelog.h
$(OBJD)\sltest.obj: sltest.cpp syelog.h
$(OBJD)\sltestp.obj: sltestp.cpp syelog.h

$(INCD)\syelog.h : syelog.h
    copy syelog.h $@

$(LIBD)\syelog.lib : $(OBJD)\syelog.obj
    link /lib $(LIBFLAGS) /out:$@ $(OBJD)\syelog.obj

$(BIND)\sltest.exe: $(OBJD)\sltest.obj $(OBJD)\syelog.obj $(DEPS)
    $(CC) $(CFLAGS) /Fe$@ /Fd$(@R).pdb $(OBJD)\sltest.obj \
        /link $(LINKFLAGS) $(LIBS)

$(OBJD)\sltest.bsc : $(OBJD)\sltest.obj
    bscmake /v /n /o $@ $(OBJD)\sltest.sbr

$(BIND)\sltestp.exe: $(OBJD)\sltestp.obj $(DEPS)
    $(CC) $(CFLAGS) /Fe$@ /Fd$(@R).pdb $(OBJD)\sltestp.obj \
        /link $(LINKFLAGS) $(LIBS)

$(OBJD)\sltestp.bsc : $(OBJD)\sltestp.obj
    bscmake /v /n /o $@ $(OBJD)\sltestp.sbr

$(LIBD)\detours.lib:
    cd $(ROOT)\src
    nmake /nologo
    cd $(MAKEDIR)

$(BIND)\syelogd.exe: $(OBJD)\syelogd.obj $(DEPS)
    $(CC) $(CFLAGS) /Fe$@ /Fd$(@R).pdb $(OBJD)\syelogd.obj \
        /link $(LINKFLAGS) ws2_32.lib mswsock.lib advapi32.lib

$(OBJD)\syelogd.bsc : $(OBJD)\syelogd.obj
    bscmake /v /n /o $@ $(OBJD)\syelogd.sbr

##############################################################################

test: $(BIND)\syelogd.exe $(BIND)\sltest.exe $(BIND)\sltestp.exe
    @echo -------- Logging output to test.txt ------------
    start $(BIND)\syelogd.exe test.txt
    $(BIND)\sleep5.exe 1
    $(BIND)\sltestp.exe
    $(BIND)\sltest.exe /x
    type test.txt

################################################################# End of File.
