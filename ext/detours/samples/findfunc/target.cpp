//////////////////////////////////////////////////////////////////////////////
//
//  Detour Test Program (target.cpp of target.dll)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//

#include <stdio.h>
#include <windows.h>
#include "target.h"

extern "C" DWORD WINAPI Hidden(DWORD dwCount)
{
    printf("target.dll:     Hidden(%ld) -> %ld.\n", dwCount, dwCount + 1);
    return dwCount + 1;
}

// We use this point to ensure Hidden isn't inlined.
static DWORD (WINAPI * SelfHidden)(DWORD dwCount) = Hidden;

DWORD WINAPI Target(DWORD dwCount)
{
    printf("target.dll:   Target  (%ld) -> %ld.\n", dwCount, dwCount + 100);
    dwCount = SelfHidden(dwCount + 100);
    printf("target.dll:   Target  (.....) -> %ld.\n", dwCount);
    return dwCount;
}

BOOL WINAPI DllMain(HINSTANCE hinst, DWORD dwReason, LPVOID reserved)
{
    (void)hinst;
    (void)dwReason;
    (void)reserved;

    return TRUE;
}

//
///////////////////////////////////////////////////////////////// End of File.
