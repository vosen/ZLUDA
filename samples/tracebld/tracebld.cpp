//////////////////////////////////////////////////////////////////////////////
//
//  Detours Test Program (tracebld.cpp of tracebld.exe)
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
#include <detours.h>
#include "tracebld.h"

#if (_MSC_VER < 1299)
typedef ULONG * PULONG_PTR;
typedef ULONG ULONG_PTR;
typedef LONG * PLONG_PTR;
typedef LONG LONG_PTR;
#endif

//////////////////////////////////////////////////////////////////////////////
#pragma warning(disable:4127)   // Many of our asserts are constants.

#define ASSERT_ALWAYS(x)   \
    do {                                                        \
    if (!(x)) {                                                 \
            AssertMessage(#x, __FILE__, __LINE__);              \
            DebugBreak();                                       \
    }                                                           \
    } while (0)

#ifndef NDEBUG
#define ASSERT(x)           ASSERT_ALWAYS(x)
#else
#define ASSERT(x)
#endif

#define UNUSED(c)       (c) = (c)

//////////////////////////////////////////////////////////////////////////////

enum {
    CLIENT_AWAITING_PIPE_ACCEPT = 0x21,
    CLIENT_AWAITING_PIPE_DATA   = 0x22,
};

typedef struct _CLIENT : OVERLAPPED
{
    HANDLE          hPipe;
    LONG            nClient;
    HANDLE          hFile;
    BOOL            fAwaitingAccept;
    PVOID           Zero;
    TBLOG_MESSAGE   Message;

    BOOL LogMessage(PTBLOG_MESSAGE pMessage, DWORD nBytes);
    BOOL LogMessageV(PCHAR pszMsg, ...);
} CLIENT, *PCLIENT;

//////////////////////////////////////////////////////////////////////////////
//
CHAR        s_szLogFile[MAX_PATH];
CHAR        s_szPipe[MAX_PATH];
LONG        s_nActiveClients = 0;
LONG        s_nTotalClients = 0;
LONGLONG    s_llStartTime;
BOOL        s_fVerbose = FALSE;
TBLOG_PAYLOAD s_Payload;

//////////////////////////////////////////////////////////////////////////////
//
VOID MyErrExit(PCSTR pszMsg)
{
    DWORD error = GetLastError();

    fprintf(stderr, "TRACEBLD: Error %ld in %s.\n", error, pszMsg);
    fflush(stderr);
    exit(1);
}

//////////////////////////////////////////////////////////////////////////////
//
BOOL CLIENT::LogMessageV(PCHAR pszMsg, ...)
{
    DWORD cbWritten = 0;
    CHAR szBuf[1024];
    PCHAR pcchEnd = szBuf + ARRAYSIZE(szBuf) - 2;
    PCHAR pcchCur = szBuf;
    HRESULT hr;

    va_list args;
    va_start(args, pszMsg);
    hr = StringCchVPrintfExA(pcchCur, pcchEnd - pcchCur,
                             &pcchCur, NULL, STRSAFE_NULL_ON_FAILURE,
                             pszMsg, args);
    va_end(args);
    if (FAILED(hr)) {
        goto cleanup;
    }

    hr = StringCchPrintfExA(pcchCur, szBuf + (ARRAYSIZE(szBuf)) - pcchCur,
                            &pcchCur, NULL, STRSAFE_NULL_ON_FAILURE,
                            "\n");

  cleanup:
    WriteFile(hFile, szBuf, (DWORD)(pcchCur - szBuf), &cbWritten, NULL);
    return TRUE;
}

BOOL CLIENT::LogMessage(PTBLOG_MESSAGE pMessage, DWORD nBytes)
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
    DWORD cbWrite = nBytes - offsetof(TBLOG_MESSAGE, szMessage);
    if (cbWrite <= 0 ) {
        return TRUE;
    }

    if (s_fVerbose) {
        printf("[%s]", pMessage->szMessage);
    }

    DWORD cbWritten = 0;
    WriteFile(hFile, pMessage->szMessage, cbWrite, &cbWritten, NULL);
    return TRUE;
}

