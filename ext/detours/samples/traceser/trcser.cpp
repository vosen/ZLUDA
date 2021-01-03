//////////////////////////////////////////////////////////////////////////////
//
//  Detours Test Program (trcser.cpp of trcser.dll)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
#define _WIN32_WINNT        0x0400
#define WIN32
#define NT

#define DBG_TRACE   0

#include <windows.h>
#include <stdio.h>
#include "detours.h"
#include "syelog.h"

#define PULONG_PTR          PVOID
#define PLONG_PTR           PVOID
#define ULONG_PTR           PVOID
#define ENUMRESNAMEPROCA    PVOID
#define ENUMRESNAMEPROCW    PVOID
#define ENUMRESLANGPROCA    PVOID
#define ENUMRESLANGPROCW    PVOID
#define ENUMRESTYPEPROCA    PVOID
#define ENUMRESTYPEPROCW    PVOID
#define STGOPTIONS          PVOID

//////////////////////////////////////////////////////////////////////
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

//////////////////////////////////////////////////////////////////////
static HMODULE s_hInst = NULL;
static CHAR s_szDllPath[MAX_PATH];

VOID _PrintDump(HANDLE h, PCHAR pszData, INT cbData);
VOID _PrintEnter(PCSTR psz, ...);
VOID _PrintExit(PCSTR psz, ...);
VOID _Print(PCSTR psz, ...);

VOID AssertMessage(CONST PCHAR pszMsg, CONST PCHAR pszFile, ULONG nLine);

//////////////////////////////////////////////////////////////////////////////
//
extern "C" {
    HANDLE (WINAPI * Real_CreateFileW)(LPCWSTR a0,
                                       DWORD a1,
                                       DWORD a2,
                                       LPSECURITY_ATTRIBUTES a3,
                                       DWORD a4,
                                       DWORD a5,
                                       HANDLE a6)
        = CreateFileW;

    BOOL (WINAPI * Real_WriteFile)(HANDLE hFile,
                                     LPCVOID lpBuffer,
                                     DWORD nNumberOfBytesToWrite,
                                     LPDWORD lpNumberOfBytesWritten,
                                     LPOVERLAPPED lpOverlapped)
        = WriteFile;
    BOOL (WINAPI * Real_FlushFileBuffers)(HANDLE hFile)
        = FlushFileBuffers;
    BOOL (WINAPI * Real_CloseHandle)(HANDLE hObject)
        = CloseHandle;

    BOOL (WINAPI * Real_WaitNamedPipeW)(LPCWSTR lpNamedPipeName, DWORD nTimeOut)
        = WaitNamedPipeW;
    BOOL (WINAPI * Real_SetNamedPipeHandleState)(HANDLE hNamedPipe,
                                                   LPDWORD lpMode,
                                                   LPDWORD lpMaxCollectionCount,
                                                   LPDWORD lpCollectDataTimeout)
        = SetNamedPipeHandleState;

    DWORD (WINAPI * Real_GetCurrentProcessId)(VOID)
        = GetCurrentProcessId;
    VOID (WINAPI * Real_GetSystemTimeAsFileTime)(LPFILETIME lpSystemTimeAsFileTime)
        = GetSystemTimeAsFileTime;

    VOID (WINAPI * Real_InitializeCriticalSection)(LPCRITICAL_SECTION lpSection)
        = InitializeCriticalSection;
    VOID (WINAPI * Real_EnterCriticalSection)(LPCRITICAL_SECTION lpSection)
        = EnterCriticalSection;
    VOID (WINAPI * Real_LeaveCriticalSection)(LPCRITICAL_SECTION lpSection)
        = LeaveCriticalSection;
}

DWORD (WINAPI * Real_GetModuleFileNameW)(HMODULE a0,
                                         LPWSTR a1,
                                         DWORD a2)
    = GetModuleFileNameW;

DWORD (WINAPI * Real_GetModuleFileNameA)(HMODULE a0,
                                         LPSTR a1,
                                         DWORD a2)
    = GetModuleFileNameA;

BOOL (WINAPI * Real_CreateProcessW)(LPCWSTR a0,
                                    LPWSTR a1,
                                    LPSECURITY_ATTRIBUTES a2,
                                    LPSECURITY_ATTRIBUTES a3,
                                    BOOL a4,
                                    DWORD a5,
                                    LPVOID a6,
                                    LPCWSTR a7,
                                    struct _STARTUPINFOW* a8,
                                    LPPROCESS_INFORMATION a9)
    = CreateProcessW;

BOOL (WINAPI * Real_BuildCommDCBA)(LPCSTR a0,
                                   struct _DCB* a1)
    = BuildCommDCBA;

BOOL (WINAPI * Real_BuildCommDCBAndTimeoutsA)(LPCSTR a0,
                                              struct _DCB* a1,
                                              struct _COMMTIMEOUTS* a2)
    = BuildCommDCBAndTimeoutsA;

