##############################################################################
##
##  Detours Test Program
##
##  Microsoft Research Detours Package
##
##  Copyright (c) Microsoft Corporation.  All rights reserved.
##

!include ..\common.mak

BIND_X86=$(ROOT)\bin.x86
BIND_X64=$(ROOT)\bin.x64

LIBS = $(LIBS) kernel32.lib

##############################################################################

all: dirs \
    $(BIND)\tstman$(DETOURS_BITS).dll \
    $(BIND)\tryman$(DETOURS_BITS).exe \
    $(BIND)\size$(DETOURS_BITS).exe \
    \
    $(BIND)\managed-x64.exe \
    $(BIND)\managed-ia64.exe \
    $(BIND)\managed-x86.exe \
    $(BIND)\managed-any.exe \
    $(BIND)\managed-any32.exe \
    \
!IF $(DETOURS_SOURCE_BROWSING)==1
    $(OBJD)\tstman$(DETOURS_BITS).bsc \
    $(OBJD)\tryman$(DETOURS_BITS).bsc \
!ENDIF
    option

##############################################################################

dirs:
    @if not exist $(BIND) mkdir $(BIND) && echo.   Created $(BIND)
    @if not exist $(OBJD) mkdir $(OBJD) && echo.   Created $(BIND)

$(OBJD)\tstman.obj : tstman.cpp

$(OBJD)\tstman.res : tstman.rc

$(BIND)\tstman$(DETOURS_BITS).dll $(BIND)\tstman$(DETOURS_BITS).lib: \
		$(OBJD)\tstman.obj $(OBJD)\tstman.res $(DEPS)
    cl /LD $(CFLAGS) /Fe$(@R).dll /Fd$(@R).pdb \
        $(OBJD)\tstman.obj $(OBJD)\tstman.res \
        /link $(LINKFLAGS) /subsystem:console \
        /export:DetourFinishHelperProcess,@1,NONAME \
        /export:Test3264 \
        $(LIBS)

$(OBJD)\tstman$(DETOURS_BITS).bsc : tstman.obj
    bscmake /v /n /o $@ tstman.sbr

$(OBJD)\tryman.obj : tryman.cpp

$(BIND)\tryman$(DETOURS_BITS).exe : $(OBJD)\tryman.obj $(DEPS) $(BIND)\tstman$(DETOURS_BITS).lib
    cl $(CFLAGS) /Fe$@ /Fd$(@R).pdb $(OBJD)\tryman.obj \
        /link $(LINKFLAGS) $(LIBS) $(BIND)\tstman$(DETOURS_BITS).lib \
        /subsystem:console

$(OBJD)\tryman$(DETOURS_BITS).bsc : $(OBJD)\tryman.obj
    bscmake /v /n /o $@ $(OBJD)\tryman.sbr

$(OBJD)\size.obj : size.cpp

$(BIND)\size$(DETOURS_BITS).exe : $(OBJD)\size.obj $(DEPS)
    cl $(CFLAGS) /Fe$@ /Fd$(@R).pdb $(OBJD)\size.obj \
        /link $(LINKFLAGS) $(LIBS) \
        /subsystem:console /fixed:no

$(OBJD)\size$(DETOURS_BITS).bsc : $(OBJD)\size.obj
    bscmake /v /n /o $@ $(OBJD)\size.sbr

$(BIND)\key.snk:
    if not exist $(BIND)\key.snk sn -k $(BIND)\key.snk

CSCFLAGS=/nowarn:1607 /unsafe- /optimize+ /debug+ /warnaserror

$(BIND)\managed-x64.exe : $(BIND)\key.snk managed.cs
    csc /nologo $(CSCFLAGS) /platform:x64 /keyfile:$(BIND)\key.snk \
        /out:$(BIND)\managed-x64.exe managed.cs

$(BIND)\managed-ia64.exe : $(BIND)\key.snk managed.cs
    csc /nologo $(CSCFLAGS) /platform:itanium /keyfile:$(BIND)\key.snk \
        /out:$(BIND)\managed-ia64.exe managed.cs

$(BIND)\managed-x86.exe : $(BIND)\key.snk managed.cs
    csc /nologo $(CSCFLAGS) /platform:x86 /keyfile:$(BIND)\key.snk \
        /out:$(BIND)\managed-x86.exe managed.cs

