##############################################################################
##
##  Makefile for Detours Test Programs.
##
##  Microsoft Research Detours Package
##
##  Copyright (c) Microsoft Corporation.  All rights reserved.
##

!include ..\common.mak

# This test is x86 only
!IF "$(DETOURS_TARGET_PROCESSOR)" == "X86" || "$(DETOURS_TARGET_PROCESSOR)" == "X64"

TARGET_NAME=dalloc
CFLAGS=\
	$(CFLAGS)\
	/EHsc\

LIBS=$(LIBS)\
	user32.lib\

all: dirs $(BIND)\$(TARGET_NAME).exe

##############################################################################

clean:
	-del $(BIND)\$(TARGET_NAME).* 2>nul
	-rmdir /q /s $(OBJD) 2>nul

realclean: clean
	-rmdir /q /s $(OBJDS) 2>nul

##############################################################################

dirs:
	@if not exist $(BIND) mkdir $(BIND) && echo.   Created $(BIND)
	@if not exist $(OBJD) mkdir $(OBJD) && echo.   Created $(OBJD)

!IF "$(DETOURS_TARGET_PROCESSOR)" == "X64"
$(OBJD)\asm.obj : x64.asm
	$(ASM) $(AFLAGS) /Fl$(OBJD)\x64.lst /Fo$(OBJD)\asm.obj x64.asm
!ELSE
$(OBJD)\asm.obj : x86.asm
	$(ASM) $(AFLAGS) /Fl$(OBJD)\x86.lst /Fo$(OBJD)\asm.obj x86.asm
!ENDIF

$(OBJD)\main.obj : main.cpp

$(BIND)\$(TARGET_NAME).exe : $(OBJD)\main.obj $(OBJD)\asm.obj $(DEPS)
	link\
		/SUBSYSTEM:CONSOLE\
		$(LINKFLAGS)\
		$(LIBS)\
		/PDB:"$(@R).pdb"\
		/OUT:"$@"\
		$**\

##############################################################################

test: all
	$(BIND)\$(TARGET_NAME).exe

##############################################################################

!ELSE

all:
  @echo The platform `$(DETOURS_TARGET_PROCESSOR)` is not supported.  Skipping.
test:
  @echo The platform `$(DETOURS_TARGET_PROCESSOR)` is not supported.  Skipping.
clean:
realclean:

!ENDIF

################################################################# End of File.