BOOL (WINAPI * Real_BuildCommDCBAndTimeoutsW)(LPCWSTR a0,
                                              struct _DCB* a1,
                                              struct _COMMTIMEOUTS* a2)
    = BuildCommDCBAndTimeoutsW;

BOOL (WINAPI * Real_BuildCommDCBW)(LPCWSTR a0,
                                   struct _DCB* a1)
    = BuildCommDCBW;

BOOL (WINAPI * Real_ClearCommBreak)(HANDLE a0)
    = ClearCommBreak;

BOOL (WINAPI * Real_ClearCommError)(HANDLE a0,
                                    LPDWORD a1,
                                    struct _COMSTAT* a2)
    = ClearCommError;

HANDLE (WINAPI * Real_CreateFileA)(LPCSTR a0,
                                   DWORD a1,
                                   DWORD a2,
                                   LPSECURITY_ATTRIBUTES a3,
                                   DWORD a4,
                                   DWORD a5,
                                   HANDLE a6)
    = CreateFileA;

BOOL (WINAPI * Real_EscapeCommFunction)(HANDLE a0,
                                        DWORD a1)
    = EscapeCommFunction;

BOOL (WINAPI * Real_GetCommConfig)(HANDLE a0,
                                   LPCOMMCONFIG a1,
                                   LPDWORD a2)
    = GetCommConfig;

BOOL (WINAPI * Real_GetCommMask)(HANDLE a0,
                                 LPDWORD a1)
    = GetCommMask;

BOOL (WINAPI * Real_GetCommModemStatus)(HANDLE a0,
                                        LPDWORD a1)
    = GetCommModemStatus;

BOOL (WINAPI * Real_GetCommProperties)(HANDLE a0,
                                       LPCOMMPROP a1)
    = GetCommProperties;

BOOL (WINAPI * Real_GetCommState)(HANDLE a0,
                                  struct _DCB* a1)
    = GetCommState;

BOOL (WINAPI * Real_GetCommTimeouts)(HANDLE a0,
                                     struct _COMMTIMEOUTS* a1)
    = GetCommTimeouts;

DWORD (WINAPI * Real_GetCurrentThreadId)(void)
    = GetCurrentThreadId;

BOOL (WINAPI * Real_GetOverlappedResult)(HANDLE a0,
                                         LPOVERLAPPED a1,
                                         LPDWORD a2,
                                         BOOL a3)
    = GetOverlappedResult;

BOOL (WINAPI * Real_PurgeComm)(HANDLE a0,
                               DWORD a1)
    = PurgeComm;

BOOL (WINAPI * Real_ReadFile)(HANDLE a0,
                              LPVOID a1,
                              DWORD a2,
                              LPDWORD a3,
                              LPOVERLAPPED a4)
    = ReadFile;

BOOL (WINAPI * Real_SetCommBreak)(HANDLE a0)
    = SetCommBreak;

BOOL (WINAPI * Real_SetCommConfig)(HANDLE a0,
                                   LPCOMMCONFIG a1,
                                   DWORD a2)
    = SetCommConfig;

BOOL (WINAPI * Real_SetCommMask)(HANDLE a0,
                                 DWORD a1)
    = SetCommMask;

BOOL (WINAPI * Real_SetCommState)(HANDLE a0,
                                  struct _DCB* a1)
    = SetCommState;

BOOL (WINAPI * Real_SetCommTimeouts)(HANDLE a0,
                                     struct _COMMTIMEOUTS* a1)
    = SetCommTimeouts;

BOOL (WINAPI * Real_SetupComm)(HANDLE a0,
                               DWORD a1,
                               DWORD a2)
    = SetupComm;

BOOL (WINAPI * Real_TransmitCommChar)(HANDLE a0,
                                      char a1)
    = TransmitCommChar;

BOOL (WINAPI * Real_WaitCommEvent)(HANDLE a0,
                                   LPDWORD a1,
                                   LPOVERLAPPED a2)
    = WaitCommEvent;

/////////////////////////////////////////////////////////////
// Detours
//

BOOL WINAPI Mine_CreateProcessW(LPCWSTR lpApplicationName,
                                LPWSTR lpCommandLine,
                                LPSECURITY_ATTRIBUTES lpProcessAttributes,
                                LPSECURITY_ATTRIBUTES lpThreadAttributes,
                                BOOL bInheritHandles,
                                DWORD dwCreationFlags,
                                LPVOID lpEnvironment,
                                LPCWSTR lpCurrentDirectory,
                                LPSTARTUPINFOW lpStartupInfo,
                                LPPROCESS_INFORMATION lpProcessInformation)
{
    _PrintEnter("CreateProcessW(%ls,%ls,%p,%p,%x,%x,%p,%ls,%p,%p)\n",
                lpApplicationName,
                lpCommandLine,
                lpProcessAttributes,
                lpThreadAttributes,
                bInheritHandles,
                dwCreationFlags,
                lpEnvironment,
                lpCurrentDirectory,
                lpStartupInfo,
                lpProcessInformation);

    _Print("Calling DetourCreateProcessWithDllExW(,%hs)\n", s_szDllPath);

    BOOL rv = 0;
    __try {
        rv = DetourCreateProcessWithDllExW(lpApplicationName,
                                           lpCommandLine,
                                           lpProcessAttributes,
                                           lpThreadAttributes,
                                           bInheritHandles,
                                           dwCreationFlags,
                                           lpEnvironment,
                                           lpCurrentDirectory,
                                           lpStartupInfo,
                                           lpProcessInformation,
                                           s_szDllPath,
                                           Real_CreateProcessW);
    } __finally {
        _PrintExit("CreateProcessW(,,,,,,,,,) -> %x\n", rv);
    };
    return rv;
}

