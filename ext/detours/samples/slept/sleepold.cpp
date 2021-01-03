//////////////////////////////////////////////////////////////////////////////
//
//  Detour Test Program (sleepold.cpp of sleepold.exe)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//

#include <windows.h>
#include <stdio.h>

#include "verify.cpp"

int __cdecl main(int argc, char **argv)
{
    BOOL fQuiet = FALSE;

    if (argc == 2 && _stricmp(argv[1], "-quiet") == 0) {
        fQuiet = TRUE;
    }

    //
    // Verify what the code looks like.
    //
    printf("sleepold.exe: Starting (at %p).\n", main);
    if (!fQuiet) {
        Verify("SleepEx", (PBYTE)SleepEx);
        printf("\n");
    }
    fflush(stdout);

    //
    // See if another process wants us to wait on a shared event.
    // This helps in testing loading a DLL into a new process.

    if (argc == 2 && _stricmp(argv[1], "-wait") == 0) {
        HANDLE hEvent = OpenEventA(SYNCHRONIZE, FALSE, "detours_load_test_event");
        if (hEvent) {
            printf("sleepold.exe: Waiting for detours_load_test_event to be set.\n");
            fflush(stdout);
            WaitForSingleObject(hEvent, INFINITE);
        }
        else {
            printf("sleepold.exe: Couldn't open detours_load_test_event.\n");
        }
    }

    //
    // Try out sleep (which may be detours).
    //
    printf("sleepold.exe: Calling Sleep for 1 second.\n");
    Sleep(1000);

    printf("sleepold.exe: Calling SleepEx for 1 second.\n");
    SleepEx(1000, false);

    printf("sleepold.exe: Calling Sleep again for 1 second.\n");
    Sleep(1000);

    // DebugBreak();

    printf("sleepold.exe: Done sleeping.\n\n");
    fflush(stdout);

    return 0;
}
//
///////////////////////////////////////////////////////////////// End of File.
