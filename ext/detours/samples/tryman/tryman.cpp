//////////////////////////////////////////////////////////////////////////////
//
//  Detours Test Program (tryman.cpp of tryman.exe)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//

#include <windows.h>

extern int WINAPI Test3264(int arg);

int __cdecl main(int argc, char ** argv)
{
    (void)argv;
    int ret = 0;

    ret = Test3264(argc);
    return ret == 0 ? ret : 0;
}
//
///////////////////////////////////////////////////////////////// End of File.
