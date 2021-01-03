//////////////////////////////////////////////////////////////////////////////
//
//  Detours Test Program (syelogd.cpp of syelogd.exe)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
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

#if (_MSC_VER < 1299)
typedef ULONG * PULONG_PTR;
typedef ULONG ULONG_PTR;
typedef LONG * PLONG_PTR;
typedef LONG LONG_PTR;
#endif

enum {
    CLIENT_AWAITING_PIPE_ACCEPT = 0x21,
    CLIENT_AWAITING_PIPE_DATA   = 0x22,
};

typedef struct _CLIENT : OVERLAPPED
{
    HANDLE          hPipe;
    BOOL            fAwaitingAccept;
    PVOID           Zero;
    SYELOG_MESSAGE  Message;
} CLIENT, *PCLIENT;

//////////////////////////////////////////////////////////////////////////////
//
BOOL        s_fLogToScreen  = TRUE;     // Log output to screen.
BOOL        s_fExitAfterOne = FALSE;
BOOL        s_fDeltaTime    = FALSE;
HANDLE      s_hOutFile      = INVALID_HANDLE_VALUE;

LONG        s_nActiveClients = 0;
LONGLONG    s_llStartTime = 0;
LONGLONG    s_llLastTime = 0;

BOOL LogMessageV(BYTE nSeverity, PCHAR pszMsg, ...);

//////////////////////////////////////////////////////////////////////////////
//
VOID MyErrExit(PCSTR pszMsg)
{
    DWORD error = GetLastError();

    LogMessageV(SYELOG_SEVERITY_FATAL, "Error %d in %s.", error, pszMsg);
    fprintf(stderr, "SYELOGD: Error %ld in %s.\n", error, pszMsg);
    fflush(stderr);
    exit(1);
}

//////////////////////////////////////////////////////////////////////////////
//
static PCSTR FileTimeToString(PCHAR pszBuffer, DWORD cbBuffer, FILETIME ftTime)
{
    (void)cbBuffer;

    static BOOL bGotTzi = FALSE;
    static DWORD dwTzi = TIME_ZONE_ID_UNKNOWN;
    static TIME_ZONE_INFORMATION tzi;
    if (!bGotTzi) {
        dwTzi = GetTimeZoneInformation(&tzi);
        if (dwTzi == TIME_ZONE_ID_UNKNOWN) {
            ZeroMemory(&tzi, sizeof(tzi));
        }
        bGotTzi = TRUE;
    }
    SYSTEMTIME stUtc;
    SYSTEMTIME stLocal;

    pszBuffer[0] = '\0';

    if (s_fDeltaTime) {
        if (s_llLastTime == 0) {
            s_llLastTime = s_llStartTime;
        }

        ULARGE_INTEGER ul;
        ul.LowPart = ftTime.dwLowDateTime;
        ul.HighPart = ftTime.dwHighDateTime;

        LONG64 delta = ul.QuadPart - s_llLastTime;
        s_llLastTime = ul.QuadPart;
        delta /= 10000;

        StringCchPrintfA(pszBuffer, cbBuffer, "%7I64d", delta);
    }
    else {
        if (!FileTimeToSystemTime(&ftTime, &stUtc)) {
            StringCchPrintfA(pszBuffer, cbBuffer, "ft:%16I64d", *(LONGLONG *)&ftTime);
            return pszBuffer;
        }
        else if (!SystemTimeToTzSpecificLocalTime(&tzi, &stUtc, &stLocal)) {
            CopyMemory(&stLocal, &stUtc, sizeof(stLocal));
        }

        StringCchPrintfA(pszBuffer, cbBuffer, "%4d%02d%02d%02d%02d%02d%03d",
                         stLocal.wYear,
                         stLocal.wMonth,
                         stLocal.wDay,
                         stLocal.wHour,
                         stLocal.wMinute,
                         stLocal.wSecond,
                         stLocal.wMilliseconds);
    }
    return pszBuffer;
}

BOOL CloseConnection(PCLIENT pClient)
{
    LogMessageV(SYELOG_SEVERITY_INFORMATION, "Client closed pipe.");

    InterlockedDecrement(&s_nActiveClients);
    if (pClient != NULL) {
        if (pClient->hPipe != INVALID_HANDLE_VALUE) {
            FlushFileBuffers(pClient->hPipe);
            if (!DisconnectNamedPipe(pClient->hPipe)) {
                MyErrExit("DisconnectNamedPipe");
            }
            CloseHandle(pClient->hPipe);
            pClient->hPipe = INVALID_HANDLE_VALUE;
        }
        GlobalFree(pClient);
        pClient = NULL;
    }

    if (s_fExitAfterOne) {
        ExitProcess(0);
    }
    return TRUE;
}

