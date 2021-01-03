//////////////////////////////////////////////////////////////////////////////
//
//  Detour Test Program (sleepnew.cpp of sleepnew.exe)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//

#include <windows.h>
#include <stdio.h>
#include "slept.h"

#include "verify.cpp"

int __cdecl main(void)
{
    printf("sleepnew.exe: Starting.\n");
    Verify("SleepEx", (PBYTE)SleepEx);
    printf("\n");
    fflush(stdout);

    printf("sleepnew.exe: Calling Sleep for 1 second.\n");
    Sleep(1000);
    printf("sleepnew.exe: Calling SleepEx for 1 second.\n");
    SleepEx(1000, true);
    printf("sleepnew.exe: Calling Sleep again for 1 second.\n");
    Sleep(1000);
    printf("sleepnew.exe: Calling TimedSleep for 1 second.\n");
    TimedSleepEx(1000, FALSE);
    printf("sleepnew.exe: Calling UntimedSleep for 1 second.\n");
    UntimedSleepEx(1000, FALSE);
    printf("sleepnew.exe: Done sleeping.\n\n");

#if 0
    // This code enumerates the virtual address space and attempts to reserve
    // all unused space below 8GB.
    //
    for (PBYTE pbTry = (PBYTE)0x10000; pbTry < (PBYTE)0x200000000;) {
        MEMORY_BASIC_INFORMATION mbi;

        if (!VirtualQuery(pbTry, &mbi, sizeof(mbi))) {
            break;
        }

        if (mbi.State == MEM_FREE && mbi.RegionSize > 0x10000) {
            PBYTE pbBase = (PBYTE)((((ULONG_PTR)pbTry) + 0xffff) & 0xffffffffffff0000);
            SIZE_T cbTry = mbi.RegionSize & 0xffffffffffff0000;
            if (cbTry > 0x40000000) {
                cbTry = 0x40000000;
            }
            PVOID pvRegion = VirtualAlloc(pbBase, cbTry,
                                          MEM_RESERVE,
                                          PAGE_NOACCESS);
            if (pvRegion == NULL) {
                printf("---%p..%p failed.\n", pbBase, mbi.RegionSize - 0x10000);
            }
            else {
                continue;
            }
        }

        printf("   %p..%p %6x [%p]\n",
               mbi.BaseAddress, (PBYTE)mbi.BaseAddress + mbi.RegionSize - 1,
               mbi.State,
               pbTry);

        pbTry = (PBYTE)mbi.BaseAddress + mbi.RegionSize;
    }
#endif

    printf("sleepnew.exe: GetSleptTicks() = %ld\n\n", GetSleptTicks());
    return 0;
}
//
///////////////////////////////////////////////////////////////// End of File.
