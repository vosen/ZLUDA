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
#include <stdlib.h>
#pragma warning(push)
#if _MSC_VER > 1400
#pragma warning(disable:6102 6103) // /analyze warnings
#endif
#include <strsafe.h>
#pragma warning(pop)
#include <detours.h>

int __cdecl main(int argc, char **argv)
{
    STARTUPINFOA si;
    PROCESS_INFORMATION pi;
    CHAR szFullExe[MAX_PATH];
    CHAR szCommand[MAX_PATH];
    PCHAR pszFileExe;
    PCHAR pszExe;

    ZeroMemory(&si, sizeof(si));
    ZeroMemory(&pi, sizeof(pi));
    si.cb = sizeof(si);


    if (argc != 2) {
        printf("size" DETOURS_STRINGIFY(DETOURS_BITS) ".exe:"
               " must take a single integer argument.\n");
        fflush(stdout);
        return 3;
    }

    int repeats = atoi(argv[1]);

    if (repeats <= 0) {
        printf("size" DETOURS_STRINGIFY(DETOURS_BITS) ".exe:"
               " End of the road, repeats=0.\n");
        fflush(stdout);
        return 0;
    }

    if ((repeats % 2) == 0) {
#ifdef DETOURS_OPTION_BITS
        pszExe = "size" DETOURS_STRINGIFY(DETOURS_OPTION_BITS) ".exe";
#else
        pszExe = "size" DETOURS_STRINGIFY(DETOURS_BITS) ".exe";
#endif
    }
    else {
        pszExe = "size" DETOURS_STRINGIFY(DETOURS_BITS) ".exe";
    }

    if (!SearchPathA(NULL, pszExe, ".exe", ARRAYSIZE(szFullExe), szFullExe, &pszFileExe)) {
        pszExe = "size" DETOURS_STRINGIFY(DETOURS_BITS) ".exe";
        SearchPathA(NULL, pszExe, ".exe", ARRAYSIZE(szFullExe), szFullExe, &pszFileExe);
    }

    StringCchPrintfA(szCommand, sizeof(szCommand), "%s %d", pszExe, repeats - 1);

    printf("size" DETOURS_STRINGIFY(DETOURS_BITS) ".exe:"
           " [%s]\n", szCommand);
    fflush(stdout);

    SetLastError(0);
    if (!CreateProcessA(szFullExe[0] ? szFullExe : NULL, szCommand,
                        NULL, NULL, TRUE, 0, NULL, NULL, &si, &pi)) {
        DWORD dwError = GetLastError();
        printf("size" DETOURS_STRINGIFY(DETOURS_BITS) ".exe:"
               " CreateProcess failed: %ld\n", dwError);
        return 1;
    }

    WaitForSingleObject(pi.hProcess, INFINITE);

    DWORD dwResult = 0;
    if (!GetExitCodeProcess(pi.hProcess, &dwResult)) {
        printf("size" DETOURS_STRINGIFY(DETOURS_BITS) ".exe:"
               " GetExitCodeProcess failed: %ld\n", GetLastError());
        return 9010;
    }

    return 0;
}
//
///////////////////////////////////////////////////////////////// End of File.