// Creates a pipe instance and initiate an accept request.
//
PCLIENT CreatePipeConnection(HANDLE hCompletionPort)
{
    HANDLE hPipe = CreateNamedPipe(SYELOG_PIPE_NAME,           // pipe name
                                   PIPE_ACCESS_INBOUND |       // read-only access
                                   FILE_FLAG_OVERLAPPED,       // overlapped mode
                                   PIPE_TYPE_MESSAGE |         // message-type pipe
                                   PIPE_READMODE_MESSAGE |     // message read mode
                                   PIPE_WAIT,                   // blocking mode
                                   PIPE_UNLIMITED_INSTANCES,   // unlimited instances
                                   0,                          // output buffer size
                                   0,                          // input buffer size
                                   20000,                      // client time-out
                                   NULL);                      // no security attributes
    if (hPipe == INVALID_HANDLE_VALUE) {
        MyErrExit("CreatePipe");
    }

    // Allocate the client data structure.
    //
    PCLIENT pClient = (PCLIENT) GlobalAlloc(GPTR, sizeof(CLIENT));
    if (pClient == NULL) {
        MyErrExit("GlobalAlloc pClient");
    }

    ZeroMemory(pClient, sizeof(*pClient));
    pClient->hPipe = hPipe;
    pClient->fAwaitingAccept = TRUE;

    // Associate file with our complietion port.
    //
    if (!CreateIoCompletionPort(pClient->hPipe, hCompletionPort, (ULONG_PTR)pClient, 0)) {
        MyErrExit("CreateIoComplietionPort pClient");
    }

    if (!ConnectNamedPipe(hPipe, pClient)) {
        if (GetLastError() != ERROR_IO_PENDING &&
            GetLastError() != ERROR_PIPE_LISTENING) {
            MyErrExit("ConnectNamedPipe");
        }
    }
    else {
        LogMessageV(SYELOG_SEVERITY_INFORMATION,
                    "ConnectNamedPipe accepted immediately.");
    }
    return pClient;
}

BOOL LogMessageV(BYTE nSeverity, PCHAR pszMsg, ...)
{
    FILETIME ftOccurance;
    CHAR szTime[64];
    GetSystemTimeAsFileTime(&ftOccurance);
    FileTimeToString(szTime, sizeof(szTime), ftOccurance);

    if (s_fLogToScreen) {
        printf(s_fDeltaTime
               ? "%-7.7s ---- --.%02x: "
               : "%-17.17s ---- --.%02x: "
               , szTime, nSeverity);
        va_list args;
        va_start(args, pszMsg);
        vprintf(pszMsg, args);
        va_end(args);
        printf("\n");
    }
    if (s_hOutFile != INVALID_HANDLE_VALUE) {
        DWORD cbWritten = 0;
        CHAR szBuf[4096] = "";
        PCHAR pcchEnd = szBuf + ARRAYSIZE(szBuf) - 2;
        PCHAR pcchCur = szBuf;
        HRESULT hr;

        hr = StringCchPrintfExA(pcchCur, pcchEnd - pcchCur,
                                &pcchCur, NULL, STRSAFE_NULL_ON_FAILURE,
                                s_fDeltaTime
                                ? "%-7.7s ---- --.%02x: "
                                : "%-17.17s ---- --.%02x: "
                                , szTime, nSeverity);
        if (FAILED(hr)) {
            goto Cleanup;
        }

        va_list args;
        va_start(args, pszMsg);
        hr = StringCchPrintfExA(pcchCur, pcchEnd - pcchCur,
                                &pcchCur, NULL, STRSAFE_NULL_ON_FAILURE,
                                pszMsg, args);
        va_end(args);
        if (FAILED(hr)) {
            goto Cleanup;
        }

        hr = StringCchPrintfExA(pcchCur, (szBuf + ARRAYSIZE(szBuf)) - pcchCur,
                                &pcchCur, NULL, STRSAFE_NULL_ON_FAILURE,
                                 "\n");
        if (FAILED(hr)) {
            goto Cleanup;
        }

      Cleanup:
        WriteFile(s_hOutFile, szBuf, (DWORD)(pcchCur - szBuf), &cbWritten, NULL);
    }
    return TRUE;
}

