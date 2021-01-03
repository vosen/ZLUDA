//////////////////////////////////////////////////////////////////////////////
//
//  File:       testogl.cpp
//  Module:     testogl.exe (oglsimple.dll)
//

#include <windows.h>
#include <stdio.h>
#include <GL/gl.h>

int __cdecl main()
{
    printf("testogl.exe: Starting\n");
    fflush(stdout);

    glFinish();

    printf("testogl.exe: done\n");
    fflush(stdout);

    return 0;
}
//
///////////////////////////////////////////////////////////////// End of File.