BOOL WINAPI Mine_BuildCommDCBA(LPCSTR a0,
                               LPDCB a1)
{
    _PrintEnter("BuildCommDCBA(%hs,%p)\n", a0, a1);

    BOOL rv = 0;
    __try {
        rv = Real_BuildCommDCBA(a0, a1);
    } __finally {
        _PrintExit("BuildCommDCBA(,) -> %x\n", rv);
    };
    return rv;
}

BOOL WINAPI Mine_BuildCommDCBAndTimeoutsA(LPCSTR a0,
                                          LPDCB a1,
                                          LPCOMMTIMEOUTS a2)
{
    _PrintEnter("BuildCommDCBAndTimeoutsA(%hs,%p,%p)\n", a0, a1, a2);

    BOOL rv = 0;
    __try {
        rv = Real_BuildCommDCBAndTimeoutsA(a0, a1, a2);
    } __finally {
        _PrintExit("BuildCommDCBAndTimeoutsA(,,) -> %x\n", rv);
    };
    return rv;
}

BOOL WINAPI Mine_BuildCommDCBAndTimeoutsW(LPCWSTR a0,
                                          LPDCB a1,
                                          LPCOMMTIMEOUTS a2)
{
    _PrintEnter("BuildCommDCBAndTimeoutsW(%ls,%p,%p)\n", a0, a1, a2);

    BOOL rv = 0;
    __try {
        rv = Real_BuildCommDCBAndTimeoutsW(a0, a1, a2);
    } __finally {
        _PrintExit("BuildCommDCBAndTimeoutsW(,,) -> %x\n", rv);
    };
    return rv;
}

BOOL WINAPI Mine_BuildCommDCBW(LPCWSTR a0,
                               LPDCB a1)
{
    _PrintEnter("BuildCommDCBW(%ls,%p)\n", a0, a1);

    BOOL rv = 0;
    __try {
        rv = Real_BuildCommDCBW(a0, a1);
    } __finally {
        _PrintExit("BuildCommDCBW(,) -> %x\n", rv);
    };
    return rv;
}

BOOL WINAPI Mine_ClearCommBreak(HANDLE a0)
{
    _PrintEnter("ClearCommBreak(%p)\n", a0);

    BOOL rv = 0;
    __try {
        rv = Real_ClearCommBreak(a0);
    } __finally {
        _PrintExit("ClearCommBreak() -> %x\n", rv);
    };
    return rv;
}

BOOL WINAPI Mine_ClearCommError(HANDLE a0,
                                LPDWORD a1,
                                LPCOMSTAT a2)
{
    _PrintEnter("ClearCommError(%p,%p,%p)\n", a0, a1, a2);

    BOOL rv = 0;
    __try {
        rv = Real_ClearCommError(a0, a1, a2);
    } __finally {
        _PrintExit("ClearCommError(,,) -> %x\n", rv);
    };
    return rv;
}

BOOL WINAPI Mine_CloseHandle(HANDLE a0)
{
    _PrintEnter("CloseHandle(%p)\n", a0);

    BOOL rv = 0;
    __try {
        rv = Real_CloseHandle(a0);
    } __finally {
        _PrintExit("CloseHandle() -> %x\n", rv);
    };
    return rv;
}

HANDLE WINAPI Mine_CreateFileA(LPCSTR a0,
                               DWORD a1,
                               DWORD a2,
                               LPSECURITY_ATTRIBUTES a3,
                               DWORD a4,
                               DWORD a5,
                               HANDLE a6)
{
    _PrintEnter("CreateFileA(%hs,%x,%x,%p,%x,%x,%p)\n", a0, a1, a2, a3, a4, a5, a6);

    HANDLE rv = 0;
    __try {
        rv = Real_CreateFileA(a0, a1, a2, a3, a4, a5, a6);
    } __finally {
        _PrintExit("CreateFileA(,,,,,,) -> %p\n", rv);
    };
    return rv;
}

HANDLE WINAPI Mine_CreateFileW(LPCWSTR a0,
                               DWORD a1,
                               DWORD a2,
                               LPSECURITY_ATTRIBUTES a3,
                               DWORD a4,
                               DWORD a5,
                               HANDLE a6)
{
    _PrintEnter("CreateFileW(%ls,%x,%x,%p,%x,%x,%p)\n", a0, a1, a2, a3, a4, a5, a6);

    HANDLE rv = 0;
    __try {
        rv = Real_CreateFileW(a0, a1, a2, a3, a4, a5, a6);
    } __finally {
        _PrintExit("CreateFileW(,,,,,,) -> %p\n", rv);
    };
    return rv;
}

