##############################################################################
##
##  Program to test DetourFindFunction.
##
##  Microsoft Research Detours Package
##
##  Copyright (c) Microsoft Corporation.  All rights reserved.
##

!include ..\common.mak

# ARM64 does not like base addresses below 4GB.
# Append two extra zeros for it.
#
!if "$(DETOURS_TARGET_PROCESSOR)" == "ARM64"
TARGET_BASE=0x190000000
EXTEND_BASE=0x1a0000000
!else
TARGET_BASE=0x1900000
EXTEND_BASE=0x1a00000
!endif

LIBS=$(LIBS) kernel32.lib

##############################################################################

all: dirs \
    $(BIND)\target$(DETOURS_BITS).dll \
    $(BIND)\extend$(DETOURS_BITS).dll \
    $(BIND)\findfunc.exe \
    $(BIND)\symtest.exe \
    $(BIND)\dbghelp.dll \
    \
!IF $(DETOURS_SOURCE_BROWSING)==1
    $(OBJD)\target$(DETOURS_BITS).bsc \
    $(OBJD)\extend$(DETOURS_BITS).bsc \
    $(OBJD)\findfunc.bsc \
    $(OBJD)\symtest.bsc \
!ENDIF
    option

##############################################################################

dirs:
    @if not exist $(BIND) mkdir $(BIND) && echo.   Created $(BIND)
    @if not exist $(OBJD) mkdir $(OBJD) && echo.   Created $(OBJD)

$(OBJD)\target.obj : target.cpp

$(OBJD)\target.res : target.rc

$(BIND)\target$(DETOURS_BITS).dll $(BIND)\target$(DETOURS_BITS).lib: \
        $(OBJD)\target.obj $(OBJD)\target.res $(DEPS)
    cl /LD $(CFLAGS) /Fe$(@R).dll /Fd$(@R).pdb \
        $(OBJD)\target.obj $(OBJD)\target.res \
        /link $(LINKFLAGS) /subsystem:console \
        /export:Target \
        /base:$(TARGET_BASE) \
        $(LIBS)

$(OBJD)\target$(DETOURS_BITS).bsc : $(OBJD)\target.obj
    bscmake /v /n /o $@ $(OBJD)\target.sbr

$(OBJD)\extend.obj : extend.cpp

$(OBJD)\extend.res : extend.rc

$(BIND)\extend$(DETOURS_BITS).dll $(BIND)\extend$(DETOURS_BITS).lib: \
        $(OBJD)\extend.obj $(OBJD)\extend.res $(DEPS)
    cl /LD $(CFLAGS) /Fe$(@R).dll /Fd$(@R).pdb \
        $(OBJD)\extend.obj $(OBJD)\extend.res \
        /link $(LINKFLAGS) /subsystem:console \
        /export:DetourFinishHelperProcess,@1,NONAME \
        /base:$(EXTEND_BASE) \
        $(LIBS)

$(OBJD)\extend$(DETOURS_BITS).bsc : $(OBJD)\extend.obj
    bscmake /v /n /o $@ $(OBJD)\extend.sbr

$(OBJD)\findfunc.obj : findfunc.cpp

$(BIND)\findfunc.exe : $(OBJD)\findfunc.obj $(BIND)\target$(DETOURS_BITS).lib $(DEPS)
    cl $(CFLAGS) /Fe$@ /Fd$(@R).pdb $(OBJD)\findfunc.obj \
        /link $(LINKFLAGS) $(LIBS) \
        /subsystem:console /fixed:no $(BIND)\target$(DETOURS_BITS).lib

$(OBJD)\findfunc.bsc : $(OBJD)\findfunc.obj
    bscmake /v /n /o $@ $(OBJD)\findfunc.sbr

$(OBJD)\symtest.obj : symtest.cpp

$(BIND)\symtest.exe : $(OBJD)\symtest.obj $(BIND)\target$(DETOURS_BITS).lib $(DEPS)
    cl $(CFLAGS) /Fe$@ /Fd$(@R).pdb $(OBJD)\symtest.obj \
        /link $(LINKFLAGS) $(LIBS) \
        /subsystem:console /fixed:no $(BIND)\target$(DETOURS_BITS).lib

