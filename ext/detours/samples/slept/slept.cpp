//////////////////////////////////////////////////////////////////////////////
//
//  Detour Test Program (slept.cpp of slept.dll)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
#include <stdio.h>
#include <windows.h>
#include "detours.h"
#include "slept.h"

#include "verify.cpp"

static BOOL fBroke = FALSE;
static LONG dwSlept = 0;
static DWORD (WINAPI * TrueSleepEx)(DWORD dwMilliseconds, BOOL bAlertable) = SleepEx;

DWORD WINAPI UntimedSleepEx(DWORD dwMilliseconds, BOOL bAlertable)
{
    return TrueSleepEx(dwMilliseconds, bAlertable);
}

DWORD WINAPI TimedSleepEx(DWORD dwMilliseconds, BOOL bAlertable)
{
    DWORD dwBeg = GetTickCount();
    DWORD ret = TrueSleepEx(dwMilliseconds, bAlertable);
    DWORD dwEnd = GetTickCount();

    if (!fBroke) {
        fBroke = TRUE;
        // DebugBreak();
    }

    InterlockedExchangeAdd(&dwSlept, dwEnd - dwBeg);
    return ret;
}

DWORD WINAPI GetSleptTicks(VOID)
{
    return dwSlept;
}

DWORD WINAPI TestTicks(VOID)
{
    return TestTicksEx(0);
}

DWORD WINAPI TestTicksEx(DWORD Add)
{
    PDWORD pdw = new DWORD [Add + 1];

    if (pdw != NULL) {
        pdw[0] = dwSlept;
        for (DWORD n = 1; n < Add + 1; n++) {
            pdw[n] = pdw[n-1] + 1;
        }

        for (DWORD n = 1; n < Add + 1; n++) {
            pdw[n-1] = pdw[n-1] - 1;
        }

        for (DWORD n = 1; n < Add + 1; n++) {
            pdw[n] = pdw[n-1] + 1;
        }

        Add = pdw[Add] - Add;

        delete [] pdw;
    }
    else {
        Add = dwSlept + Add;
    }

    return Add;
}

BOOL WINAPI DllMain(HINSTANCE hinst, DWORD dwReason, LPVOID reserved)
{
    LONG error;
    (void)hinst;
    (void)reserved;

    if (DetourIsHelperProcess()) {
        return TRUE;
    }

    if (dwReason == DLL_PROCESS_ATTACH) {
        DetourRestoreAfterWith();

        printf("slept" DETOURS_STRINGIFY(DETOURS_BITS) ".dll: "
               " Starting.\n");
        PVOID pbExeEntry = DetourGetEntryPoint(NULL);
        PVOID pbDllEntry = DetourGetEntryPoint(hinst);
        printf("slept" DETOURS_STRINGIFY(DETOURS_BITS) ".dll: "
               " ExeEntry=%p, DllEntry=%p\n", pbExeEntry, pbDllEntry);

        Verify("SleepEx", (PVOID)SleepEx);
        printf("\n");
        fflush(stdout);

        DetourTransactionBegin();
        DetourUpdateThread(GetCurrentThread());
        DetourAttach(&(PVOID&)TrueSleepEx, TimedSleepEx);
        error = DetourTransactionCommit();

        if (error == NO_ERROR) {
            printf("slept" DETOURS_STRINGIFY(DETOURS_BITS) ".dll: "
                   " Detoured SleepEx() @ %p.\n", TrueSleepEx);
        }
        else {
            printf("slept" DETOURS_STRINGIFY(DETOURS_BITS) ".dll: "
                   " Error detouring SleepEx(): %ld\n", error);
        }
    }
    else if (dwReason == DLL_PROCESS_DETACH) {
        DetourTransactionBegin();
        DetourUpdateThread(GetCurrentThread());
        DetourDetach(&(PVOID&)TrueSleepEx, TimedSleepEx);
        error = DetourTransactionCommit();
        printf("slept" DETOURS_STRINGIFY(DETOURS_BITS) ".dll: "
               " Removed SleepEx() detour (%ld), slept %ld ticks.\n", error, dwSlept);
        fflush(stdout);
    }
    return TRUE;
}

//
///////////////////////////////////////////////////////////////// End of File.