BOOL WINAPI Mine_EscapeCommFunction(HANDLE a0,
                                    DWORD a1)
{
    _PrintEnter("EscapeCommFunction(%p,%x)\n", a0, a1);

    BOOL rv = 0;
    __try {
        rv = Real_EscapeCommFunction(a0, a1);
    } __finally {
        _PrintExit("EscapeCommFunction(,) -> %x\n", rv);
    };
    return rv;
}

BOOL WINAPI Mine_GetCommConfig(HANDLE a0,
                               LPCOMMCONFIG a1,
                               LPDWORD a2)
{
    _PrintEnter("GetCommConfig(%p,%p,%p)\n", a0, a1, a2);

    BOOL rv = 0;
    __try {
        rv = Real_GetCommConfig(a0, a1, a2);
    } __finally {
        _PrintExit("GetCommConfig(,,) -> %x\n", rv);
    };
    return rv;
}

BOOL WINAPI Mine_GetCommMask(HANDLE a0,
                                LPDWORD a1)
{
    _PrintEnter("GetCommMask(%p,%p)\n", a0, a1);

    BOOL rv = 0;
    __try {
        rv = Real_GetCommMask(a0, a1);
    } __finally {
        _PrintExit("GetCommMask(,) -> %x\n", rv);
    };
    return rv;
}

BOOL WINAPI Mine_GetCommModemStatus(HANDLE a0,
                                    LPDWORD a1)
{
    _PrintEnter("GetCommModemStatus(%p,%p)\n", a0, a1);

    BOOL rv = 0;
    __try {
        rv = Real_GetCommModemStatus(a0, a1);
    } __finally {
        _PrintExit("GetCommModemStatus(,) -> %x\n", rv);
    };
    return rv;
}

BOOL WINAPI Mine_GetCommProperties(HANDLE a0,
                                   LPCOMMPROP a1)
{
    _PrintEnter("GetCommProperties(%p,%p)\n", a0, a1);

    BOOL rv = 0;
    __try {
        rv = Real_GetCommProperties(a0, a1);
    } __finally {
        _PrintExit("GetCommProperties(,) -> %x\n", rv);
    };
    return rv;
}

BOOL WINAPI Mine_GetCommState(HANDLE a0,
                              LPDCB a1)
{
    _PrintEnter("GetCommState(%p,%p)\n", a0, a1);

    BOOL rv = 0;
    __try {
        rv = Real_GetCommState(a0, a1);
    } __finally {
        _PrintExit("GetCommState(,) -> %x\n", rv);
    };
    return rv;
}

BOOL WINAPI Mine_GetCommTimeouts(HANDLE a0,
                                 LPCOMMTIMEOUTS a1)
{
    _PrintEnter("GetCommTimeouts(%p,%p)\n", a0, a1);

    BOOL rv = 0;
    __try {
        rv = Real_GetCommTimeouts(a0, a1);
    } __finally {
        _PrintExit("GetCommTimeouts(,) -> %x\n", rv);
    };
    return rv;
}

DWORD WINAPI Mine_GetCurrentThreadId(void)
{
    _PrintEnter("GetCurrentThreadId()\n");

    DWORD rv = 0;
    __try {
        rv = Real_GetCurrentThreadId();
    } __finally {
        _PrintExit("GetCurrentThreadId() -> %x\n", rv);
    };
    return rv;
}

DWORD WINAPI Mine_GetModuleFileNameW(HINSTANCE a0,
                                     LPWSTR a1,
                                     DWORD a2)
{
    _PrintEnter("GetModuleFileNameW(%p,%p,%x)\n", a0, a1, a2);

    DWORD rv = 0;
    __try {
        rv = Real_GetModuleFileNameW(a0, a1, a2);
    } __finally {
        _PrintExit("GetModuleFileNameW(,%ls,) -> %x\n", a1, rv);
    };
    return rv;
}

BOOL WINAPI Mine_GetOverlappedResult(HANDLE a0,
                                     LPOVERLAPPED a1,
                                     LPDWORD a2,
                                     BOOL a3)
{
    _PrintEnter("GetOverlappedResult(%p,%p,%p,%x)\n", a0, a1, a2, a3);

    BOOL rv = 0;
    __try {
        rv = Real_GetOverlappedResult(a0, a1, a2, a3);
    } __finally {
        _PrintExit("GetOverlappedResult(,,,) -> %x\n", rv);
    };
    return rv;
}

BOOL WINAPI Mine_PurgeComm(HANDLE a0,
                           DWORD a1)
{
    _PrintEnter("PurgeComm(%p,%x)\n", a0, a1);

    BOOL rv = 0;
    __try {
        rv = Real_PurgeComm(a0, a1);
    } __finally {
        _PrintExit("PurgeComm(,) -> %x\n", rv);
    };
    return rv;
}