$(BIND)\managed-any.exe : $(BIND)\key.snk managed.cs
    csc /nologo $(CSCFLAGS) /platform:anycpu /keyfile:$(BIND)\key.snk \
        /out:$(BIND)\managed-any.exe managed.cs

$(BIND)\managed-any32.exe : $(BIND)\key.snk managed.cs
    -csc /nologo $(CSCFLAGS) /platform:anycpu32bitpreferred /keyfile:$(BIND)\key.snk \
        /out:$(BIND)\managed-any32.exe managed.cs

##############################################################################

clean:
    -del *~ 2>nul
    -del $(BIND)\managed-*.* 2>nul
    -del $(BIND)\tstman*.* 2>nul
    -del $(BIND)\tryman*.* 2>nul
    -del $(BIND)\size*.* 2>nul
    -rmdir /q /s $(OBJD) 2>nul

realclean: clean
    -rmdir /q /s $(OBJDS) 2>nul

############################################### Install non-bit-size binaries.

!IF "$(DETOURS_OPTION_PROCESSOR)" != ""

$(OPTD)\tstman$(DETOURS_OPTION_BITS).dll:
$(OPTD)\tstman$(DETOURS_OPTION_BITS).pdb:
$(OPTD)\tryman$(DETOURS_OPTION_BITS).exe:
$(OPTD)\tryman$(DETOURS_OPTION_BITS).pdb:
$(OPTD)\size$(DETOURS_OPTION_BITS).exe:
$(OPTD)\size$(DETOURS_OPTION_BITS).pdb:

$(BIND)\tstman$(DETOURS_OPTION_BITS).dll : $(OPTD)\tstman$(DETOURS_OPTION_BITS).dll
    @if exist $? copy /y $? $(BIND) >nul >nul && echo.   $@ copied.
$(BIND)\tstman$(DETOURS_OPTION_BITS).pdb : $(OPTD)\tstman$(DETOURS_OPTION_BITS).pdb
    @if exist $? copy /y $? $(BIND) >nul >nul && echo.   $@ copied.
$(BIND)\tryman$(DETOURS_OPTION_BITS).exe : $(OPTD)\tryman$(DETOURS_OPTION_BITS).exe
    @if exist $? copy /y $? $(BIND) >nul >nul && echo.   $@ copied.
$(BIND)\tryman$(DETOURS_OPTION_BITS).pdb : $(OPTD)\tryman$(DETOURS_OPTION_BITS).pdb
    @if exist $? copy /y $? $(BIND) >nul >nul && echo.   $@ copied.
$(BIND)\size$(DETOURS_OPTION_BITS).exe : $(OPTD)\size$(DETOURS_OPTION_BITS).exe
    @if exist $? copy /y $? $(BIND) >nul >nul && echo.   $@ copied.
$(BIND)\size$(DETOURS_OPTION_BITS).pdb : $(OPTD)\size$(DETOURS_OPTION_BITS).pdb
    @if exist $? copy /y $? $(BIND) >nul >nul && echo.   $@ copied.

option: \
    $(BIND)\tstman$(DETOURS_OPTION_BITS).dll \
    $(BIND)\tstman$(DETOURS_OPTION_BITS).pdb \
    $(BIND)\tryman$(DETOURS_OPTION_BITS).exe \
    $(BIND)\tryman$(DETOURS_OPTION_BITS).pdb \
    $(BIND)\size$(DETOURS_OPTION_BITS).exe \
    $(BIND)\size$(DETOURS_OPTION_BITS).pdb \

!ELSE

option:

!ENDIF

##############################################################################

# !IF "$(DETOURS_TARGET_PROCESSOR)" == "X64"
# #!MESSAGE Building for 64-bit X64.
# DETOURS_SOURCE_BROWSING = 0
# !ELSEIF "$(DETOURS_TARGET_PROCESSOR)" == "IA64"
# #!MESSAGE Building for 64-bit IA64.
# !ELSEIF "$(DETOURS_TARGET_PROCESSOR)" == "X86"
# #!MESSAGE Building for 32-bit X86.
# !ELSE