BOOL CloseConnection(PCLIENT pClient)
{
    InterlockedDecrement(&s_nActiveClients);
    if (pClient != NULL) {
        if (pClient->hPipe != INVALID_HANDLE_VALUE) {
            //FlushFileBuffers(pClient->hPipe);
            if (!DisconnectNamedPipe(pClient->hPipe)) {
                DWORD error = GetLastError();
                pClient->LogMessageV("<!-- Error %d in DisconnectNamedPipe. -->\n", error);
            }
            CloseHandle(pClient->hPipe);
            pClient->hPipe = INVALID_HANDLE_VALUE;
        }
        if (pClient->hFile != INVALID_HANDLE_VALUE) {
            CloseHandle(pClient->hFile);
            pClient->hFile = INVALID_HANDLE_VALUE;
        }
        GlobalFree(pClient);
        pClient = NULL;
    }
    return TRUE;
}

// Creates a pipe instance and initiate an accept request.
//
PCLIENT CreatePipeConnection(HANDLE hCompletionPort, LONG nClient)
{
    HANDLE hPipe = CreateNamedPipeA(s_szPipe,                   // pipe name
                                    PIPE_ACCESS_INBOUND |       // read-only access
                                    FILE_FLAG_OVERLAPPED,       // overlapped mode
                                    PIPE_TYPE_MESSAGE |         // message-type pipe
                                    PIPE_READMODE_MESSAGE |     // message read mode
                                    PIPE_WAIT,                  // blocking mode
                                    PIPE_UNLIMITED_INSTANCES,   // unlimited instances
                                    0,                          // output buffer size
                                    0,                          // input buffer size
                                    20000,                      // client time-out
                                    NULL);                      // no security attributes
    if (hPipe == INVALID_HANDLE_VALUE) {
        MyErrExit("CreateNamedPipe");
    }

    // Allocate the client data structure.
    //
    PCLIENT pClient = (PCLIENT) GlobalAlloc(GPTR, sizeof(CLIENT));
    if (pClient == NULL) {
        MyErrExit("GlobalAlloc pClient");
    }

    CHAR szLogFile[MAX_PATH];
    StringCchPrintfA(szLogFile, ARRAYSIZE(szLogFile), "%s.%08d.xml", s_szLogFile, nClient);

    ZeroMemory(pClient, sizeof(*pClient));
    pClient->hPipe = hPipe;
    pClient->nClient = nClient;
    pClient->fAwaitingAccept = TRUE;
    pClient->hFile = CreateFileA(szLogFile,
                                 GENERIC_WRITE,
                                 FILE_SHARE_READ,
                                 NULL,
                                 CREATE_ALWAYS,
                                 FILE_ATTRIBUTE_NORMAL |
                                 FILE_FLAG_SEQUENTIAL_SCAN,
                                 NULL);
    if (pClient->hFile == INVALID_HANDLE_VALUE) {
        fprintf(stderr, "TRACEBLD: Error opening output file: %s: %ld\n\n",
                szLogFile, GetLastError());
        fflush(stderr);
        MyErrExit("CreateFile");
    }

    // Associate file with our complietion port.
    //
    if (!CreateIoCompletionPort(pClient->hPipe, hCompletionPort, (ULONG_PTR)pClient, 0)) {
        MyErrExit("CreateIoComplietionPort pClient");
    }

    if (!ConnectNamedPipe(hPipe, pClient)) {
        DWORD error = GetLastError();

        if (error == ERROR_IO_PENDING) {
            return NULL;
        }
        if (error == ERROR_PIPE_CONNECTED) {
#if 0
            pClient->LogMessageV("<!-- ConnectNamedPipe client already connected. -->");
#endif
            pClient->fAwaitingAccept = FALSE;
        }
        else if (error != ERROR_IO_PENDING &&
                 error != ERROR_PIPE_LISTENING) {

            MyErrExit("ConnectNamedPipe");
        }
    }
    else {
        fprintf(stderr, "*** ConnectNamedPipe accepted immediately.\n");
#if 0
        pClient->LogMessageV("<!-- ConnectNamedPipe accepted immediately. -->");
#endif
        pClient->fAwaitingAccept = FALSE;
    }
    return pClient;
}