BOOL WINAPI Mine_ReadFile(HANDLE a0,
                          LPVOID a1,
                          DWORD a2,
                          LPDWORD a3,
                          LPOVERLAPPED a4)
{
    _PrintEnter("ReadFile(%p,%p,%x,%p,%p)\n", a0, a1, a2, a3, a4);

    BOOL rv = 0;
    __try {
        rv = Real_ReadFile(a0, a1, a2, a3, a4);
    } __finally {
        _PrintExit("ReadFile(,,,,) -> %x\n", rv);
    };
    return rv;
}

BOOL WINAPI Mine_SetCommBreak(HANDLE a0)
{
    _PrintEnter("SetCommBreak(%p)\n", a0);

    BOOL rv = 0;
    __try {
        rv = Real_SetCommBreak(a0);
    } __finally {
        _PrintExit("SetCommBreak() -> %x\n", rv);
    };
    return rv;
}

BOOL WINAPI Mine_SetCommConfig(HANDLE a0,
                               LPCOMMCONFIG a1,
                               DWORD a2)
{
    _PrintEnter("SetCommConfig(%p,%p,%x)\n", a0, a1, a2);

    BOOL rv = 0;
    __try {
        rv = Real_SetCommConfig(a0, a1, a2);
    } __finally {
        _PrintExit("SetCommConfig(,,) -> %x\n", rv);
    };
    return rv;
}

BOOL WINAPI Mine_SetCommMask(HANDLE a0,
                             DWORD a1)
{
    _PrintEnter("SetCommMask(%p,%x)\n", a0, a1);

    BOOL rv = 0;
    __try {
        rv = Real_SetCommMask(a0, a1);
    } __finally {
        _PrintExit("SetCommMask(,) -> %x\n", rv);
    };
    return rv;
}

BOOL WINAPI Mine_SetCommState(HANDLE a0,
                              LPDCB a1)
{
    _PrintEnter("SetCommState(%p,%p)\n", a0, a1);

    BOOL rv = 0;
    __try {
        rv = Real_SetCommState(a0, a1);
    } __finally {
        _PrintExit("SetCommState(,) -> %x\n", rv);
    };
    return rv;
}

BOOL WINAPI Mine_SetCommTimeouts(HANDLE a0,
                                 LPCOMMTIMEOUTS a1)
{
    _PrintEnter("SetCommTimeouts(%p,%p)\n", a0, a1);

    BOOL rv = 0;
    __try {
        rv = Real_SetCommTimeouts(a0, a1);
    } __finally {
        _PrintExit("SetCommTimeouts(,) -> %x\n", rv);
    };
    return rv;
}

BOOL WINAPI Mine_SetupComm(HANDLE a0,
                           DWORD a1,
                           DWORD a2)
{
    _PrintEnter("SetupComm(%p,%x,%x)\n", a0, a1, a2);

    BOOL rv = 0;
    __try {
        rv = Real_SetupComm(a0, a1, a2);
    } __finally {
        _PrintExit("SetupComm(,,) -> %x\n", rv);
    };
    return rv;
}

BOOL WINAPI Mine_TransmitCommChar(HANDLE a0,
                                  char a1)
{
    _PrintEnter("TransmitCommChar(%p,%x)\n", a0, a1);

    BOOL rv = 0;
    __try {
        rv = Real_TransmitCommChar(a0, a1);
    } __finally {
        _PrintExit("TransmitCommChar(,) -> %x\n", rv);
    };
    return rv;
}

BOOL WINAPI Mine_WaitCommEvent(HANDLE a0,
                               LPDWORD a1,
                               LPOVERLAPPED a2)
{
    _PrintEnter("WaitCommEvent(%p,%p,%p)\n", a0, a1, a2);

    BOOL rv = 0;
    __try {
        rv = Real_WaitCommEvent(a0, a1, a2);
    } __finally {
        _PrintExit("WaitCommEvent(,,) -> %x\n", rv);
    };
    return rv;
}

BOOL WINAPI Mine_WriteFile(HANDLE a0,
                           LPCVOID a1,
                           DWORD a2,
                           LPDWORD a3,
                           LPOVERLAPPED a4)
{
    _PrintEnter("WriteFile(%p,%p,%x,%p,%p)\n", a0, a1, a2, a3, a4);

    BOOL rv = 0;
    __try {
        _PrintDump(a0, (PCHAR)a1, a2);
        rv = Real_WriteFile(a0, a1, a2, a3, a4);
    } __finally {
        _PrintExit("WriteFile(,,,,) -> %x\n", rv);
    };
    return rv;
}

/////////////////////////////////////////////////////////////
// AttachDetours
//
PCHAR DetRealName(PCHAR psz)
{
    PCHAR pszBeg = psz;
    // Move to end of name.
    while (*psz) {
        psz++;
    }
    // Move back through A-Za-z0-9 names.
    while (psz > pszBeg &&
           ((psz[-1] >= 'A' && psz[-1] <= 'Z') ||
            (psz[-1] >= 'a' && psz[-1] <= 'z') ||
            (psz[-1] >= '0' && psz[-1] <= '9'))) {
        psz--;
    }
    return psz;
}

