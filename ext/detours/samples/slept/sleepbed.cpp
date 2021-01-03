//////////////////////////////////////////////////////////////////////////////
//
//  Detour Test Program (sleepbed.cpp of sleepbed.exe)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//

#include <windows.h>
#include <stdio.h>

#include "verify.cpp"

static BOOL fBroke = FALSE;
static LONG dwSlept = 0;
static DWORD (WINAPI * TrueSleepEx)(DWORD dwMilliseconds, BOOL bAlertable)
    = SleepEx;

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

//
///////////////////////////////////////////////////////////////// End of File.

int __cdecl main(void)
{
    int error = 0;

    printf("sleepbed.exe: Starting.\n");
    PVOID pbExeEntry = DetourGetEntryPoint(NULL);
    printf("sleepbed.exe: ExeEntry=%p\n", pbExeEntry);

    Verify("SleepEx", (PVOID)SleepEx);
    printf("\n");
    fflush(stdout);

    DetourTransactionBegin();
    DetourUpdateThread(GetCurrentThread());
    DetourAttach(&(PVOID&)TrueSleepEx, TimedSleepEx);
    error = DetourTransactionCommit();

    if (error == NO_ERROR) {
        printf("sleepbed.exe: Detoured SleepEx().\n");
    }
    else {
        printf("sleepbed.exe: Error detouring SleepEx(): %d\n", error);
        return error;
    }
    fflush(stdout);

    printf("sleepbed.exe: After detour.\n");
    Verify("SleepEx", (PBYTE)SleepEx);
    printf("\n");
    fflush(stdout);

    printf("sleepbed.exe: Calling Sleep for 1 second.\n");
    Sleep(1000);
    printf("sleepbed.exe: Calling SleepEx for 1 second.\n");
    SleepEx(1000, true);
    printf("sleepbed.exe: Calling Sleep again for 1 second.\n");
    Sleep(1000);
    printf("sleepbed.exe: Calling TimedSleepEx for 1 second.\n");
    TimedSleepEx(1000, false);
    printf("sleepbed.exe: Calling UntimedSleepEx for 1 second.\n");
    UntimedSleepEx(1000, false);
    printf("sleepbed.exe: Done sleeping.\n\n");

    DetourTransactionBegin();
    DetourUpdateThread(GetCurrentThread());
    DetourDetach(&(PVOID&)TrueSleepEx, TimedSleepEx);
    error = DetourTransactionCommit();
    printf("sleepbed.exe: Removed SleepEx() detour (%d), slept %ld ticks.\n",
           error, dwSlept);
    fflush(stdout);

    printf("sleepbed.exe: GetSleptTicks() = %ld\n\n", GetSleptTicks());
    return error;
}
//
///////////////////////////////////////////////////////////////// End of File.
