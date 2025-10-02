##############################################################################
##
##  Makefile for Detours Test Programs.
##
##  Microsoft Research Detours Package
##
##  Copyright (c) Microsoft Corporation.  All rights reserved.
##

!include ..\common.mak

# temporarily disable this test for ARM64
!if "$(DETOURS_TARGET_PROCESSOR)" != "ARM64"

LIBS=$(LIBS) kernel32.lib

all: dirs \
    $(BIND)\disas.exe \
!IF $(DETOURS_SOURCE_BROWSING)==1
    $(OBJD)\disas.bsc
!ENDIF

clean:
    -del *~ *.obj *.sbr *.lst 2>nul
    -del $(BIND)\disas.* 2> nul 
    -rmdir /q /s $(OBJD) 2>nul

realclean: clean
    -rmdir /q /s $(OBJDS) 2>nul

dirs:
    @if not exist $(BIND) mkdir $(BIND) && echo.   Created $(BIND)
    @if not exist $(OBJD) mkdir $(OBJD) && echo.   Created $(OBJD)

!IF "$(DETOURS_TARGET_PROCESSOR)" == "X86"
$(OBJD)\disasm.obj : x86.cpp
    cl $(CFLAGS) /Fe$@ /FAcs /Fa$(OBJD)\x86.lst \
        /Fd$(@R).pdb /Fo$(OBJD)\disasm.obj /c x86.cpp
!ELSEIF "$(DETOURS_TARGET_PROCESSOR)" == "X64"
$(OBJD)\disasm.obj : x64.asm
    $(ASM) $(AFLAGS) /Fo$(OBJD)\disasm.obj /Fl$(OBJD)\x64.lst x64.asm
!ELSEIF "$(DETOURS_TARGET_PROCESSOR)" == "IA64"
$(OBJD)\disasm.obj : ia64.asm
    $(ASM) $(AFLAGS) -o $(OBJD)\disasm.obj ia64.asm
!ELSEIF "$(DETOURS_TARGET_PROCESSOR)" == "ARM"
$(OBJD)\disasm.obj : arm.asm
    $(ASM) $(AFLAGS) -list $(OBJD)\arm.lst -o $(OBJD)\disasm.obj arm.asm
!ENDIF

$(BIND)\disas.obj : disas.cpp

$(BIND)\disas.exe : $(OBJD)\disas.obj $(OBJD)\disasm.obj $(DEPS)
    cl $(CFLAGS) /Fe$@ /FAcs /Fa$(OBJD)\disas.lst /Fd$(@R).pdb \
        $(OBJD)\disas.obj $(OBJD)\disasm.obj \
        /link $(LINKFLAGS) $(LIBS) /subsystem:console /entry:WinMainCRTStartup

$(OBJD)\disas.bsc : $(OBJD)\disas.obj
    bscmake /v /n /o $@ $(OBJD)\disas.sbr

##############################################################################

test: $(BIND)\disas.exe
    $(BIND)\disas.exe

##############################################################################

!else

all:
test:
clean:
realclean:

!endif

################################################################# End of File.
