//////////////////////////////////////////////////////////////////////////////
//
//  Detours Test Program (sltest.cpp of sltest.exe)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
//  Test the named-pipe-based connection with syelog.lib to the syelog
//  system-event logger.
//
#include <windows.h>
#include <stdio.h>
#include <stdlib.h>
#include <stddef.h>
#pragma warning(push)
#if _MSC_VER > 1400
#pragma warning(disable:6102 6103) // /analyze warnings
#endif
#include <strsafe.h>
#pragma warning(pop)
#include "syelog.h"
#include "detours.h"

extern "C" {

    HANDLE ( WINAPI *
             Real_CreateFileW)(LPCWSTR a0,
                               DWORD a1,
                               DWORD a2,
                               LPSECURITY_ATTRIBUTES a3,
                               DWORD a4,
                               DWORD a5,
                               HANDLE a6)
        = CreateFileW;

    BOOL ( WINAPI *
           Real_WriteFile)(HANDLE hFile,
                           LPCVOID lpBuffer,
                           DWORD nNumberOfBytesToWrite,
                           LPDWORD lpNumberOfBytesWritten,
                           LPOVERLAPPED lpOverlapped)
        = WriteFile;
    BOOL ( WINAPI *
           Real_FlushFileBuffers)(HANDLE hFile)
        = FlushFileBuffers;
    BOOL ( WINAPI *
           Real_CloseHandle)(HANDLE hObject)
        = CloseHandle;

    BOOL ( WINAPI *
           Real_WaitNamedPipeW)(LPCWSTR lpNamedPipeName, DWORD nTimeOut)
        = WaitNamedPipeW;
    BOOL ( WINAPI *
           Real_SetNamedPipeHandleState)(HANDLE hNamedPipe,
                                         LPDWORD lpMode,
                                         LPDWORD lpMaxCollectionCount,
                                         LPDWORD lpCollectDataTimeout)
        = SetNamedPipeHandleState;

    DWORD ( WINAPI *
            Real_GetCurrentProcessId)(VOID)
        = GetCurrentProcessId;
    VOID ( WINAPI *
           Real_GetSystemTimeAsFileTime)(LPFILETIME lpSystemTimeAsFileTime)
        = GetSystemTimeAsFileTime;

    VOID ( WINAPI *
           Real_InitializeCriticalSection)(LPCRITICAL_SECTION lpSection)
        = InitializeCriticalSection;
    VOID ( WINAPI *
           Real_EnterCriticalSection)(LPCRITICAL_SECTION lpSection)
        = EnterCriticalSection;
    VOID ( WINAPI *
           Real_LeaveCriticalSection)(LPCRITICAL_SECTION lpSection)
        = LeaveCriticalSection;
}

int main(int argc, char **argv)
{
    BOOL fNeedHelp = FALSE;
    BOOL fRequestExitOnClose = FALSE;

    int arg = 1;
    for (; arg < argc && (argv[arg][0] == '-' || argv[arg][0] == '/'); arg++) {
        CHAR *argn = argv[arg] + 1;
        CHAR *argp = argn;
        while (*argp && *argp != ':') {
            argp++;
        }
        if (*argp == ':') {
            *argp++ = '\0';
        }

        switch (argn[0]) {

          case 'x':                                 // Request exit on close.
          case 'X':
            fRequestExitOnClose = TRUE;
            break;

          case '?':                                 // Help.
            fNeedHelp = TRUE;
            break;

          default:
            fNeedHelp = TRUE;
            printf("SLTEST: Bad argument: %s:%s\n", argn, argp);
            break;
        }
    }

    if (fNeedHelp) {
        printf("Usage:\n"
               "    sltest.exe [options] message\n"
               "Options:\n"
               "    /x         Ask syelogd.exe to terminate when this connect closes.\n"
               "    /?         Display this help message.\n"
               "\n");
        exit(1);
    }

    SyelogOpen("sltest", SYELOG_FACILITY_APPLICATION);
    if (arg >= argc) {
        Syelog(SYELOG_SEVERITY_INFORMATION, "Hello World! [1 of 4]");
        Syelog(SYELOG_SEVERITY_INFORMATION, "Hello World! [2 of 4]");
        Syelog(SYELOG_SEVERITY_INFORMATION, "Hello World! [3 of 4]");
        Syelog(SYELOG_SEVERITY_INFORMATION, "Hello World! [4 of 4]");
    }
    else {
        CHAR Buffer[1024] = "";

        for (; arg < argc; arg++) {
            StringCchCatA(Buffer, ARRAYSIZE(Buffer), argv[arg]);
            if (arg + 1 < argc) {
                StringCchCatA(Buffer, ARRAYSIZE(Buffer), " ");
            }
        }
        Syelog(SYELOG_SEVERITY_INFORMATION, Buffer);
    }

    SyelogClose(fRequestExitOnClose);

    return 0;
}
