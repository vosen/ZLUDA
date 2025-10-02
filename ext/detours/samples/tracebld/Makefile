##############################################################################
##
##  Utility to registry and file access APIs.
##
##  Microsoft Research Detours Package
##
##  Copyright (c) Microsoft Corporation.  All rights reserved.
##

!include ..\common.mak

LIBS=$(LIBS) kernel32.lib

##############################################################################

all: dirs \
    $(BIND)\trcbld$(DETOURS_BITS).dll \
    $(BIND)\tracebld.exe \
    \
!IF $(DETOURS_SOURCE_BROWSING)==1
    $(OBJD)\trcbld$(DETOURS_BITS).bsc    \
    $(OBJD)\tracebld.bsc    \
!ENDIF
    option

##############################################################################

clean:
    -del *~ test.txt log.*.xml 2>nul
    -del $(BIND)\tracebld.* $(BIND)\trcbld*.* 2>nul
    -rmdir /q /s $(OBJD) 2>nul

realclean: clean
    -rmdir /q /s $(OBJDS) 2>nul

dirs:
    @if not exist $(BIND) mkdir $(BIND) && echo.   Created $(BIND)
    @if not exist $(OBJD) mkdir $(OBJD) && echo.   Created $(OBJD)

##############################################################################

$(OBJD)\trcbld.obj : trcbld.cpp
$(OBJD)\trcbld.res : trcbld.rc
$(BIND)\trcbld$(DETOURS_BITS).dll : $(OBJD)\trcbld.obj $(OBJD)\trcbld.res $(DEPS)
    cl /LD $(CFLAGS) /Fe$@ /Fd$(@R).pdb \
        $(OBJD)\trcbld.obj $(OBJD)\trcbld.res \
        /link $(LINKFLAGS) /release /subsystem:console \
        /export:DetourFinishHelperProcess,@1,NONAME \
        $(LIBS)

$(OBJD)\trcbld$(DETOURS_BITS).bsc : $(OBJD)\trcbld.obj
    bscmake /v /n /o $@ $(OBJD)\trcbld.sbr

$(OBJD)\tracebld.obj : tracebld.cpp

$(BIND)\tracebld.exe : $(OBJD)\tracebld.obj $(DEPS)
    cl $(CFLAGS) /Fe$@ /Fd$(@R).pdb $(OBJD)\tracebld.obj \
        /link $(LINKFLAGS) $(LIBS) \
        /subsystem:console /fixed:no

$(OBJD)\tracebld.bsc : $(OBJD)\tracebld.obj
    bscmake /v /n /o $@ $(OBJD)\tracebld.sbr

############################################### Install non-bit-size binaries.

!IF "$(DETOURS_OPTION_PROCESSOR)" != ""

$(OPTD)\trcbld$(DETOURS_OPTION_BITS).dll:
$(OPTD)\trcbld$(DETOURS_OPTION_BITS).pdb:

$(BIND)\trcbld$(DETOURS_OPTION_BITS).dll : $(OPTD)\trcbld$(DETOURS_OPTION_BITS).dll
    @if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\trcbld$(DETOURS_OPTION_BITS).pdb : $(OPTD)\trcbld$(DETOURS_OPTION_BITS).pdb
    @if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).

option: \
    $(BIND)\trcbld$(DETOURS_OPTION_BITS).dll \
    $(BIND)\trcbld$(DETOURS_OPTION_BITS).pdb \

!ELSE

option:

!ENDIF

##############################################################################

test: all
    -del log.*.xml 2>nul
    $(BIND)\tracebld.exe /o:log %COMSPEC% /c dir
    @echo -------- Log from log.00000000 ---------------------
    type log.00000000.xml

test0: all
    -del log.*.xml 2>nul
    $(BIND)\tracebld.exe /o:log %COMSPEC% /c xx.cmd
    @echo -------- Log from log.00000000 ---------------------
    type log.00000000.xml
    @echo -------- Log from log.00000001 ---------------------
    type log.00000001.xml

test1: all
    set FooBAR=1
    -del log.*.xml 2>nul
    @echo -------- Logging output to log ------------
    -rmdir /q /s obj 2>nul
    $(BIND)\tracebld.exe /o:log nmake.exe test2
    @echo -------- Log from log.00000000 ---------------------
    type log.00000000.xml | findstr /c:"t:Line"
    type log.00000002.xml | findstr /c:"t:Line"

test1d: all
    -del log.*.xml 2>nul
    @echo -------- Logging output to log ------------
    -rmdir /q /s obj 2>nul
    windbg -g -G -o $(BIND)\tracebld.exe /o:log nmake.exe test2
    @echo -------- Log from log.00000000 ---------------------
    type log.00000000.xml | findstr /c:"t:Line"
    type log.00000002.xml | findstr /c:"t:Line"


test2: all
    -del foo.txt
    echo foo1 >> foo.txt
    echo foo2 >> foo.txt
    echo foo3 >> foo.txt
    -del log.foo.xml
    -mkdir obj
    cmd.exe /c "set Foo=BAR&&cmd.exe /c echo. FOO=%Foo%"
    cl /LD $(CFLAGS) /Tp<< @<<obj\response.txt > log.foo.xml
#include <windows.h>
<<
/Feobj\tbtest.exe /Fd$(@R).pdb $(DET_SRC)
/link $(LINKFLAGS) /subsystem:console
/export:DetourFinishHelperProcess,@1 
<<NOKEEP
    type $(BIND)\trcbld.cpp | findstr CreateFile
    echo int main() { return 0; } > obj\test.cpp
    cl /c /Foobj\test.obj obj\test.cpp
    -del tbtest.* 2>nul

test3: all
    -del log.*.xml 2>nul
    @echo -------- Logging output to log ------------
    -rmdir /q /s foo
    $(BIND)\tracebld.exe /o:log xcopy.exe $(BIND)\trcbld.cpp foo\ /y
    @echo -------- Log from log.00000000 ---------------------
    type log.00000000.xml

test4: all
    -del log.*.xml 2>nul
    echo int main() { return 0; } > obj\test.cpp
    $(BIND)\tracebld.exe /o:log cl /c /Foobj\test.obj obj\test.cpp
    @echo -------- Log from log.00000000 ---------------------
    type log.00000000.xml

################################################################# End of File.
