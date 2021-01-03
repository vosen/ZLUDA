//////////////////////////////////////////////////////////////////////////////
//
//  Detour Test Program (dslept.cpp of dslept.dll)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
//  An example dynamically detouring a function.
//
#include <stdio.h>
#include <windows.h>
#include "detours.h"
#include "slept.h"

#include "verify.cpp"

LONG dwSlept = 0;

static DWORD (WINAPI * TrueSleepEx)(DWORD dwMilliseconds, BOOL bAlertable) = NULL;
static int (WINAPI * TrueEntryPoint)(VOID) = NULL;
static int (WINAPI * RawEntryPoint)(VOID) = NULL;

DWORD WINAPI UntimedSleepEx(DWORD dwMilliseconds, BOOL bAlertable)
{
    if (TrueSleepEx != NULL) {
        return TrueSleepEx(dwMilliseconds, bAlertable);
    }
    return 0;
}

DWORD WINAPI TimedSleepEx(DWORD dwMilliseconds, BOOL bAlertable)
{
    DWORD dwBeg = GetTickCount();
    DWORD ret = TrueSleepEx(dwMilliseconds, bAlertable);
    DWORD dwEnd = GetTickCount();

    InterlockedExchangeAdd(&dwSlept, dwEnd - dwBeg);
    return ret;
}

DWORD WINAPI GetSleptTicks(VOID)
{
    return dwSlept;
}

int WINAPI TimedEntryPoint(VOID)
{
    // We couldn't call LoadLibrary in DllMain,
    // so we detour SleepEx here...
    LONG error;

    TrueSleepEx = (DWORD (WINAPI *)(DWORD, BOOL))
        DetourFindFunction("kernel32.dll", "SleepEx");

    DetourTransactionBegin();
    DetourUpdateThread(GetCurrentThread());
    DetourAttach(&(PVOID&)TrueSleepEx, TimedSleepEx);
    error = DetourTransactionCommit();

    if (error == NO_ERROR) {
        printf("dslept" DETOURS_STRINGIFY(DETOURS_BITS) ".dll: "
               " Detoured SleepEx().\n");

    }
    else {
        printf("dslept" DETOURS_STRINGIFY(DETOURS_BITS) ".dll: "
               " Error detouring SleepEx(): %ld\n", error);
    }

    Verify("SleepEx", (PVOID)SleepEx);
    printf("\n");
    fflush(stdout);

    printf("dslept" DETOURS_STRINGIFY(DETOURS_BITS) ".dll: "
           " Calling EntryPoint\n");
    fflush(stdout);

    return TrueEntryPoint();
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

        printf("dslept" DETOURS_STRINGIFY(DETOURS_BITS) ".dll: "
               " Starting.\n");
        Verify("SleepEx", (PVOID)SleepEx);
        printf("\n");
        fflush(stdout);

        // NB: DllMain can't call LoadLibrary, so we hook the app entry point.
        TrueEntryPoint = (int (WINAPI *)(VOID))DetourGetEntryPoint(NULL);
        RawEntryPoint = TrueEntryPoint;

        Verify("EntryPoint", RawEntryPoint);

        DetourTransactionBegin();
        DetourUpdateThread(GetCurrentThread());
        DetourAttach(&(PVOID&)TrueEntryPoint, TimedEntryPoint);
        error = DetourTransactionCommit();

        Verify("EntryPoint after attach", RawEntryPoint);
        Verify("EntryPoint trampoline", TrueEntryPoint);

        if (error == NO_ERROR) {
            printf("dslept" DETOURS_STRINGIFY(DETOURS_BITS) ".dll: "
                   " Detoured EntryPoint().\n");
        }
        else {
            printf("dslept" DETOURS_STRINGIFY(DETOURS_BITS) ".dll: "
                   " Error detouring EntryPoint(): %ld\n", error);
        }
    }
    else if (dwReason == DLL_PROCESS_DETACH) {
        DetourTransactionBegin();
        DetourUpdateThread(GetCurrentThread());
        if (TrueSleepEx != NULL) {
            DetourDetach(&(PVOID&)TrueSleepEx, (PVOID)TimedSleepEx);
        }
        DetourDetach(&(PVOID&)TrueEntryPoint, TimedEntryPoint);
        error = DetourTransactionCommit();

        printf("dslept" DETOURS_STRINGIFY(DETOURS_BITS) ".dll: "
               " Removed Sleep() detours (%ld), slept %ld ticks.\n", error, dwSlept);

        fflush(stdout);
    }
    return TRUE;
}
//
///////////////////////////////////////////////////////////////// End of File.
