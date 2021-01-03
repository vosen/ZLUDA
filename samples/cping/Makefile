##############################################################################
##
##  Makefile for Detours Test Programs.
##
##  Microsoft Research Detours Package
##
##  Copyright (c) Microsoft Corporation.  All rights reserved.
##

!include ..\common.mak

LIBS=$(LIBS) \
	kernel32.lib \
	user32.lib \
	shell32.lib \
	uuid.lib \
    ole32.lib \
    rpcrt4.lib \
    advapi32.lib \
    wsock32.lib \

# RpcProxy.h uses #ifdef WIN32.

!if "$(DETOURS_TARGET_PROCESSOR)" == "ARM"
CFLAGS = $(CFLAGS) /D_WIN32_WINNT=0x0500
!else
CFLAGS = $(CFLAGS) /D_WIN32_WINNT=0x0400
!endif

CFLAGS = $(CFLAGS) /Fd$(OBJD)\vc.pdb \
        /DCONST_VTABLE \
        /DCOBJMACROS -DWIN32 -DNT

C__FLAGS=-DENTRY_PREFIX=iping_ -DREGISTER_PROXY_DLL 
CPPFLAGS=

##############################################################################

.SUFFIXES: .c .cpp .h .idl .obj .res .rc

{$(OBJD)}.c{$(OBJD)}.obj:
    $(CC) $(CFLAGS:/W4=/W3) $(C__FLAGS) /I$(OBJD) /Fo$(OBJD)\ /c $<

!ifdef DETOURS_ANALYZE
.cpp{$(OBJD)}.obj:
    $(CC) $(CFLAGS) $(CPPFLAGS) /I$(OBJD) /Fo$(OBJD)\ /c $<
!else
.cpp{$(OBJD)}.obj::
    $(CC) $(CFLAGS) $(CPPFLAGS) /I$(OBJD) /Fo$(OBJD)\ /c $<
!endif

.rc{$(OBJD)}.res:
    rc /nologo /Fo$@ .\$(*B).rc

##############################################################################
##
C__FLAGS=-DENTRY_PREFIX=iping_ -DREGISTER_PROXY_DLL 
CPPFLAGS=


MIDLFLAGS=/nologo /Oif /no_format_opt

!IF "$(DETOURS_TARGET_PROCESSOR)" == "X86"
MIDLFLAGS=$(MIDLFLAGS) /no_robust /win32
!ELSEIF "$(DETOURS_TARGET_PROCESSOR)" == "IA64"
MIDLFLAGS=$(MIDLFLAGS) /ia64
!ELSEIF "$(DETOURS_TARGET_PROCESSOR)" == "X64"
MIDLFLAGS=$(MIDLFLAGS) /x64
!ELSEIF "$(DETOURS_TARGET_PROCESSOR)" == "ARM"
MIDLFLAGS=$(MIDLFLAGS) /arm32
!ELSEIF "$(DETOURS_TARGET_PROCESSOR)" == "ARM64"
MIDLFLAGS=$(MIDLFLAGS) /arm64
!ENDIF

OBJS = \
    $(OBJD)\cping.obj   \
    \
    $(OBJD)\iping_i.obj \
    $(OBJD)\iping_p.obj \
    $(OBJD)\iping_d.obj \

##############################################################################

all: dirs \
    $(BIND)\cping.exe   \
!IF $(DETOURS_SOURCE_BROWSING)==1
    $(OBJD)\cping.bsc
!ENDIF

##############################################################################

clean:
    -del iping.h *.c *.obj *.sbr *~ 2>nul
    -del $(BIND)\cping.* 2>nul
    -rmdir /q /s $(OBJD) 2>nul

realclean: clean
    -rmdir /q /s $(OBJDS) 2>nul

##############################################################################

dirs:
    @if not exist $(BIND) mkdir $(BIND) && echo.   Created $(BIND)
    @if not exist $(OBJD) mkdir $(OBJD) && echo.   Created $(OBJD)

$(OBJD)\cping.bsc : $(OBJS)
    bscmake /v /n /o $@ $(OBJS:.obj=.sbr)

$(BIND)\cping.exe : $(OBJS) $(DEPS)
    cl $(CFLAGS) /Fe$@ $(OBJS) /link $(LINKFLAGS) \
        /subsystem:console $(LIBS)

$(OBJD)\cping.obj: cping.cpp $(OBJD)\iping.h

##############################################################################
##
$(OBJD)\iping.h $(OBJD)\iping_d.c $(OBJD)\iping_i.c $(OBJD)\iping_p.c : iping.idl
    midl $(MIDLFLAGS) /out $(OBJD) /prefix all iping_ /dlldata iping_d.c iping.idl

$(OBJD)\iping_i.obj: $(OBJD)\iping_i.c
$(OBJD)\iping_p.obj: $(OBJD)\iping_p.c $(OBJD)\iping.h
$(OBJD)\iping_d.obj: $(OBJD)\iping_d.c

##############################################################################

test: $(BIND)\cping.exe
    start $(BIND)\cping.exe /s
    $(BIND)\cping.exe /p localhost

################################################################# End of File.
