##############################################################################
##
##  Makefile for Detours Test Programs.
##
##  Microsoft Research Detours Package
##
##  Copyright (c) Microsoft Corporation.  All rights reserved.
##

!include ..\common.mak

# This test tests the allocation algorithm. .dlls are carefully placed
# immediately adjacent to each other to force the algorithm to look more places.

# /noentry keeps the .dlls small, so they all fit at their bases

!if "$(DETOURS_TARGET_PROCESSOR:64=)" == "$(DETOURS_TARGET_PROCESSOR)"
# 32bit bases
DETOURS_TALLOC1=0x28000000
DETOURS_TALLOC2=0x38000000
DETOURS_TALLOC3=0x48050000
DETOURS_TALLOC4=0x58000000
DETOURS_TALLOC5=0x68000000
DETOURS_TALLOC6=0x68010000
DETOURS_TALLOC7=0x68020000
DETOURS_TALLOC8=0x68030000
DETOURS_TALLOC9=0x68040000
!else
# 64bit bases
DETOURS_TALLOC1=0x280000000
DETOURS_TALLOC2=0x380000000
DETOURS_TALLOC3=0x480050000
DETOURS_TALLOC4=0x580000000
DETOURS_TALLOC5=0x680000000
DETOURS_TALLOC6=0x680010000
DETOURS_TALLOC7=0x680020000
DETOURS_TALLOC8=0x680030000
DETOURS_TALLOC9=0x680040000
!endif

LIBS=$(LIBS) kernel32.lib psapi.lib

all: dirs \
    $(BIND)\tdll1x$(DETOURS_BITS).dll \
    $(BIND)\tdll2x$(DETOURS_BITS).dll \
    $(BIND)\tdll3x$(DETOURS_BITS).dll \
    $(BIND)\tdll4x$(DETOURS_BITS).dll \
    $(BIND)\tdll5x$(DETOURS_BITS).dll \
    $(BIND)\tdll6x$(DETOURS_BITS).dll \
    $(BIND)\tdll7x$(DETOURS_BITS).dll \
    $(BIND)\tdll8x$(DETOURS_BITS).dll \
    $(BIND)\tdll9x$(DETOURS_BITS).dll \
    $(BIND)\talloc.exe \
    \
!IF $(DETOURS_SOURCE_BROWSING)==1
    $(OBJD)\tdll1x$(DETOURS_BITS).bsc \
    $(OBJD)\tdll2x$(DETOURS_BITS).bsc \
    $(OBJD)\tdll3x$(DETOURS_BITS).bsc \
    $(OBJD)\tdll4x$(DETOURS_BITS).bsc \
    $(OBJD)\tdll5x$(DETOURS_BITS).bsc \
    $(OBJD)\tdll6x$(DETOURS_BITS).bsc \
    $(OBJD)\tdll7x$(DETOURS_BITS).bsc \
    $(OBJD)\tdll8x$(DETOURS_BITS).bsc \
    $(OBJD)\tdll9x$(DETOURS_BITS).bsc \
    $(OBJD)\talloc.bsc \
!ENDIF
	option

clean:
    -del *~ 2>nul
    -del $(BIND)\tdll1x* 2>nul
    -del $(BIND)\tdll2x* 2>nul
    -del $(BIND)\tdll3x* 2>nul
    -del $(BIND)\tdll4x* 2>nul
    -del $(BIND)\tdll5x* 2>nul
    -del $(BIND)\tdll6x* 2>nul
    -del $(BIND)\tdll7x* 2>nul
    -del $(BIND)\tdll8x* 2>nul
    -del $(BIND)\tdll9x* 2>nul
    -del $(BIND)\talloc* 2>nul
    -rmdir /q /s $(OBJD) 2>nul

realclean: clean
    -rmdir /q /s $(OBJDS) 2>nul

dirs:
    @if not exist $(BIND) mkdir $(BIND) && echo.   Created $(BIND)
    @if not exist $(OBJD) mkdir $(OBJD) && echo.   Created $(OBJD)

##############################################################################

$(OBJD)\talloc.obj : talloc.cpp

$(BIND)\talloc.exe : $(OBJD)\talloc.obj $(DEPS)
    cl $(CFLAGS) /Fe$@ /Fd$(@R).pdb $(OBJD)\talloc.obj \
        /link $(LINKFLAGS) $(LIBS) \
        $(BIND)\tdll1x$(DETOURS_BITS).lib \
		$(BIND)\tdll2x$(DETOURS_BITS).lib \
		$(BIND)\tdll3x$(DETOURS_BITS).lib \
		$(BIND)\tdll4x$(DETOURS_BITS).lib \
		$(BIND)\tdll5x$(DETOURS_BITS).lib \
		$(BIND)\tdll6x$(DETOURS_BITS).lib \
		$(BIND)\tdll7x$(DETOURS_BITS).lib \
		$(BIND)\tdll8x$(DETOURS_BITS).lib \
		$(BIND)\tdll9x$(DETOURS_BITS).lib \
        /subsystem:console /entry:WinMainCRTStartup

