##############################################################################
##
##  Utility to trace Dynamic Linking.
##
##  Microsoft Research Detours Package
##
##  Copyright (c) Microsoft Corporation.  All rights reserved.
##

!include ..\common.mak

LIBS=$(LIBS) kernel32.lib

##############################################################################

all: dirs \
    $(BIND)\trclnk$(DETOURS_BITS).dll \
!IF $(DETOURS_SOURCE_BROWSING)==1
    $(OBJD)\trclnk$(DETOURS_BITS).bsc \
!ENDIF
    option

##############################################################################

clean:
    -del *~ test.txt 2>nul
    -del $(BIND)\trclnk*.* 2>nul
    -rmdir /q /s $(OBJD) 2>nul

realclean: clean
    -rmdir /q /s $(OBJDS) 2>nul

dirs:
    @if not exist $(BIND) mkdir $(BIND) && echo .   Created $(BIND)
    @if not exist $(OBJD) mkdir $(OBJD) && echo .   Created $(OBJD)

##############################################################################

$(OBJD)\trclnk.obj : trclnk.cpp

$(OBJD)\trclnk.res : trclnk.rc

$(BIND)\trclnk$(DETOURS_BITS).dll : $(OBJD)\trclnk.obj $(OBJD)\trclnk.res $(DEPS)
    cl /LD $(CFLAGS) /Fe$@ /Fd$(@R).pdb \
        $(OBJD)\trclnk.obj $(OBJD)\trclnk.res \
        /link $(LINKFLAGS) /subsystem:console \
        /export:DetourFinishHelperProcess,@1,NONAME \
        $(LIBS)

$(OBJD)\trclnk$(DETOURS_BITS).bsc : $(OBJD)\trclnk.obj
    bscmake /v /n /o $@ $(OBJD)\trclnk.sbr

############################################### Install non-bit-size binaries.

!IF "$(DETOURS_OPTION_PROCESSOR)" != ""

$(OPTD)\trclnk$(DETOURS_OPTION_BITS).dll:
$(OPTD)\trclnk$(DETOURS_OPTION_BITS).pdb:

$(BIND)\trclnk$(DETOURS_OPTION_BITS).dll : $(OPTD)\trclnk$(DETOURS_OPTION_BITS).dll
    @if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\trclnk$(DETOURS_OPTION_BITS).pdb : $(OPTD)\trclnk$(DETOURS_OPTION_BITS).pdb
    @if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).

option: \
    $(BIND)\trclnk$(DETOURS_OPTION_BITS).dll \
    $(BIND)\trclnk$(DETOURS_OPTION_BITS).pdb \

!ELSE

option:

!ENDIF

##############################################################################

notepad: all
    @echo -------- Logging output to test.txt ------------
    start $(BIND)\syelogd.exe /o test.txt
    $(BIND)\sleep5.exe 1
    @echo -------- Should load trclnk$(DETOURS_BITS).dll dynamically using withdll.exe ------------
    @echo .
    @echo ** NOTE NOTE NOTE NOTE NOTE NOTE NOTE NOTE NOTE NOTE NOTE **
    @echo **
    @echo ** Close the NotePad window to continue test.
    @echo **
    @echo ** NOTE NOTE NOTE NOTE NOTE NOTE NOTE NOTE NOTE NOTE NOTE **
    @echo .
    $(BIND)\withdll -d:$(BIND)\trclnk$(DETOURS_BITS).dll $(SYSTEMROOT)\system32\notepad.exe
    @echo -------- Log from syelog -------------
    type test.txt

test: all
    @echo -------- Logging output to test.txt ------------
    start $(BIND)\syelogd.exe /o test.txt
    $(BIND)\sleep5.exe 1
    @echo -------- Should load trclnk$(DETOURS_BITS).dll dynamically using withdll.exe ------------
    @echo .
    $(BIND)\withdll -d:$(BIND)\trclnk$(DETOURS_BITS).dll $(SYSTEMROOT)\system32\cmd.exe /c dir
    @echo -------- Log from syelog -------------
    type test.txt

################################################################# End of File.