VOID DetAttach(PVOID *ppbReal, PVOID pbMine, PCHAR psz)
{
    LONG l = DetourAttach(ppbReal, pbMine);
    if (l != 0) {
        Syelog(SYELOG_SEVERITY_NOTICE,
               "Attach failed: `%s': error %d\n", DetRealName(psz), l);
    }
}

VOID DetDetach(PVOID *ppbReal, PVOID pbMine, PCHAR psz)
{
    LONG l = DetourDetach(ppbReal, pbMine);
    if (l != 0) {
        Syelog(SYELOG_SEVERITY_NOTICE,
               "Detach failed: `%s': error %d\n", DetRealName(psz), l);
    }
}

#define ATTACH(x)       DetAttach(&(PVOID&)Real_##x,Mine_##x,#x)
#define DETACH(x)       DetDetach(&(PVOID&)Real_##x,Mine_##x,#x)

LONG AttachDetours(VOID)
{
    DetourTransactionBegin();
    DetourUpdateThread(GetCurrentThread());

    ATTACH(BuildCommDCBA);
    ATTACH(BuildCommDCBAndTimeoutsA);
    ATTACH(BuildCommDCBAndTimeoutsW);
    ATTACH(BuildCommDCBW);
    ATTACH(ClearCommBreak);
    ATTACH(ClearCommError);
    ATTACH(CloseHandle);
    ATTACH(CreateFileA);
    ATTACH(CreateFileW);
    ATTACH(EscapeCommFunction);
    ATTACH(GetCommConfig);
    ATTACH(GetCommMask);
    ATTACH(GetCommModemStatus);
    ATTACH(GetCommProperties);
    ATTACH(GetCommState);
    ATTACH(GetCommTimeouts);
    ATTACH(GetCurrentThreadId);
    ATTACH(GetModuleFileNameW);
    ATTACH(GetOverlappedResult);
    ATTACH(PurgeComm);
    ATTACH(ReadFile);
    ATTACH(SetCommBreak);
    ATTACH(SetCommConfig);
    ATTACH(SetCommMask);
    ATTACH(SetCommState);
    ATTACH(SetCommTimeouts);
    ATTACH(SetupComm);
    ATTACH(TransmitCommChar);
    ATTACH(WaitCommEvent);
    ATTACH(WriteFile);

    return DetourTransactionCommit();
}

LONG DetachDetours(VOID)
{
    DetourTransactionBegin();
    DetourUpdateThread(GetCurrentThread());

    DETACH(BuildCommDCBA);
    DETACH(BuildCommDCBAndTimeoutsA);
    DETACH(BuildCommDCBAndTimeoutsW);
    DETACH(BuildCommDCBW);
    DETACH(ClearCommBreak);
    DETACH(ClearCommError);
    DETACH(CloseHandle);
    DETACH(CreateFileA);
    DETACH(CreateFileW);
    DETACH(EscapeCommFunction);
    DETACH(GetCommConfig);
    DETACH(GetCommMask);
    DETACH(GetCommModemStatus);
    DETACH(GetCommProperties);
    DETACH(GetCommState);
    DETACH(GetCommTimeouts);
    DETACH(GetCurrentThreadId);
    DETACH(GetModuleFileNameW);
    DETACH(GetOverlappedResult);
    DETACH(PurgeComm);
    DETACH(ReadFile);
    DETACH(SetCommBreak);
    DETACH(SetCommConfig);
    DETACH(SetCommMask);
    DETACH(SetCommState);
    DETACH(SetCommTimeouts);
    DETACH(SetupComm);
    DETACH(TransmitCommChar);
    DETACH(WaitCommEvent);
    DETACH(WriteFile);

    return DetourTransactionCommit();
}

/////////////////////////////////////////////////////////////
// Detours
//

