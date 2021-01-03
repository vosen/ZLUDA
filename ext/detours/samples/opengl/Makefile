######################################################################
##
##  Hook test for glFinish
##

!include ..\common.mak

LIBS=$(LIBS) kernel32.lib gdi32.lib

##############################################################################

all: dirs \
    $(BIND)\ogldet$(DETOURS_BITS).dll \
    $(BIND)\testogl.exe \
    \
!IF $(DETOURS_SOURCE_BROWSING)==1
    $(OBJD)\ogldet$(DETOURS_BITS).bsc \
    $(OBJD)\testogl.bsc \
!ENDIF
    option

##############################################################################

dirs:
    @if not exist $(BIND) mkdir $(BIND) && echo.   Created $(BIND)
    @if not exist $(OBJD) mkdir $(OBJD) && echo.   Created $(OBJD)

$(OBJD)\ogldet.obj : ogldet.cpp

$(OBJD)\ogldet.res : ogldet.rc

$(BIND)\ogldet$(DETOURS_BITS).dll $(BIND)\ogldet$(DETOURS_BITS).lib: \
        $(OBJD)\ogldet.obj $(OBJD)\ogldet.res $(DEPS)
    cl /LD $(CFLAGS) /Fe$(@R).dll /Fd$(@R).pdb \
        $(OBJD)\ogldet.obj $(OBJD)\ogldet.res \
        /link $(LINKFLAGS) /subsystem:console \
        /export:DetourFinishHelperProcess,@1,NONAME \
        /export:hookedGlFinish \
        $(LIBS) opengl32.lib

$(OBJD)\ogldet$(DETOURS_BITS).bsc : $(OBJD)\ogldet.obj
    bscmake /v /n /o $@ $(OBJD)\ogldet.sbr

$(OBJD)\testogl.obj : testogl.cpp

$(BIND)\testogl.exe : $(OBJD)\testogl.obj $(DEPS)
    cl $(CFLAGS) /Fe$@ /Fd$(@R).pdb $(OBJD)\testogl.obj \
        /link $(LINKFLAGS) $(LIBS) opengl32.lib \
        /subsystem:console

$(OBJD)\testogl.bsc : $(OBJD)\testogl.obj
    bscmake /v /n /o $@ $(OBJD)\testogl.sbr

##############################################################################

clean:
    -del *~ 2>nul
    -del $(BIND)\ogldet*.* 2>nul
    -del $(BIND)\testogl.* 2>nul
    -rmdir /q /s $(OBJD) 2>nul

realclean: clean
    -rmdir /q /s $(OBJDS) 2>nul

############################################### Install non-bit-size binaries.

!IF "$(DETOURS_OPTION_PROCESSOR)" != ""

$(OPTD)\olgdet$(DETOURS_OPTION_BITS).dll:
$(OPTD)\olgdet$(DETOURS_OPTION_BITS).pdb:

$(BIND)\olgdet$(DETOURS_OPTION_BITS).dll : $(OPTD)\olgdet$(DETOURS_OPTION_BITS).dll
    @if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).
$(BIND)\olgdet$(DETOURS_OPTION_BITS).pdb : $(OPTD)\olgdet$(DETOURS_OPTION_BITS).pdb
    @if exist $? copy /y $? $(BIND) >nul && echo $@ copied from $(DETOURS_OPTION_PROCESSOR).

option: \
    $(BIND)\olgdet$(DETOURS_OPTION_BITS).dll \
    $(BIND)\olgdet$(DETOURS_OPTION_BITS).pdb \

!ELSE

option:

!ENDIF

##############################################################################

test: all
    @echo -------- Reseting test binaries to initial state. ---------------------
    $(BIND)\setdll.exe -r $(BIND)\testogl.exe
    @echo.
    @echo -------- Should not load ogldet$(DETOURS_BITS).dll -----------------------------------
    $(BIND)\testogl.exe
    @echo.
    @echo -------- Adding ogldet$(DETOURS_BITS).dll to testogl.exe ------------------------------
    $(BIND)\setdll.exe -d:$(BIND)\ogldet$(DETOURS_BITS).dll $(BIND)\testogl.exe
    @echo.
    @echo -------- Should load ogldet$(DETOURS_BITS).dll statically ----------------------------
    $(BIND)\testogl.exe
    @echo.
    @echo -------- Removing ogldet$(DETOURS_BITS).dll from testogl.exe --------------------------
    $(BIND)\setdll.exe -r $(BIND)\testogl.exe
    @echo.
    @echo -------- Should not load ogldet$(DETOURS_BITS).dll -----------------------------------
    $(BIND)\testogl.exe
    @echo.
    @echo -------- Should load ogldet$(DETOURS_BITS).dll dynamically using withdll.exe----------
    $(BIND)\withdll.exe -d:$(BIND)\ogldet$(DETOURS_BITS).dll $(BIND)\testogl.exe
    @echo.

################################################################# End of File.
