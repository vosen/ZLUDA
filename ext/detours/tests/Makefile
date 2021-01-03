##############################################################################
##
##  Detours Unit Tests.
##
##  Microsoft Research Detours Package
##
##  Copyright (c) Microsoft Corporation.  All rights reserved.
##

ROOT = ..
!include ..\samples\common.mak

DEPS = $(LIBD)\detours.lib
LIBS=$(LIBS) kernel32.lib
CFLAGS=$(CFLAGS) /EHsc /DCATCH_CONFIG_NO_WINDOWS_SEH

##############################################################################

all: dirs \
    $(BIND)\unittests.exe \
    \

##############################################################################

dirs:
    @if not exist $(BIND) mkdir $(BIND) && echo.   Created $(BIND)
    @if not exist $(OBJD) mkdir $(OBJD) && echo.   Created $(OBJD)

$(OBJD)\main.obj : main.cpp
$(OBJD)\test_module_api.obj : test_module_api.cpp
$(OBJD)\test_image_api.obj : test_image_api.cpp
$(OBJD)\corruptor.obj : corruptor.cpp

$(BIND)\unittests.exe : $(OBJD)\main.obj \
   						$(OBJD)\test_module_api.obj \
   						$(OBJD)\test_image_api.obj \
						$(OBJD)\corruptor.obj $(DEPS)
    cl $(CFLAGS) /Fe$@ /Fd$(@R).pdb \
	   	$(OBJD)\main.obj \
	   	$(OBJD)\test_module_api.obj \
	   	$(OBJD)\test_image_api.obj \
	   	$(OBJD)\corruptor.obj \
        /link $(LINKFLAGS) $(LIBS) /subsystem:console

##############################################################################

clean:
    -del *~ 2>nul
    -del $(BIND)\unittests*.* 2>nul
    -rmdir /q /s $(OBJD) 2>nul

realclean: clean
    -rmdir /q /s $(OBJDS) 2>nul

option:
##############################################################################

test: all
	@cls
	$(BIND)\unittests.exe --reporter console --success --durations yes

debug: all
    windbg -o $(BIND)\unittests.exe


################################################################# End of File.