$(OBJD)\symtest.bsc : $(OBJD)\symtest.obj
    bscmake /v /n /o $@ $(OBJD)\symtest.sbr

# We try to get the 64-bit dbghelp first because it is a lot more useful.
$(BIND)\dbghelp.dll : {"$(PROGRAMFILES)\Debugging Tools for Windows 64-bit";$(PATH)}dbghelp.dll
    -copy $** $(BIND)\dbghelp.dll 

##############################################################################

clean:
    -del *~ 2>nul
    -del $(BIND)\target*.* $(BIND)\extend*.* 2>nul
    -del $(BIND)\findfunc.* $(BIND)\symtest.* $(BIND)\dbghelp.dll 2>nul
    -rmdir /q /s $(OBJD) 2>nul

realclean: clean
    -rmdir /q /s $(OBJDS) 2>nul

############################################### Install non-bit-size binaries.

!IF "$(DETOURS_OPTION_PROCESSOR)" != ""

$(OPTD)\extend$(DETOURS_OPTION_BITS).dll:
$(OPTD)\extend$(DETOURS_OPTION_BITS).pdb:
$(OPTD)\target$(DETOURS_OPTION_BITS).dll:
$(OPTD)\target$(DETOURS_OPTION_BITS).pdb:

$(BIND)\extend$(DETOURS_OPTION_BITS).dll : $(OPTD)\extend$(DETOURS_OPTION_BITS).dll
    @if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\extend$(DETOURS_OPTION_BITS).pdb : $(OPTD)\extend$(DETOURS_OPTION_BITS).pdb
    @if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\target$(DETOURS_OPTION_BITS).dll : $(OPTD)\target$(DETOURS_OPTION_BITS).dll
    @if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\target$(DETOURS_OPTION_BITS).pdb : $(OPTD)\target$(DETOURS_OPTION_BITS).pdb
    @if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).

option: \
    $(BIND)\extend$(DETOURS_OPTION_BITS).dll \
    $(BIND)\extend$(DETOURS_OPTION_BITS).pdb \
    $(BIND)\target$(DETOURS_OPTION_BITS).dll \
    $(BIND)\target$(DETOURS_OPTION_BITS).pdb \

!ELSE

option:

!ENDIF

##############################################################################

verbose: all
    cls
    $(BIND)\symtest.exe

test: all
    @echo -------- Reseting test binaries to initial state. -----------------------
    $(BIND)\setdll.exe -r $(BIND)\findfunc.exe
    @echo.
    @echo -------- Should not load extend$(DETOURS_BITS).dll--------------------------------------
    $(BIND)\findfunc.exe
    @echo.
    @echo -------- Adding extend$(DETOURS_BITS).dll to findfunc.exe ------------------------------
    $(BIND)\setdll.exe -d:$(BIND)\extend$(DETOURS_BITS).dll $(BIND)\findfunc.exe
    @echo.
    @echo -------- Should load extend$(DETOURS_BITS).dll statically ------------------------------
    $(BIND)\findfunc.exe
    @echo.
    @echo -------- Removing extend$(DETOURS_BITS).dll from findfunc.exe --------------------------
    $(BIND)\setdll.exe -r $(BIND)\findfunc.exe
    @echo.
    @echo -------- Should not load extend$(DETOURS_BITS).dll -------------------------------------
    $(BIND)\findfunc.exe
    @echo.
    @echo -------- Should load extend$(DETOURS_BITS).dll dynamically using withdll.exe -----------
    $(BIND)\withdll.exe -d:$(BIND)\extend$(DETOURS_BITS).dll $(BIND)\findfunc.exe
    @echo.
    @echo -------- Should list symbols using symtest.exe -----------
    $(BIND)\symtest.exe
    @echo.
    @echo -------- Test completed. ------------------------------------------------

################################################################# End of File.