BOOL DoRead(PCLIENT pClient)
{
    SetLastError(NO_ERROR);
    DWORD nBytes = 0;
    BOOL b = ReadFile(pClient->hPipe, &pClient->Message, sizeof(pClient->Message),
                      &nBytes, pClient);

    DWORD error = GetLastError();

    if (b && error == NO_ERROR) {
        return TRUE;
    }
    if (error == ERROR_BROKEN_PIPE) {
        pClient->LogMessageV("<!-- **** ReadFile 002 *** ERROR_BROKEN_PIPE [%d] -->\n", nBytes);
        CloseConnection(pClient);
        return TRUE;
    }
    else if (error == ERROR_INVALID_HANDLE) {
        // ?
        pClient->LogMessageV("<!-- **** ReadFile 002 *** ERROR_INVALID_HANDLE -->\n");
        // I have no idea why this happens.  Our remedy is to drop the connection.
        return TRUE;
    }
    else if (error != ERROR_IO_PENDING) {
        if (b) {
            pClient->LogMessageV("<!-- **** ReadFile 002 succeeded: %d -->\n", error);
        }
        else {
            pClient->LogMessageV("<!-- **** ReadFile 002 failed: %d -->\n", error);
        }
        CloseConnection(pClient);
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

        if (!b) {
            if (pClient) {
                if (GetLastError() == ERROR_BROKEN_PIPE) {
                    pClient->LogMessageV("<!-- Client closed pipe. -->");
                }
                else {
                    pClient->LogMessageV("<!-- *** GetQueuedCompletionStatus failed %d -->",
                                         GetLastError());
                }
                CloseConnection(pClient);
            }
            continue;
        }

        if (pClient->fAwaitingAccept) {
            BOOL fAgain = TRUE;
            while (fAgain) {
                LONG nClient = InterlockedIncrement(&s_nTotalClients);
                InterlockedIncrement(&s_nActiveClients);
                pClient->fAwaitingAccept = FALSE;

                PCLIENT pNew = CreatePipeConnection(hCompletionPort, nClient);

                fAgain = FALSE;
                if (pNew != NULL) {
                    fAgain = !pNew->fAwaitingAccept;
                    DoRead(pNew);
                }
            }
        }
        else {
            if (nBytes <= offsetof(TBLOG_MESSAGE, szMessage)) {
                pClient->LogMessageV("</t:Process>\n");
                CloseConnection(pClient);
                continue;
            }
            pClient->LogMessage(&pClient->Message, nBytes);
        }

        DoRead(pClient);
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

    for (i = 0; i < 1; i++) {
        hThread = CreateThread(NULL, 0, WorkerThread, hCompletionPort, 0, &dwThread);
        if (!hThread) {
            MyErrExit("CreateThread WorkerThread");
            // Unreachable: return FALSE;
        }
        CloseHandle(hThread);
    }
    return TRUE;
}

DWORD CopyEnvironment(PWCHAR pwzzOut, PCWSTR pwzzIn)
{
    PCWSTR pwzzBeg = pwzzOut;
    while (*pwzzIn) {
        while (*pwzzIn) {
            *pwzzOut++ = *pwzzIn++;
        }
        *pwzzOut++ = *pwzzIn++;   // Copy zero.
    }
    *pwzzOut++ = '\0';    // Add last zero.

    return (DWORD)(pwzzOut - pwzzBeg);
}

//////////////////////////////////////////////////////////////////////////////
//
DWORD main(int argc, char **argv)
{
    HANDLE hCompletionPort;
    BOOL fNeedHelp = FALSE;
    WCHAR wzzDrop[1024] = L"build\0nmake\0";

    GetSystemTimeAsFileTime((FILETIME *)&s_llStartTime);
    StringCchPrintfA(s_szPipe, ARRAYSIZE(s_szPipe), "%s.%d", TBLOG_PIPE_NAME, GetCurrentProcessId());

    int arg = 1;
    for (; arg < argc && (argv[arg][0] == '-' || argv[arg][0] == '/'); arg++) {
        CHAR *argn = argv[arg] + 1;
        CHAR *argp = argn;
        while (*argp && *argp != ':' && *argp != '=') {
            argp++;
        }
        if (*argp == ':' || *argp == '=') {
            *argp++ = '\0';
        }

        switch (argn[0]) {

          case 'd':                                     // Drop Processes
          case 'D':
            if (*argp) {
                PWCHAR pwz = wzzDrop;
                while (*argp) {
                    if (*argp == ';') {
                        *pwz++ = '\0';
                    }
                    else {
                        *pwz++ = *argp++;
                    }
                }
                *pwz++ = '\0';
                *pwz = '\0';
            }
          case 'o':                                 // Output file.
          case 'O':
            StringCchCopyA(s_szLogFile, ARRAYSIZE(s_szLogFile), argp);
            break;

          case 'v':                                     // Verbose
          case 'V':
            s_fVerbose = TRUE;
            break;

          case '?':                                 // Help.
            fNeedHelp = TRUE;
            break;

          default:
            fNeedHelp = TRUE;
            printf("TRACEBLD: Bad argument: %s:%s\n", argn, argp);
            break;
        }
    }

    if (arg >= argc) {
        fNeedHelp = TRUE;
    }

    if (fNeedHelp) {
        printf("Usage:\n"
               "    tracebld [options] command {command arguments}\n"
               "Options:\n"
               "    /o:file    Log all events to the output files.\n"
               "    /?         Display this help message.\n"
               "Summary:\n"
               "    Runs the build commands and figures out which files have dependencies..\n"
               "\n");
        exit(9001);
    }

    // Create the completion port.
    hCompletionPort = CreateIoCompletionPort(INVALID_HANDLE_VALUE, NULL, NULL, 0);
    if (hCompletionPort == NULL) {
        MyErrExit("CreateIoCompletionPort");
    }

    // Create completion port worker threads.
    //
    CreateWorkers(hCompletionPort);
    CreatePipeConnection(hCompletionPort, 0);

    printf("TRACEBLD: Ready for clients.  Press Ctrl-C to stop.\n");

    /////////////////////////////////////////////////////////// Validate DLLs.
    //
    CHAR szTmpPath[MAX_PATH];
    CHAR szExePath[MAX_PATH];
    CHAR szDllPath[MAX_PATH];
    PCHAR pszFilePart = NULL;

    if (!GetModuleFileNameA(NULL, szTmpPath, ARRAYSIZE(szTmpPath))) {
        printf("TRACEBLD: Couldn't retreive exe name.\n");
        return 9002;
    }
    if (!GetFullPathNameA(szTmpPath, ARRAYSIZE(szExePath), szExePath, &pszFilePart) ||
        pszFilePart == NULL) {
        printf("TRACEBLD: Error: %s is not a valid path name..\n", szTmpPath);
        return 9002;
    }

    StringCchCopyA(pszFilePart, szExePath + ARRAYSIZE(szExePath) - pszFilePart,
             "trcbld" DETOURS_STRINGIFY(DETOURS_BITS) ".dll");
    StringCchCopyA(szDllPath, ARRAYSIZE(szDllPath), szExePath);

    //////////////////////////////////////////////////////////////////////////
    STARTUPINFOA si;
    PROCESS_INFORMATION pi;
    CHAR szCommand[2048];
    CHAR szExe[MAX_PATH];
    CHAR szFullExe[MAX_PATH] = "\0";
    PCHAR pszFileExe = NULL;

    ZeroMemory(&si, sizeof(si));
    ZeroMemory(&pi, sizeof(pi));
    si.cb = sizeof(si);

    szCommand[0] = L'\0';

    StringCchCopyA(szExe, sizeof(szExe), argv[arg]);
    for (; arg < argc; arg++) {
        if (strchr(argv[arg], ' ') != NULL || strchr(argv[arg], '\t') != NULL) {
            StringCchCatA(szCommand, sizeof(szCommand), "\"");
            StringCchCatA(szCommand, sizeof(szCommand), argv[arg]);
            StringCchCatA(szCommand, sizeof(szCommand), "\"");
        }
        else {
            StringCchCatA(szCommand, sizeof(szCommand), argv[arg]);
        }

        if (arg + 1 < argc) {
            StringCchCatA(szCommand, sizeof(szCommand), " ");
        }
    }
    printf("TRACEBLD: Starting: `%s'\n", szCommand);
    printf("TRACEBLD:   with `%s'\n", szDllPath);
    fflush(stdout);

    DWORD dwFlags = CREATE_DEFAULT_ERROR_MODE | CREATE_SUSPENDED;

    SetLastError(0);
    SearchPathA(NULL, szExe, ".exe", ARRAYSIZE(szFullExe), szFullExe, &pszFileExe);


    if (!DetourCreateProcessWithDllExA(szFullExe[0] ? szFullExe : NULL, szCommand,
                                       NULL, NULL, TRUE, dwFlags, NULL, NULL,
                                       &si, &pi, szDllPath, NULL)) {
        printf("TRACEBLD: DetourCreateProcessWithDllEx failed: %ld\n", GetLastError());
        ExitProcess(9007);
    }

    ZeroMemory(&s_Payload, sizeof(s_Payload));
    s_Payload.nParentProcessId = GetCurrentProcessId();
    s_Payload.nTraceProcessId = GetCurrentProcessId();
    s_Payload.nGeneology = 1;
    s_Payload.rGeneology[0] = 0;
    StringCchCopyW(s_Payload.wzStdin, ARRAYSIZE(s_Payload.wzStdin), L"\\\\.\\CONIN$");
    StringCchCopyW(s_Payload.wzStdout, ARRAYSIZE(s_Payload.wzStdout), L"\\\\.\\CONOUT$");
    StringCchCopyW(s_Payload.wzStderr, ARRAYSIZE(s_Payload.wzStderr), L"\\\\.\\CONOUT$");
    StringCchCopyW(s_Payload.wzParents, ARRAYSIZE(s_Payload.wzParents), L"");
    CopyEnvironment(s_Payload.wzzDrop, wzzDrop);
    LPWCH pwStrings = GetEnvironmentStringsW();
    CopyEnvironment(s_Payload.wzzEnvironment, pwStrings);
    FreeEnvironmentStringsW(pwStrings);

    if (!DetourCopyPayloadToProcess(pi.hProcess, s_guidTrace,
                                    &s_Payload, sizeof(s_Payload))) {
        printf("TRACEBLD: DetourCopyPayloadToProcess failed: %ld\n", GetLastError());
        ExitProcess(9008);
    }

    ResumeThread(pi.hThread);

    WaitForSingleObject(pi.hProcess, INFINITE);

    DWORD dwResult = 0;
    if (!GetExitCodeProcess(pi.hProcess, &dwResult)) {
        printf("TRACEBLD: GetExitCodeProcess failed: %ld\n", GetLastError());
        return 9008;
    }

    printf("TRACEBLD: %ld processes.\n", s_nTotalClients);

    return dwResult;
}
//
//////////////////////////////////////////////////////////////////////////////