$(OBJD)\talloc.bsc : $(OBJD)\talloc.obj
    bscmake /v /n /o $@ $(OBJD)\talloc.sbr

$(OBJD)\tdll1x.obj : tdll1x.cpp

# /noentry keeps the .dlls small, so they all fit at their bases
$(BIND)\tdll1x$(DETOURS_BITS).dll : $(OBJD)\tdll1x.obj $(DEPS)
    cl $(CFLAGS) /Fe$@ /Fd$(@R).pdb \
        $(OBJD)\tdll1x.obj /LD \
        /link $(LINKFLAGS:/profile=/fixed) $(LIBS) \
        /subsystem:windows \
        /noentry \
        /base:$(DETOURS_TALLOC1)

$(OBJD)\tdll1x$(DETOURS_BITS).bsc : $(OBJD)\tdll1x.obj
    bscmake /v /n /o $@ $(OBJD)\tdll1x.sbr

$(OBJD)\tdll2x.obj : tdll2x.cpp

# /noentry keeps the .dlls small, so they all fit at their bases
$(BIND)\tdll2x$(DETOURS_BITS).dll : $(OBJD)\tdll2x.obj $(DEPS)
    cl $(CFLAGS) /Fe$@ /Fd$(@R).pdb \
        $(OBJD)\tdll2x.obj /LD \
        /link $(LINKFLAGS:/profile=/fixed) $(LIBS) \
        /subsystem:console \
        /noentry \
        /base:$(DETOURS_TALLOC2)

$(OBJD)\tdll2x$(DETOURS_BITS).bsc : $(OBJD)\tdll2x.obj
    bscmake /v /n /o $@ $(OBJD)\tdll2x.sbr

$(OBJD)\tdll3x.obj : tdll3x.cpp

# /noentry keeps the .dlls small, so they all fit at their bases
$(BIND)\tdll3x$(DETOURS_BITS).dll : $(OBJD)\tdll3x.obj $(DEPS)
    cl $(CFLAGS) /Fe$@ /Fd$(@R).pdb \
        $(OBJD)\tdll3x.obj /LD \
        /link $(LINKFLAGS:/profile=/fixed) $(LIBS) \
        /subsystem:console \
        /noentry \
        /base:$(DETOURS_TALLOC3)

$(OBJD)\tdll3x$(DETOURS_BITS).bsc : $(OBJD)\tdll3x.obj
    bscmake /v /n /o $@ $(OBJD)\tdll3x.sbr

$(OBJD)\tdll4x.obj : tdll4x.cpp

# /noentry keeps the .dlls small, so they all fit at their bases
$(BIND)\tdll4x$(DETOURS_BITS).dll : $(OBJD)\tdll4x.obj $(DEPS)
    cl $(CFLAGS) /Fe$@ /Fd$(@R).pdb \
        $(OBJD)\tdll4x.obj /LD \
        /link $(LINKFLAGS:/profile=/fixed) $(LIBS) \
        /subsystem:console \
        /noentry \
        /base:$(DETOURS_TALLOC4)

$(OBJD)\tdll4x$(DETOURS_BITS).bsc : $(OBJD)\tdll4x.obj
    bscmake /v /n /o $@ $(OBJD)\tdll4x.sbr


$(OBJD)\tdll5x.obj : tdll5x.cpp

# /noentry keeps the .dlls small, so they all fit at their bases
$(BIND)\tdll5x$(DETOURS_BITS).dll : $(OBJD)\tdll5x.obj $(DEPS)
    cl $(CFLAGS) /Fe$@ /Fd$(@R).pdb \
        $(OBJD)\tdll5x.obj /LD \
        /link $(LINKFLAGS:/profile=/fixed) $(LIBS) \
        /subsystem:console \
        /noentry \
        /base:$(DETOURS_TALLOC5)

$(OBJD)\tdll5x$(DETOURS_BITS).bsc : $(OBJD)\tdll5x.obj
    bscmake /v /n /o $@ $(OBJD)\tdll5x.sbr

$(OBJD)\tdll6x.obj : tdll6x.cpp