VOID _PrintDump(HANDLE h, PCHAR pszData, INT cbData)
{
    if (pszData && cbData > 0) {
        CHAR szBuffer[256];
        PCHAR pszBuffer = szBuffer;
        INT cbBuffer = 0;
        INT nLines = 0;

        while (cbData > 0) {
            if (nLines > 20) {
                *pszBuffer++ = '.';
                *pszBuffer++ = '.';
                *pszBuffer++ = '.';
                cbBuffer += 3;
                break;
            }

            if (*pszData == '\t') {
                *pszBuffer++ = '\\';
                *pszBuffer++ = 't';
                cbBuffer += 2;
                pszData++;
                cbData--;
                continue;
            }
            if (*pszData == '\r') {
                *pszBuffer++ = '\\';
                *pszBuffer++ = 'r';
                cbBuffer += 2;
                pszData++;
                cbData--;
                continue;
            }
            else if (*pszData == '\n') {
                *pszBuffer++ = '\\';
                *pszBuffer++ = 'n';
                cbBuffer += 2;
                *pszBuffer++ = '\0';
                _Print("%p:   %hs\n", h, szBuffer);
                nLines++;
                pszBuffer = szBuffer;
                cbBuffer = 0;
                pszData++;
                cbData--;
                continue;
            }
            else if (cbBuffer >= 80) {
                *pszBuffer++ = '\0';
                _Print("%p:   %hs\n", h, szBuffer);
                nLines++;
                pszBuffer = szBuffer;
                cbBuffer = 0;
            }

            if (*pszData < ' ' || *pszData >= 127) {
                *pszBuffer++ = '\\';
                *pszBuffer++ = 'x';
                *pszBuffer++ = "0123456789ABCDEF"[(*pszData & 0xf0) >> 4];
                *pszBuffer++ = "0123456789ABCDEF"[(*pszData & 0x0f)];
                cbBuffer += 4;
            }
            else {
                *pszBuffer++ = *pszData;
            }
            cbBuffer++;
            pszData++;
            cbData--;
        }

        if (cbBuffer > 0) {
            *pszBuffer++ = '\0';
            _Print("%p:   %hs\n", h, szBuffer);
        }
    }
}

////////////////////////////////////////////////////////////// Logging System.
//
static BOOL s_bLog = 1;
static LONG s_nTlsIndent = -1;
static LONG s_nTlsThread = -1;
static LONG s_nThreadCnt = 0;

VOID _PrintEnter(const CHAR *psz, ...)
{
    DWORD dwErr = GetLastError();

    LONG nIndent = 0;
    LONG nThread = 0;
    if (s_nTlsIndent >= 0) {
        nIndent = (LONG)(LONG_PTR)TlsGetValue(s_nTlsIndent);
        TlsSetValue(s_nTlsIndent, (PVOID)(LONG_PTR)(nIndent + 1));
    }
    if (s_nTlsThread >= 0) {
        nThread = (LONG)(LONG_PTR)TlsGetValue(s_nTlsThread);
    }

    if (s_bLog && psz) {
        CHAR szBuf[1024];
        PCHAR pszBuf = szBuf;
        PCHAR pszEnd = szBuf + ARRAYSIZE(szBuf) - 1;
        LONG nLen = (nIndent > 0) ? (nIndent < 35 ? nIndent * 2 : 70) : 0;
        *pszBuf++ = (CHAR)('0' + ((nThread / 100) % 10));
        *pszBuf++ = (CHAR)('0' + ((nThread / 10) % 10));
        *pszBuf++ = (CHAR)('0' + ((nThread / 1) % 10));
        *pszBuf++ = ' ';
        while (nLen-- > 0) {
            *pszBuf++ = ' ';
        }

        va_list  args;
        va_start(args, psz);

        while ((*pszBuf++ = *psz++) != 0 && pszBuf < pszEnd) {
            // Copy characters.
        }
        *pszEnd = '\0';
        SyelogV(SYELOG_SEVERITY_INFORMATION,
                szBuf, args);

        va_end(args);
    }
    SetLastError(dwErr);
}

VOID _PrintExit(const CHAR *psz, ...)
{
    DWORD dwErr = GetLastError();

    LONG nIndent = 0;
    LONG nThread = 0;
    if (s_nTlsIndent >= 0) {
        nIndent = (LONG)(LONG_PTR)TlsGetValue(s_nTlsIndent) - 1;
        ASSERT(nIndent >= 0);
        TlsSetValue(s_nTlsIndent, (PVOID)(LONG_PTR)nIndent);
    }
    if (s_nTlsThread >= 0) {
        nThread = (LONG)(LONG_PTR)TlsGetValue(s_nTlsThread);
    }

    if (s_bLog && psz) {
        CHAR szBuf[1024];
        PCHAR pszBuf = szBuf;
        PCHAR pszEnd = szBuf + ARRAYSIZE(szBuf) - 1;
        LONG nLen = (nIndent > 0) ? (nIndent < 35 ? nIndent * 2 : 70) : 0;
        *pszBuf++ = (CHAR)('0' + ((nThread / 100) % 10));
        *pszBuf++ = (CHAR)('0' + ((nThread / 10) % 10));
        *pszBuf++ = (CHAR)('0' + ((nThread / 1) % 10));
        *pszBuf++ = ' ';
        while (nLen-- > 0) {
            *pszBuf++ = ' ';
        }

        va_list  args;
        va_start(args, psz);

        while ((*pszBuf++ = *psz++) != 0 && pszBuf < pszEnd) {
            // Copy characters.
        }
        *pszEnd = '\0';
        SyelogV(SYELOG_SEVERITY_INFORMATION,
                szBuf, args);

        va_end(args);
    }
    SetLastError(dwErr);
}