!if "$(DETOURS_OPTION_PROCESSOR)" != ""
test: all size32 size64
!else
test: all
!endif
    @echo ---- Trying native binary w/o test ----------------------
    $(BIND)\tryman$(DETOURS_BITS).exe
    @echo.
    @echo ---- Trying native binary -------------------------------
    $(BIND)\withdll.exe -d:$(BIND)\tstman$(DETOURS_BITS).dll $(BIND)\tryman$(DETOURS_BITS).exe
    @echo.
    @echo ---- Trying anycpu managed binary -----------------------
    -$(BIND)\withdll.exe -d:$(BIND)\tstman$(DETOURS_BITS).dll $(BIND)\managed-any.exe
    @echo.
    @echo ---- Trying anycpu managed 32-bit preferrred binary -----
    -if exist $(BIND)\managed-any32.exe $(BIND)\withdll.exe -d:$(BIND)\tstman$(DETOURS_BITS).dll $(BIND)\managed-any32.exe
    @echo.
    @echo ---- Trying x86 managed binary --------------------------
    -$(BIND)\withdll.exe -d:$(BIND)\tstman$(DETOURS_BITS).dll $(BIND)\managed-x86.exe
    @echo.
    @echo ---- Trying x64 managed binary --------------------------
    -$(BIND)\withdll.exe -d:$(BIND)\tstman$(DETOURS_BITS).dll $(BIND)\managed-x64.exe
    @echo.
    @echo ---- Trying ia64 managed binary -------------------------
    -$(BIND)\withdll.exe -d:$(BIND)\tstman$(DETOURS_BITS).dll $(BIND)\managed-ia64.exe
    @echo.

testm: all
    csc managed.cs
    @echo.
    $(BIND)\withdll.exe -d:$(BIND)\tstman$(DETOURS_BITS).dll $(BIND)\managed-any.exe
    @echo.

size: all
    @echo.
    $(BIND)\withdll.exe -d:$(BIND)\tstman$(DETOURS_BITS).dll $(BIND)\size$(DETOURS_BITS).exe 10
    @echo.

size32: all
    @echo.
    $(BIND)\withdll.exe -d:$(BIND)\tstman$(DETOURS_BITS).dll $(BIND)\size32.exe 10
    @echo.

size64: all
    @echo.
    $(BIND)\withdll.exe -d:$(BIND)\tstman$(DETOURS_BITS).dll $(BIND)\size64.exe 10
    @echo.

sizedbg: all
    @echo.
    windbg -o $(BIND)\withdll.exe -d:$(BIND)\tstman$(DETOURS_BITS).dll $(BIND)\size$(DETOURS_BITS).exe 10
    @echo.

tx: all
    cd $(MAKEDIR)\..\..\src
    nmake /nologo
    cd $(MAKEDIR)\..\..\samples\withdll
    nmake /nologo
    cd $(MAKEDIR)
    rem $(BIND)\withdll.exe -d:$(BIND)\tstman$(DETOURS_BITS).dll $(BIND)\managed.exe
    windbg -g -o $(BIND)\withdll.exe -d:$(BIND)\tstman$(DETOURS_BITS).dll $(BIND)\managed-any.exe

mx: all
    cd $(MAKEDIR)\..\..\src
    nmake /nologo
    cd $(MAKEDIR)\..\..\samples\withdll
    nmake /nologo
    cd $(MAKEDIR)
    $(BIND)\withdll.exe -d:$(BIND)\tstman$(DETOURS_BITS).dll $(BIND)\tryman$(DETOURS_BITS).exe

test-managed: all
    @echo ---- Trying anycpu managed binary -----------------------
    -$(BIND)\withdll.exe -d:$(BIND)\tstman$(DETOURS_BITS).dll $(BIND)\managed-any.exe
    @echo.
    @echo ---- Trying x86 managed binary --------------------------
    -$(BIND_X86)\withdll.exe -d:$(BIND_X86)\tstman32.dll $(BIND_X86)\managed-x86.exe
    @echo.
    @echo ---- Trying anycpu managed 32-bit preferrred binary -----
    -if exist $(BIND_X86)\managed-any32.exe $(BIND_X86)\withdll.exe -d:$(BIND_X86)\tstman32.dll $(BIND_X86)\managed-any32.exe
    @echo.
    @echo ---- Trying x64 managed binary --------------------------
    -$(BIND)\withdll.exe -d:$(BIND_X64)\tstman64.dll $(BIND)\managed-x64.exe

################################################################# End of File.