# /noentry keeps the .dlls small, so they all fit at their bases
$(BIND)\tdll6x$(DETOURS_BITS).dll : $(OBJD)\tdll6x.obj $(DEPS)
    cl $(CFLAGS) /Fe$@ /Fd$(@R).pdb \
        $(OBJD)\tdll6x.obj /LD \
        /link $(LINKFLAGS:/profile=/fixed) $(LIBS) \
        /subsystem:console \
        /noentry \
        /base:$(DETOURS_TALLOC6)

$(OBJD)\tdll6x$(DETOURS_BITS).bsc : $(OBJD)\tdll6x.obj
    bscmake /v /n /o $@ $(OBJD)\tdll6x.sbr


$(OBJD)\tdll7x.obj : tdll7x.cpp

# /noentry keeps the .dlls small, so they all fit at their bases
$(BIND)\tdll7x$(DETOURS_BITS).dll : $(OBJD)\tdll7x.obj $(DEPS)
    cl $(CFLAGS) /Fe$@ /Fd$(@R).pdb \
        $(OBJD)\tdll7x.obj /LD \
        /link $(LINKFLAGS:/profile=/fixed) $(LIBS) \
        /subsystem:console \
        /noentry \
        /base:$(DETOURS_TALLOC7)

$(OBJD)\tdll7x$(DETOURS_BITS).bsc : $(OBJD)\tdll7x.obj
    bscmake /v /n /o $@ $(OBJD)\tdll7x.sbr


$(OBJD)\tdll8x.obj : tdll8x.cpp

# /noentry keeps the .dlls small, so they all fit at their bases
$(BIND)\tdll8x$(DETOURS_BITS).dll : $(OBJD)\tdll8x.obj $(DEPS)
    cl $(CFLAGS) /Fe$@ /Fd$(@R).pdb \
        $(OBJD)\tdll8x.obj /LD \
        /link $(LINKFLAGS:/profile=/fixed) $(LIBS) \
        /subsystem:console \
        /noentry \
        /base:$(DETOURS_TALLOC8)

$(OBJD)\tdll8x$(DETOURS_BITS).bsc : $(OBJD)\tdll8x.obj
    bscmake /v /n /o $@ $(OBJD)\tdll8x.sbr


$(OBJD)\tdll9x.obj : tdll9x.cpp

# /noentry keeps the .dlls small, so they all fit at their bases
$(BIND)\tdll9x$(DETOURS_BITS).dll : $(OBJD)\tdll9x.obj $(DEPS)
    cl $(CFLAGS) /Fe$@ /Fd$(@R).pdb \
        $(OBJD)\tdll9x.obj /LD \
        /link $(LINKFLAGS:/profile=/fixed) $(LIBS) \
        /subsystem:console \
        /noentry \
        /base:$(DETOURS_TALLOC9)

$(OBJD)\tdll9x$(DETOURS_BITS).bsc : $(OBJD)\tdll9x.obj
    bscmake /v /n /o $@ $(OBJD)\tdll9x.sbr

############################################### Install non-bit-size binaries.

!IF "$(DETOURS_OPTION_PROCESSOR)" != ""

$(OPTD)\tdll1x$(DETOURS_OPTION_BITS).dll:
$(OPTD)\tdll1x$(DETOURS_OPTION_BITS).pdb:
$(OPTD)\tdll2x$(DETOURS_OPTION_BITS).dll:
$(OPTD)\tdll2x$(DETOURS_OPTION_BITS).pdb:
$(OPTD)\tdll3x$(DETOURS_OPTION_BITS).dll:
$(OPTD)\tdll3x$(DETOURS_OPTION_BITS).pdb:
$(OPTD)\tdll4x$(DETOURS_OPTION_BITS).dll:
$(OPTD)\tdll4x$(DETOURS_OPTION_BITS).pdb:
$(OPTD)\tdll5x$(DETOURS_OPTION_BITS).dll:
$(OPTD)\tdll5x$(DETOURS_OPTION_BITS).pdb:
$(OPTD)\tdll6x$(DETOURS_OPTION_BITS).dll:
$(OPTD)\tdll6x$(DETOURS_OPTION_BITS).pdb:
$(OPTD)\tdll7x$(DETOURS_OPTION_BITS).dll:
$(OPTD)\tdll7x$(DETOURS_OPTION_BITS).pdb:
$(OPTD)\tdll8x$(DETOURS_OPTION_BITS).dll:
$(OPTD)\tdll8x$(DETOURS_OPTION_BITS).pdb:
$(OPTD)\tdll9x$(DETOURS_OPTION_BITS).dll:
$(OPTD)\tdll9x$(DETOURS_OPTION_BITS).pdb:

$(BIND)\tdll1x$(DETOURS_OPTION_BITS).dll : $(OPTD)\tdll1x$(DETOURS_OPTION_BITS).dll
	@if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\tdll1x$(DETOURS_OPTION_BITS).pdb : $(OPTD)\tdll1x$(DETOURS_OPTION_BITS).pdb
	@if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\tdll2x$(DETOURS_OPTION_BITS).dll : $(OPTD)\tdll2x$(DETOURS_OPTION_BITS).dll
	@if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\tdll2x$(DETOURS_OPTION_BITS).pdb : $(OPTD)\tdll2x$(DETOURS_OPTION_BITS).pdb
	@if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\tdll3x$(DETOURS_OPTION_BITS).dll : $(OPTD)\tdll3x$(DETOURS_OPTION_BITS).dll
	@if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\tdll3x$(DETOURS_OPTION_BITS).pdb : $(OPTD)\tdll3x$(DETOURS_OPTION_BITS).pdb
	@if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\tdll4x$(DETOURS_OPTION_BITS).dll : $(OPTD)\tdll4x$(DETOURS_OPTION_BITS).dll
	@if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\tdll4x$(DETOURS_OPTION_BITS).pdb : $(OPTD)\tdll4x$(DETOURS_OPTION_BITS).pdb
	@if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\tdll5x$(DETOURS_OPTION_BITS).dll : $(OPTD)\tdll5x$(DETOURS_OPTION_BITS).dll
	@if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\tdll5x$(DETOURS_OPTION_BITS).pdb : $(OPTD)\tdll5x$(DETOURS_OPTION_BITS).pdb
	@if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\tdll6x$(DETOURS_OPTION_BITS).dll : $(OPTD)\tdll6x$(DETOURS_OPTION_BITS).dll
	@if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\tdll6x$(DETOURS_OPTION_BITS).pdb : $(OPTD)\tdll6x$(DETOURS_OPTION_BITS).pdb
	@if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\tdll7x$(DETOURS_OPTION_BITS).dll : $(OPTD)\tdll7x$(DETOURS_OPTION_BITS).dll
	@if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\tdll7x$(DETOURS_OPTION_BITS).pdb : $(OPTD)\tdll7x$(DETOURS_OPTION_BITS).pdb
	@if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\tdll8x$(DETOURS_OPTION_BITS).dll : $(OPTD)\tdll8x$(DETOURS_OPTION_BITS).dll
	@if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\tdll8x$(DETOURS_OPTION_BITS).pdb : $(OPTD)\tdll8x$(DETOURS_OPTION_BITS).pdb
	@if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\tdll9x$(DETOURS_OPTION_BITS).dll : $(OPTD)\tdll9x$(DETOURS_OPTION_BITS).dll
	@if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\tdll9x$(DETOURS_OPTION_BITS).pdb : $(OPTD)\tdll9x$(DETOURS_OPTION_BITS).pdb
	@if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).

option: \
	$(BIND)\tdll1x$(DETOURS_OPTION_BITS).dll \
	$(BIND)\tdll1x$(DETOURS_OPTION_BITS).pdb \
	$(BIND)\tdll2x$(DETOURS_OPTION_BITS).dll \
	$(BIND)\tdll2x$(DETOURS_OPTION_BITS).pdb \
	$(BIND)\tdll3x$(DETOURS_OPTION_BITS).dll \
	$(BIND)\tdll3x$(DETOURS_OPTION_BITS).pdb \
	$(BIND)\tdll4x$(DETOURS_OPTION_BITS).dll \
	$(BIND)\tdll4x$(DETOURS_OPTION_BITS).pdb \
	$(BIND)\tdll5x$(DETOURS_OPTION_BITS).dll \
	$(BIND)\tdll5x$(DETOURS_OPTION_BITS).pdb \
	$(BIND)\tdll6x$(DETOURS_OPTION_BITS).dll \
	$(BIND)\tdll6x$(DETOURS_OPTION_BITS).pdb \
	$(BIND)\tdll7x$(DETOURS_OPTION_BITS).dll \
	$(BIND)\tdll7x$(DETOURS_OPTION_BITS).pdb \
	$(BIND)\tdll8x$(DETOURS_OPTION_BITS).dll \
	$(BIND)\tdll8x$(DETOURS_OPTION_BITS).pdb \
	$(BIND)\tdll9x$(DETOURS_OPTION_BITS).dll \
	$(BIND)\tdll9x$(DETOURS_OPTION_BITS).pdb \

!ELSE

option:

!ENDIF

##############################################################################

test: all
    $(BIND)\talloc.exe

################################################################# End of File.