VOID _Print(const CHAR *psz, ...)
{
    DWORD dwErr = GetLastError();

    LONG nIndent = 0;
    LONG nThread = 0;
    if (s_nTlsIndent >= 0) {
        nIndent = (LONG)(LONG_PTR)TlsGetValue(s_nTlsIndent);
    }
    if (s_nTlsThread >= 0) {
        nThread = (LONG)(LONG_PTR)TlsGetValue(s_nTlsThread);
    }

    if (s_bLog && psz) {
        CHAR szBuf[1024];
        PCHAR pszBuf = szBuf;
        PCHAR pszEnd = szBuf + ARRAYSIZE(szBuf) - 1;
        LONG nLen = (nIndent > 0) ? (nIndent < 35 ? nIndent * 2 : 70) : 0;
        *pszBuf++ = (CHAR)('0' + ((nThread / 100) % 10));
        *pszBuf++ = (CHAR)('0' + ((nThread / 10) % 10));
        *pszBuf++ = (CHAR)('0' + ((nThread / 1) % 10));
        *pszBuf++ = ' ';
        while (nLen-- > 0) {
            *pszBuf++ = ' ';
        }

        va_list  args;
        va_start(args, psz);

        while ((*pszBuf++ = *psz++) != 0 && pszBuf < pszEnd) {
            // Copy characters.
        }
        *pszEnd = '\0';
        SyelogV(SYELOG_SEVERITY_INFORMATION,
                szBuf, args);

        va_end(args);
    }

    SetLastError(dwErr);
}

VOID AssertMessage(CONST PCHAR pszMsg, CONST PCHAR pszFile, ULONG nLine)
{
    Syelog(SYELOG_SEVERITY_FATAL,
           "ASSERT(%s) failed in %s, line %d.\n", pszMsg, pszFile, nLine);
}

//////////////////////////////////////////////////////////////////////////////
//
// DLL module information
//
BOOL ThreadAttach(HMODULE hDll)
{
    (void)hDll;

    if (s_nTlsIndent >= 0) {
        TlsSetValue(s_nTlsIndent, (PVOID)0);
    }
    if (s_nTlsThread >= 0) {
        LONG nThread = InterlockedIncrement(&s_nThreadCnt);
        TlsSetValue(s_nTlsThread, (PVOID)(LONG_PTR)nThread);
    }
    return TRUE;
}

BOOL ThreadDetach(HMODULE hDll)
{
    (void)hDll;

    if (s_nTlsIndent >= 0) {
        TlsSetValue(s_nTlsIndent, (PVOID)0);
    }
    if (s_nTlsThread >= 0) {
        TlsSetValue(s_nTlsThread, (PVOID)0);
    }
    return TRUE;
}

BOOL ProcessAttach(HMODULE hDll)
{
    s_bLog = FALSE;
    s_nTlsIndent = TlsAlloc();
    s_nTlsThread = TlsAlloc();

    WCHAR wzExePath[MAX_PATH];

    s_hInst = hDll;
    Real_GetModuleFileNameA(s_hInst, s_szDllPath, ARRAYSIZE(s_szDllPath));
    Real_GetModuleFileNameW(NULL, wzExePath, ARRAYSIZE(wzExePath));

    SyelogOpen("trcser" DETOURS_STRINGIFY(DETOURS_BITS), SYELOG_FACILITY_APPLICATION);
    Syelog(SYELOG_SEVERITY_INFORMATION,
           "##################################################################\n");
    Syelog(SYELOG_SEVERITY_INFORMATION,
           "### %ls\n", wzExePath);
    LONG error = AttachDetours();
    if (error != NO_ERROR) {
        Syelog(SYELOG_SEVERITY_FATAL, "### Error attaching detours: %d\n", error);
    }

    ThreadAttach(hDll);

    s_bLog = TRUE;
    return TRUE;
}

BOOL ProcessDetach(HMODULE hDll)
{
    ThreadDetach(hDll);
    s_bLog = FALSE;

    LONG error = DetachDetours();
    if (error != NO_ERROR) {
        Syelog(SYELOG_SEVERITY_FATAL, "### Error detaching detours: %d\n", error);
    }

    Syelog(SYELOG_SEVERITY_NOTICE, "### Closing.\n");
    SyelogClose(FALSE);

    if (s_nTlsIndent >= 0) {
        TlsFree(s_nTlsIndent);
    }
    if (s_nTlsThread >= 0) {
        TlsFree(s_nTlsThread);
    }
    return TRUE;
}

BOOL APIENTRY DllMain(HINSTANCE hModule, DWORD dwReason, PVOID lpReserved)
{
    (void)hModule;
    (void)lpReserved;

    if (DetourIsHelperProcess()) {
        return TRUE;
    }

    switch (dwReason) {
      case DLL_PROCESS_ATTACH:
        DetourRestoreAfterWith();
        return ProcessAttach(hModule);
      case DLL_PROCESS_DETACH:
        return ProcessDetach(hModule);
      case DLL_THREAD_ATTACH:
        return ThreadAttach(hModule);
      case DLL_THREAD_DETACH:
        return ThreadDetach(hModule);
    }
    return TRUE;
}
//
///////////////////////////////////////////////////////////////// End of File.