BOOL LogMessage(PSYELOG_MESSAGE pMessage, DWORD nBytes)
{
    // Sanity check the size of the message.
    //
    if (nBytes > pMessage->nBytes) {
        nBytes = pMessage->nBytes;
    }
    if (nBytes >= sizeof(*pMessage)) {
        nBytes = sizeof(*pMessage) - 1;
    }

    // Don't log message if there isn't and message text.
    //
    if (nBytes <= offsetof(SYELOG_MESSAGE, szMessage)) {
        return FALSE;
    }

    CHAR szTime[64];
    FileTimeToString(szTime, sizeof(szTime), pMessage->ftOccurance);

    PCHAR pszMsg = pMessage->szMessage;
    while (*pszMsg) {
        pszMsg++;
    }
    while (pszMsg > pMessage->szMessage && isspace(pszMsg[-1])) {
        *--pszMsg = '\0';
    }

    if (s_fLogToScreen) {
        printf(s_fDeltaTime
               ? "%-7.7s %4d %02x.%02x: %s\n"
               : "%-17.17s %4d %02x.%02x: %s\n",
               szTime,
               pMessage->nProcessId,
               pMessage->nFacility,
               pMessage->nSeverity,
               pMessage->szMessage);
    }
    if (s_hOutFile != INVALID_HANDLE_VALUE) {
        DWORD cbWritten = 0;
        CHAR szBuf[4096];
        PCHAR pcchEnd = szBuf + ARRAYSIZE(szBuf);
        PCHAR pcchCur = szBuf;
        HRESULT hr;

        hr = StringCchPrintfExA(pcchCur, pcchEnd - pcchCur,
                                &pcchCur, NULL, STRSAFE_NULL_ON_FAILURE,
                                s_fDeltaTime
                                ? "%-7.7s %4d %02x.%02x: %s\n"
                                : "%-17.17s %4d %02x.%02x: %s\n",
                                szTime,
                                pMessage->nProcessId,
                                pMessage->nFacility,
                                pMessage->nSeverity,
                                pMessage->szMessage);
        if (FAILED(hr)) {
            goto Cleanup;
        }

      Cleanup:
        WriteFile(s_hOutFile, szBuf, (DWORD)(pcchCur - szBuf), &cbWritten, NULL);
    }
    return TRUE;
}

DWORD WINAPI WorkerThread(LPVOID pvVoid)
{
    PCLIENT pClient;
    BOOL b;
    LPOVERLAPPED lpo;
    DWORD nBytes;
    HANDLE hCompletionPort = (HANDLE)pvVoid;

    for (BOOL fKeepLooping = TRUE; fKeepLooping;) {
        pClient = NULL;
        lpo = NULL;
        nBytes = 0;
        b = GetQueuedCompletionStatus(hCompletionPort,
                                      &nBytes, (PULONG_PTR)&pClient, &lpo, INFINITE);

        if (!b || lpo == NULL) {
            fKeepLooping = FALSE;
            MyErrExit("GetQueuedCompletionState");
            break;
        }
        else if (!b) {
            if (pClient) {
                if (GetLastError() == ERROR_BROKEN_PIPE) {
                    LogMessageV(SYELOG_SEVERITY_INFORMATION, "Client closed pipe.");
                }
                else {
                    LogMessageV(SYELOG_SEVERITY_ERROR,
                                "GetQueuedCompletionStatus failed %d [%p]",
                                GetLastError(), pClient);
                }
                CloseConnection(pClient);
            }
            continue;
        }

        if (pClient->fAwaitingAccept) {
            InterlockedIncrement(&s_nActiveClients);
            pClient->fAwaitingAccept = FALSE;
            b = ReadFile(pClient->hPipe,
                         &pClient->Message,
                         sizeof(pClient->Message),
                         &nBytes,
                         pClient);
            if (!b) {
                if (GetLastError() != ERROR_IO_PENDING) {
                    LogMessageV(SYELOG_SEVERITY_ERROR,
                                "ReadFile failed %d.", GetLastError());
                    continue;
                }
            }

            CreatePipeConnection(hCompletionPort);
        }
        else {
            if (nBytes < offsetof(SYELOG_MESSAGE, szMessage)) {
                CloseConnection(pClient);
            }

            if (pClient->Message.fTerminate) {
                LogMessageV(SYELOG_SEVERITY_NOTICE,
                            "Client requested terminate on next connection close.");
                s_fExitAfterOne = TRUE;
            }

            LogMessage(&pClient->Message, nBytes);

            b = ReadFile(pClient->hPipe,
                         &pClient->Message,
                         sizeof(pClient->Message),
                         &nBytes,
                         pClient);
            if (!b && GetLastError() == ERROR_BROKEN_PIPE) {
                CloseConnection(pClient);
            }
        }
    }
    return 0;
}

BOOL CreateWorkers(HANDLE hCompletionPort)
{
    DWORD dwThread;
    HANDLE hThread;
    DWORD i;
    SYSTEM_INFO SystemInfo;

    GetSystemInfo(&SystemInfo);

    for (i = 0; i < 2 * SystemInfo.dwNumberOfProcessors; i++) {
        hThread = CreateThread(NULL, 0, WorkerThread, hCompletionPort, 0, &dwThread);
        if (!hThread) {
            MyErrExit("CreateThread WorkerThread");
            // Unreachable: return FALSE;
        }
        CloseHandle(hThread);
    }
    return TRUE;
}

