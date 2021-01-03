//////////////////////////////////////////////////////////////////////////////
//
//  Test the different system region bounds (region.cpp of region.exe)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
#include <stdio.h>

#include <windows.h>
#include <detours.h>

//////////////////////////////////////////////////////////////////////////////
//
static DWORD (WINAPI * TrueSleepEx)(DWORD dwMilliseconds, BOOL bAlertable) = SleepEx;

DWORD WINAPI LoudSleepEx(DWORD dwMilliseconds, BOOL bAlertable)
{
    DWORD dwBeg = GetTickCount();
    DWORD ret = TrueSleepEx(dwMilliseconds, bAlertable);
    DWORD dwEnd = GetTickCount();

    printf("Slept %lu ticks.\n", dwEnd - dwBeg);
    return ret;
}

//////////////////////////////////////////////////////////////////////////////
//
PVOID AttachAndDetach(DWORD dwMilliseconds)
{
    LONG error;
    PVOID trampoline;

    DetourTransactionBegin();
    DetourUpdateThread(GetCurrentThread());
    DetourAttach(&(PVOID&)TrueSleepEx, LoudSleepEx);
    error = DetourTransactionCommit();

    printf("Attach: %ld, Trampoline: %p\n", error, TrueSleepEx);

    trampoline = TrueSleepEx;

    printf("\n");
    printf("Sleep(%lu)\n", dwMilliseconds);
    Sleep(dwMilliseconds);
    printf("\n");

    DetourTransactionBegin();
    DetourUpdateThread(GetCurrentThread());
    DetourDetach(&(PVOID&)TrueSleepEx, LoudSleepEx);
    error = DetourTransactionCommit();

    return trampoline;
}

int main(int argc, char **argv)
{
    (void)argc;
    (void)argv;

    // First, save the default system region.

    PVOID pDefaultLower = DetourSetSystemRegionLowerBound(NULL);
    PVOID pDefaultUpper = DetourSetSystemRegionUpperBound(NULL);

    // Now attach the detour with the default system region.

    DetourSetSystemRegionLowerBound(pDefaultLower);
    DetourSetSystemRegionUpperBound(pDefaultUpper);

    printf("%p..%p: ", pDefaultLower, pDefaultUpper);
    PVOID pTramp1 = AttachAndDetach(10);

    printf("%p..%p: ", pDefaultLower, pDefaultUpper);
    PVOID pTramp2 = AttachAndDetach(10);

    // Now attach the detour with a smaller system region.

    PVOID pSmallerLower = (PVOID)( ((ULONG_PTR)pTramp1) & ~(ULONG_PTR)0x3fffffff );
    PVOID pSmallerUpper = (PVOID)( ((ULONG_PTR)pTramp1 + 0x3fffffff) & ~(ULONG_PTR)0x3fffffff );

    DetourSetSystemRegionLowerBound(pSmallerLower);
    DetourSetSystemRegionUpperBound(pSmallerUpper);

    printf("%p..%p: ", pSmallerLower, pSmallerUpper);
    PVOID pTramp3 = AttachAndDetach(20);

    printf("Sleep(30)\n");
    Sleep(30);
    printf("\n");

    if (pTramp1 != pTramp2) {
        printf("!!!!!! Trampoling allocation is not deterministic.  %p != %p\n", pTramp1, pTramp2);
        return 1;
    }
    else if (pTramp2 == pTramp3) {
        printf("!!!!!! Trampoling allocation doesn't skip region.  %p == %p\n", pTramp2, pTramp3);
        return 2;
    }

    return 0;
}

