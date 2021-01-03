//////////////////////////////////////////////////////////////////////////////
//
//  Detours Test Program (sltestp.cpp of sltestp.exe)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
//  Test the named-pipe-based connection to the syelog system-event logger.
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

VOID MyErrExit(PCSTR pszMsg)
{
    fprintf(stderr, "Error %s: %ld\n", pszMsg, GetLastError());
    exit(1);
}

DWORD main(int argc, char *argv[])
{
    HANDLE hPipe;
    SYELOG_MESSAGE Message;
    BOOL fSuccess;
    DWORD cbWritten, dwMode;

    // Try to open a named pipe; wait for it, if necessary.

    TIME_ZONE_INFORMATION tzi;
    GetTimeZoneInformation(&tzi);

    for (;;) {
        hPipe = CreateFileW(SYELOG_PIPE_NAMEW,  // pipe name
                            GENERIC_WRITE,      // write access only
                            0,                  // no sharing
                            NULL,               // no security attributes
                            OPEN_EXISTING,      // opens existing pipe
                            0,                  // default attributes
                            NULL);              // no template file

        // Break if the pipe handle is valid.
         if (hPipe != INVALID_HANDLE_VALUE)
            break;

        // Exit if an error other than ERROR_PIPE_BUSY occurs.

        if (GetLastError() != ERROR_PIPE_BUSY)
            MyErrExit("Could not open pipe");

        // All pipe instances are busy, so wait for 1 seconds.

        if (!WaitNamedPipeW(SYELOG_PIPE_NAMEW, 1000))
            MyErrExit("Could not open pipe");
    }

    // The pipe connected; change to message-read mode.
    dwMode = PIPE_READMODE_MESSAGE;
    fSuccess = SetNamedPipeHandleState(hPipe,    // pipe handle
                                       &dwMode,  // new pipe mode
                                       NULL,     // don't set maximum bytes
                                       NULL);    // don't set maximum time
    if (!fSuccess)
        MyErrExit("SetNamedPipeHandleState");

    // Send a message to the pipe server.

    memset(&Message, 0, sizeof(Message));

    StringCchCopyA(Message.szMessage, ARRAYSIZE(Message.szMessage),
                   (argc > 1) ? argv[1] : "sltestp: hello world!");

    Message.nFacility = SYELOG_FACILITY_APPLICATION;
    Message.nSeverity = SYELOG_SEVERITY_INFORMATION;
    Message.nProcessId = GetCurrentProcessId();
    GetSystemTimeAsFileTime(&Message.ftOccurance);
    PCSTR pszEnd = Message.szMessage;
    for (; *pszEnd; pszEnd++) {
        // no internal contents.
    }
    Message.nBytes = (USHORT)(pszEnd - ((PCSTR)&Message) + 1);

    fSuccess = WriteFile(hPipe,                  // pipe handle
                         &Message,             // message
                         Message.nBytes, // message length
                         &cbWritten,             // bytes written
                         NULL);                  // not overlapped
    if (! fSuccess)
        MyErrExit("WriteFile");

    CloseHandle(hPipe);

    GetTimeZoneInformation(&tzi);

    return 0;
}