//////////////////////////////////////////////////////////////////////////////
//
BOOL WINAPI ControlHandler(DWORD dwCtrlType)
{
    switch (dwCtrlType) {
      case CTRL_C_EVENT:
      case CTRL_BREAK_EVENT:
      case CTRL_CLOSE_EVENT:
      case CTRL_LOGOFF_EVENT:
      case CTRL_SHUTDOWN_EVENT:
        LogMessageV(SYELOG_SEVERITY_INFORMATION, "User requested stop.");
        printf("\nSYELOGD: Closing connections.\n");
        if (s_hOutFile != INVALID_HANDLE_VALUE) {
            printf("Closing file.\n");
            FlushFileBuffers(s_hOutFile);
            CloseHandle(s_hOutFile);
            s_hOutFile = INVALID_HANDLE_VALUE;
        }
        ExitProcess(0);
    }
    return FALSE;
}

DWORD main(int argc, char **argv)
{
    HANDLE hCompletionPort;
    BOOL fNeedHelp = FALSE;

    GetSystemTimeAsFileTime((FILETIME *)&s_llStartTime);
    SetConsoleCtrlHandler(ControlHandler, TRUE);

    int arg = 1;
    for (; arg < argc; arg++) {
        if (argv[arg][0] == '-' || argv[arg][0] == '/') {
            CHAR *argn = argv[arg] + 1;
            CHAR *argp = argn;
            while (*argp && *argp != ':') {
                argp++;
            }
            if (*argp == ':') {
                *argp++ = '\0';
            }

            switch (argn[0]) {

              case 'd':                                 // Delta time.
              case 'D':
                s_fDeltaTime = TRUE;
                break;

              case 'o':                                 // Only one.
              case 'O':
                s_fExitAfterOne = TRUE;
                break;

              case 'q':                                 // Quiet.
              case 'Q':
                s_fLogToScreen = FALSE;
                break;

              case '?':                                 // Help.
                fNeedHelp = TRUE;
                break;

              default:
                fNeedHelp = TRUE;
                printf("SYELOGD: Bad argument: %s:%s\n", argn, argp);
                break;
            }
        }
        else {
            if (s_hOutFile != INVALID_HANDLE_VALUE) {
                printf("SYELOGD: Error, more than one output file specified.\n\n");
                fNeedHelp = TRUE;
                break;
            }

            s_hOutFile = CreateFileA(argv[arg],
                                     GENERIC_WRITE,
                                     FILE_SHARE_READ,
                                     NULL,
                                     CREATE_ALWAYS,
                                     FILE_ATTRIBUTE_NORMAL |
                                     FILE_FLAG_SEQUENTIAL_SCAN,
                                     NULL);
            if (s_hOutFile == INVALID_HANDLE_VALUE) {
                printf("SYELOGD: Error opening output file: %s: %ld\n\n",
                       argv[arg], GetLastError());
                fNeedHelp = TRUE;
                break;
            }
            else {
                printf("SYELOGD: Logging to %s.\n", argv[arg]);
            }
        }
    }
    if (fNeedHelp) {
        printf("Usage:\n"
               "    syelogd [options] {output_file}\n"
               "Options:\n"
               "    /d         List delta time in ms from previous event (not absolute time).\n"
               "    /o         Exit after one client disconnects.\n"
               "    /q         Disable event logging to screen (quiet mode).\n"
               "    /?         Display this help message.\n"
               "Summary:\n"
               "    If given, all events will be logged to the output file.\n"
               "\n");
        exit(1);
    }


    // Create the completion port.
    hCompletionPort = CreateIoCompletionPort(INVALID_HANDLE_VALUE, NULL, NULL, 0);
    if (hCompletionPort == NULL) {
        MyErrExit("CreateIoCompletionPort");
    }

    // Create completion port worker threads.
    //
    CreateWorkers(hCompletionPort);
    CreatePipeConnection(hCompletionPort);

    printf("SYELOGD: Ready for clients.  Press Ctrl-C to stop.\n");
    while (argc) {
        Sleep(10000);
    }

    SetConsoleCtrlHandler(ControlHandler, FALSE);

    if (s_hOutFile != INVALID_HANDLE_VALUE) {
        FlushFileBuffers(s_hOutFile);
        CloseHandle(s_hOutFile);
        s_hOutFile = INVALID_HANDLE_VALUE;
    }

    return 0;
}
//
//////////////////////////////////////////////////////////////////////////////
