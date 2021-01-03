//////////////////////////////////////////////////////////////////////////////
//
//  First Chance Exception Handling Test Program (excep.cpp of excep.exe)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
//  For more information on exception handling, see "A Crash Course on the
//  Depths of Win32 Structured Exception Handling," by Matt Pietrek in the
//  January 1997 issue of Microsoft Systems Journal.
//
#include <stdio.h>
#include <windows.h>
#include <detours.h>
#include "firstexc.h"

//////////////////////////////////////////////////////////////////////////////
//
static LPVOID   s_pvData = NULL;
static DWORD    s_dwDataPerm = 0;

static LONG ExceptCatch(LONG nTry, DWORD dwException, LPEXCEPTION_POINTERS pinfo)
{
    printf("      ExceptCatch(%ld, %08lx, %08lx)\n", nTry, dwException, (ULONG)pinfo);
#ifdef INCLUDE_THIS
    if (nTry == 0) {
        return EXCEPTION_CONTINUE_EXECUTION;
    }
#endif
    return EXCEPTION_EXECUTE_HANDLER;
}

static int BadCode(int nTry)
{
    printf("    BadCode(Try:%d)\n", nTry);
    printf("      BadCode -> %ld\n", *(PULONG)s_pvData);
    ((PULONG)s_pvData)[0] = 0;
    printf("      BadCode -> %ld\n", *(PULONG)s_pvData);
    ((PULONG)s_pvData)[-1] = 0;
    printf("      BadCode -> %ld\n", *(PULONG)s_pvData);

    return 0;
}

void safe(int nTry)
{
    __try {
        printf("  try(%d)\n", nTry);
        BadCode(nTry);
        printf("  good(%d)\n", nTry);
    } __except(ExceptCatch(nTry,
                           GetExceptionCode(),
                           GetExceptionInformation())) {
        DWORD dwExcept = GetExceptionCode();

        printf("  handler(%d) : %08lx\n", nTry, dwExcept);
    }
}

void raw(int nTry)
{
    BadCode(nTry);
}

LONG WINAPI MyVirtualFaultFilter(PEXCEPTION_POINTERS pException)
{
    PEXCEPTION_RECORD pExceptRec = pException->ExceptionRecord;

    if (pExceptRec->ExceptionCode == 0xc0000005) {
        printf("--        Memory access exception.\n");
        if (pExceptRec->NumberParameters >= 2 &&
            pExceptRec->ExceptionInformation[1] >= (ULONG)s_pvData &&
            pExceptRec->ExceptionInformation[1] <= (ULONG)s_pvData + sizeof(ULONG)) {

            VirtualProtect(s_pvData, sizeof(ULONG), PAGE_READWRITE, &s_dwDataPerm);
            printf("--        Changed permissions.\n");
            return EXCEPTION_CONTINUE_EXECUTION;
        }
    }
    return EXCEPTION_CONTINUE_SEARCH;
}

int WINAPI WinMain(HINSTANCE hinst, HINSTANCE hprev, LPSTR lpszCmdLine, int nCmdShow)
{
    (void)hinst;
    (void)hprev;
    (void)lpszCmdLine;
    (void)nCmdShow;

    s_pvData = VirtualAlloc(NULL, sizeof(ULONG), MEM_RESERVE|MEM_COMMIT, PAGE_READWRITE);
    if (s_pvData == NULL) {
        printf("VirtualAlloc failed: %ld\n", GetLastError());
        return 0;
    }
    *(PULONG)s_pvData = 1;

    VirtualProtect(s_pvData, sizeof(ULONG), PAGE_READONLY, &s_dwDataPerm);

    DetourFirstChanceExceptionFilter(MyVirtualFaultFilter);

    printf("main\n");
    printf("--------------------------------------------------\n");
    int nTry = 0;
    for (; nTry < 1; nTry++) {
        // safe(nTry);
    }
    printf("-- safe ------------------------------------------\n");
    safe(nTry);
    VirtualProtect(s_pvData, sizeof(ULONG), PAGE_READWRITE, &s_dwDataPerm);
    *(PULONG)s_pvData = 1;
    VirtualProtect(s_pvData, sizeof(ULONG), PAGE_READONLY, &s_dwDataPerm);

    printf("-- raw -------------------------------------------\n");
    printf("*\n");
    printf("* NB: The second attempt to write will fail because it isn't handled.\n");
    printf("*\n");
    raw(nTry);
    printf("--------------------------------------------------\n");
    printf("exit\n");

    return 0;
}
//
///////////////////////////////////////////////////////////////// End of File.
