//////////////////////////////////////////////////////////////////////////////
//
//  Detours Test Program (trctcp.cpp of trctcp.dll)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
#define _WIN32_WINNT        0x0400
#define WIN32
#define NT
#define _WINSOCK_DEPRECATED_NO_WARNINGS

#define DBG_TRACE   0

#if _MSC_VER >= 1300
#include <winsock2.h>
#endif
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
static HMODULE s_hInst = NULL;
static WCHAR s_wzDllPath[MAX_PATH];

VOID _PrintDump(SOCKET socket, PCHAR pszData, INT cbData);
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

BOOL (WINAPI * Real_CreateProcessW)(LPCWSTR a0,
                                    LPWSTR a1,
                                    LPSECURITY_ATTRIBUTES a2,
                                    LPSECURITY_ATTRIBUTES a3,
                                    BOOL a4,
                                    DWORD a5,
                                    LPVOID a6,
                                    LPCWSTR a7,
                                    LPSTARTUPINFOW a8,
                                    LPPROCESS_INFORMATION a9)
    = CreateProcessW;

#if _MSC_VER < 1300
SOCKET (WINAPI * Real_WSAAccept)(SOCKET a0,
                                 sockaddr* a1,
                                 LPINT a2,
                                 LPCONDITIONPROC a3,
                                 DWORD a4)
    = WSAAccept;
#else
SOCKET (WINAPI * Real_WSAAccept)(SOCKET a0,
                                 sockaddr* a1,
                                 LPINT a2,
                                 LPCONDITIONPROC a3,
                                 DWORD_PTR a4)
    = WSAAccept;
#endif

INT (WINAPI * Real_WSAAddressToStringA)(LPSOCKADDR a0,
                                        DWORD a1,
                                        LPWSAPROTOCOL_INFOA a2,
                                        LPSTR a3,
                                        LPDWORD a4)
    = WSAAddressToStringA;

INT (WINAPI * Real_WSAAddressToStringW)(LPSOCKADDR a0,
                                        DWORD a1,
                                        LPWSAPROTOCOL_INFOW a2,
                                        LPWSTR a3,
                                        LPDWORD a4)
    = WSAAddressToStringW;

HANDLE (WINAPI * Real_WSAAsyncGetHostByAddr)(HWND a0,
                                             u_int a1,
                                             CONST char* a2,
                                             int a3,
                                             int a4,
                                             char* a5,
                                             int a6)
    = WSAAsyncGetHostByAddr;

HANDLE (WINAPI * Real_WSAAsyncGetHostByName)(HWND a0,
                                             u_int a1,
                                             CONST char* a2,
                                             char* a3,
                                             int a4)
    = WSAAsyncGetHostByName;

HANDLE (WINAPI * Real_WSAAsyncGetProtoByName)(HWND a0,
                                              u_int a1,
                                              CONST char* a2,
                                              char* a3,
                                              int a4)
    = WSAAsyncGetProtoByName;

HANDLE (WINAPI * Real_WSAAsyncGetProtoByNumber)(HWND a0,
                                                u_int a1,
                                                int a2,
                                                char* a3,
                                                int a4)
    = WSAAsyncGetProtoByNumber;

HANDLE (WINAPI * Real_WSAAsyncGetServByName)(HWND a0,
                                             u_int a1,
                                             CONST char* a2,
                                             CONST char* a3,
                                             char* a4,
                                             int a5)
    = WSAAsyncGetServByName;

HANDLE (WINAPI * Real_WSAAsyncGetServByPort)(HWND a0,
                                             u_int a1,
                                             int a2,
                                             CONST char* a3,
                                             char* a4,
                                             int a5)
    = WSAAsyncGetServByPort;

int (WINAPI * Real_WSACancelAsyncRequest)(HANDLE a0)
    = WSACancelAsyncRequest;

int (WINAPI * Real_WSACancelBlockingCall)(void)
    = WSACancelBlockingCall;

int (WINAPI * Real_WSACleanup)(void)
    = WSACleanup;

BOOL (WINAPI * Real_WSACloseEvent)(HANDLE a0)
    = WSACloseEvent;

int (WINAPI * Real_WSAConnect)(SOCKET a0,
                               CONST sockaddr* a1,
                               int a2,
                               LPWSABUF a3,
                               LPWSABUF a4,
                               LPQOS a5,
                               LPQOS a6)
    = WSAConnect;

HANDLE (WINAPI * Real_WSACreateEvent)(void)
    = WSACreateEvent;

int (WINAPI * Real_WSADuplicateSocketA)(SOCKET a0,
                                        DWORD a1,
                                        LPWSAPROTOCOL_INFOA a2)
    = WSADuplicateSocketA;

int (WINAPI * Real_WSADuplicateSocketW)(SOCKET a0,
                                        DWORD a1,
                                        LPWSAPROTOCOL_INFOW a2)
    = WSADuplicateSocketW;

INT (WINAPI * Real_WSAEnumNameSpaceProvidersA)(LPDWORD a0,
                                               LPWSANAMESPACE_INFOA a1)
    = WSAEnumNameSpaceProvidersA;

INT (WINAPI * Real_WSAEnumNameSpaceProvidersW)(LPDWORD a0,
                                               LPWSANAMESPACE_INFOW a1)
    = WSAEnumNameSpaceProvidersW;

int (WINAPI * Real_WSAEnumNetworkEvents)(SOCKET a0,
                                         HANDLE a1,
                                         LPWSANETWORKEVENTS a2)
    = WSAEnumNetworkEvents;

int (WINAPI * Real_WSAEnumProtocolsA)(LPINT a0,
                                      LPWSAPROTOCOL_INFOA a1,
                                      LPDWORD a2)
    = WSAEnumProtocolsA;

int (WINAPI * Real_WSAEnumProtocolsW)(LPINT a0,
                                      LPWSAPROTOCOL_INFOW a1,
                                      LPDWORD a2)
    = WSAEnumProtocolsW;

BOOL (WINAPI * Real_WSAGetOverlappedResult)(SOCKET a0,
                                            LPWSAOVERLAPPED a1,
                                            LPDWORD a2,
                                            BOOL a3,
                                            LPDWORD a4)
    = WSAGetOverlappedResult;

BOOL (WINAPI * Real_WSAIsBlocking)(void)
    = WSAIsBlocking;

SOCKET (WINAPI * Real_WSAJoinLeaf)(SOCKET a0,
                                   CONST sockaddr* a1,
                                   int a2,
                                   LPWSABUF a3,
                                   LPWSABUF a4,
                                   LPQOS a5,
                                   LPQOS a6,
                                   DWORD a7)
    = WSAJoinLeaf;

INT (WINAPI * Real_WSALookupServiceBeginA)(
                                           LPWSAQUERYSETA lpqsRestrictions,
                                           DWORD          dwControlFlags,
                                           LPHANDLE       lphLookup
                                          )
    = WSALookupServiceBeginA;

INT (WINAPI * Real_WSALookupServiceBeginW)(
                                           LPWSAQUERYSETW lpqsRestrictions,
                                           DWORD          dwControlFlags,
                                           LPHANDLE       lphLookup
                                          )
    = WSALookupServiceBeginW;

INT (WINAPI * Real_WSALookupServiceNextA)(
                                          HANDLE           hLookup,
                                          DWORD            dwControlFlags,
                                          LPDWORD       lpdwBufferLength,
                                          LPWSAQUERYSETA lpqsResults
                                         ) = WSALookupServiceNextA;

INT (WINAPI * Real_WSALookupServiceNextW)(
                                          HANDLE         hLookup,
                                          DWORD          dwControlFlags,
                                          LPDWORD        lpdwBufferLength,
                                          LPWSAQUERYSETW lpqsResults
                                         ) = WSALookupServiceNextW;

INT (WINAPI * Real_WSALookupServiceEnd)(
                                        HANDLE  hLookup
                                       ) = WSALookupServiceEnd;

INT (WINAPI * Real_WSAProviderConfigChange)(LPHANDLE a0,
                                            LPWSAOVERLAPPED a1,
                                            LPWSAOVERLAPPED_COMPLETION_ROUTINE a2)
    = WSAProviderConfigChange;

int (WINAPI * Real_WSARecv)(SOCKET a0,
                            LPWSABUF a1,
                            DWORD a2,
                            LPDWORD a3,
                            LPDWORD a4,
                            LPWSAOVERLAPPED a5,
                            LPWSAOVERLAPPED_COMPLETION_ROUTINE a6)
    = WSARecv;

int (WINAPI * Real_WSARecvDisconnect)(SOCKET a0,
                                      LPWSABUF a1)
    = WSARecvDisconnect;

int (WINAPI * Real_WSARecvFrom)(SOCKET a0,
                                LPWSABUF a1,
                                DWORD a2,
                                LPDWORD a3,
                                LPDWORD a4,
                                sockaddr* a5,
                                LPINT a6,
                                LPWSAOVERLAPPED a7,
                                LPWSAOVERLAPPED_COMPLETION_ROUTINE a8)
    = WSARecvFrom;

BOOL (WINAPI * Real_WSAResetEvent)(HANDLE a0)
    = WSAResetEvent;

int (WINAPI * Real_WSASend)(SOCKET a0,
                            LPWSABUF a1,
                            DWORD a2,
                            LPDWORD a3,
                            DWORD a4,
                            LPWSAOVERLAPPED a5,
                            LPWSAOVERLAPPED_COMPLETION_ROUTINE a6)
    = WSASend;

int (WINAPI * Real_WSASendDisconnect)(SOCKET a0,
                                      LPWSABUF a1)
    = WSASendDisconnect;

int (WINAPI * Real_WSASendTo)(SOCKET a0,
                              LPWSABUF a1,
                              DWORD a2,
                              LPDWORD a3,
                              DWORD a4,
                              CONST sockaddr* a5,
                              int a6,
                              LPWSAOVERLAPPED a7,
                              LPWSAOVERLAPPED_COMPLETION_ROUTINE a8)
    = WSASendTo;

INT (WINAPI * Real_WSAStringToAddressA)(LPSTR a0,
                                        INT a1,
                                        LPWSAPROTOCOL_INFOA a2,
                                        LPSOCKADDR a3,
                                        LPINT a4)
    = WSAStringToAddressA;

INT (WINAPI * Real_WSAStringToAddressW)(LPWSTR a0,
                                        INT a1,
                                        LPWSAPROTOCOL_INFOW a2,
                                        LPSOCKADDR a3,
                                        LPINT a4)
    = WSAStringToAddressW;

DWORD (WINAPI * Real_WSAWaitForMultipleEvents)(DWORD a0,
                                               CONST HANDLE * a1,
                                               BOOL a2,
                                               DWORD a3,
                                               BOOL a4)
    = WSAWaitForMultipleEvents;

SOCKET (WINAPI * Real_accept)(SOCKET a0,
                              sockaddr* a1,
                              int* a2)
    = accept;

int (WINAPI * Real_closesocket)(SOCKET a0)
    = closesocket;

int (WINAPI * Real_connect)(SOCKET a0,
                            CONST sockaddr* a1,
                            int a2)
    = connect;

int (WINAPI * Real_listen)(SOCKET a0,
                           int a1)
    = listen;

int (WINAPI * Real_recv)(SOCKET a0,
                         char* a1,
                         int a2,
                         int a3)
    = recv;

int (WINAPI * Real_recvfrom)(SOCKET a0,
                             char* a1,
                             int a2,
                             int a3,
                             sockaddr* a4,
                             int* a5)
    = recvfrom;

int (WINAPI * Real_send)(SOCKET a0,
                         CONST char* a1,
                         int a2,
                         int a3)
    = send;

int (WINAPI * Real_sendto)(SOCKET a0,
                           CONST char* a1,
                           int a2,
                           int a3,
                           CONST sockaddr* a4,
                           int a5)
    = sendto;

int (WINAPI * Real_shutdown)(SOCKET a0,
                             int a1)
    = shutdown;

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

    BOOL rv = 0;
    __try {
        rv = Real_CreateProcessW(lpApplicationName,
                                 lpCommandLine,
                                 lpProcessAttributes,
                                 lpThreadAttributes,
                                 bInheritHandles,
                                 dwCreationFlags,
                                 lpEnvironment,
                                 lpCurrentDirectory,
                                 lpStartupInfo,
                                 lpProcessInformation);
    } __finally {
        _PrintExit("CreateProcessW(,,,,,,,,,) -> %x\n", rv);
    };
    return rv;
}

#if _MSC_VER < 1300
SOCKET WINAPI Mine_WSAAccept(SOCKET a0,
                             sockaddr* a1,
                             LPINT a2,
                             LPCONDITIONPROC a3,
                             DWORD a4)
#else
SOCKET WINAPI Mine_WSAAccept(SOCKET a0,
                             sockaddr* a1,
                             LPINT a2,
                             LPCONDITIONPROC a3,
                             DWORD_PTR a4)
#endif
{

    SOCKET rv = 0;
    __try {
        rv = Real_WSAAccept(a0, a1, a2, a3, a4);
    } __finally {
        _PrintEnter("%p: WSAAccept(,%p,%p,%p,%p) -> %p\n", a0, a1, a2, a3, a4, rv);
        _PrintExit(NULL);
    };
    return rv;
}

INT WINAPI Mine_WSAAddressToStringA(LPSOCKADDR a0,
                                    DWORD a1,
                                    LPWSAPROTOCOL_INFOA a2,
                                    LPSTR a3,
                                    LPDWORD a4)
{
    _PrintEnter("WSAAddressToStringA(%p,%x,%p,%p,%p)\n", a0, a1, a2, a3, a4);

    INT rv = 0;
    __try {
        __try {
            sockaddr_in *in = (sockaddr_in *)a0;
            _Print("in=%d.%d.%d.%d\n",
                   in->sin_addr.S_un.S_un_b.s_b1,
                   in->sin_addr.S_un.S_un_b.s_b2,
                   in->sin_addr.S_un.S_un_b.s_b3,
                   in->sin_addr.S_un.S_un_b.s_b4);
        }
        __finally {
        }

        rv = Real_WSAAddressToStringA(a0, a1, a2, a3, a4);
    } __finally {
        _PrintExit("WSAAddressToStringA(,,,,) -> %x\n", rv);
    };
    return rv;
}

INT WINAPI Mine_WSAAddressToStringW(LPSOCKADDR a0,
                                    DWORD a1,
                                    LPWSAPROTOCOL_INFOW a2,
                                    LPWSTR a3,
                                    LPDWORD a4)
{
    _PrintEnter("WSAAddressToStringW(%p,%x,%p,%p,%p)\n", a0, a1, a2, a3, a4);

    INT rv = 0;
    __try {
        __try {
            sockaddr_in *in = (sockaddr_in *)a0;
            _Print("in=%d.%d.%d.%d\n",
                   in->sin_addr.S_un.S_un_b.s_b1,
                   in->sin_addr.S_un.S_un_b.s_b2,
                   in->sin_addr.S_un.S_un_b.s_b3,
                   in->sin_addr.S_un.S_un_b.s_b4);
        }
        __finally {
        }
        rv = Real_WSAAddressToStringW(a0, a1, a2, a3, a4);
    } __finally {
        _PrintExit("WSAAddressToStringW(,,,,) -> %x\n", rv);
    };
    return rv;
}

HANDLE WINAPI Mine_WSAAsyncGetHostByAddr(HWND a0,
                                         u_int a1,
                                         char* a2,
                                         int a3,
                                         int a4,
                                         char* a5,
                                         int a6)
{
    _PrintEnter("WSAAsyncGetHostByAddr(%p,%x,%p,%x,%x,%p,%x)\n", a0, a1, a2, a3, a4, a5, a6);

    HANDLE rv = 0;
    __try {
        rv = Real_WSAAsyncGetHostByAddr(a0, a1, a2, a3, a4, a5, a6);
    } __finally {
        _PrintExit("WSAAsyncGetHostByAddr(,,,,,,) -> %p\n", rv);
    };
    return rv;
}

HANDLE WINAPI Mine_WSAAsyncGetHostByName(HWND a0,
                                         u_int a1,
                                         char* a2,
                                         char* a3,
                                         int a4)
{
    _PrintEnter("WSAAsyncGetHostByName(%p,%x,%p,%p,%x)\n", a0, a1, a2, a3, a4);

    HANDLE rv = 0;
    __try {
        rv = Real_WSAAsyncGetHostByName(a0, a1, a2, a3, a4);
    } __finally {
        _PrintExit("WSAAsyncGetHostByName(,,,,) -> %p\n", rv);
    };
    return rv;
}

HANDLE WINAPI Mine_WSAAsyncGetProtoByName(HWND a0,
                                          u_int a1,
                                          char* a2,
                                          char* a3,
                                          int a4)
{
    _PrintEnter("WSAAsyncGetProtoByName(%p,%x,%p,%p,%x)\n", a0, a1, a2, a3, a4);

    HANDLE rv = 0;
    __try {
        rv = Real_WSAAsyncGetProtoByName(a0, a1, a2, a3, a4);
    } __finally {
        _PrintExit("WSAAsyncGetProtoByName(,,,,) -> %p\n", rv);
    };
    return rv;
}

HANDLE WINAPI Mine_WSAAsyncGetProtoByNumber(HWND a0,
                                            u_int a1,
                                            int a2,
                                            char* a3,
                                            int a4)
{
    _PrintEnter("WSAAsyncGetProtoByNumber(%p,%x,%p,%p,%x)\n", a0, a1, a2, a3, a4);

    HANDLE rv = 0;
    __try {
        rv = Real_WSAAsyncGetProtoByNumber(a0, a1, a2, a3, a4);
    } __finally {
        _PrintExit("WSAAsyncGetProtoByNumber(,,,,) -> %p\n", rv);
    };
    return rv;
}

HANDLE WINAPI Mine_WSAAsyncGetServByName(HWND a0,
                                         u_int a1,
                                         char* a2,
                                         char* a3,
                                         char* a4,
                                         int a5)
{
    _PrintEnter("WSAAsyncGetServByName(%p,%x,%p,%p,%p,%x)\n", a0, a1, a2, a3, a4, a5);

    HANDLE rv = 0;
    __try {
        rv = Real_WSAAsyncGetServByName(a0, a1, a2, a3, a4, a5);
    } __finally {
        _PrintExit("WSAAsyncGetServByName(,,,,,) -> %p\n", rv);
    };
    return rv;
}

HANDLE WINAPI Mine_WSAAsyncGetServByPort(HWND a0,
                                         u_int a1,
                                         int a2,
                                         char* a3,
                                         char* a4,
                                         int a5)
{
    _PrintEnter("WSAAsyncGetServByPort(%p,%x,%x,%p,%p,%x)\n", a0, a1, a2, a3, a4, a5);

    HANDLE rv = 0;
    __try {
        rv = Real_WSAAsyncGetServByPort(a0, a1, a2, a3, a4, a5);
    } __finally {
        _PrintExit("WSAAsyncGetServByPort(,,,,,) -> %p\n", rv);
    };
    return rv;
}

int WINAPI Mine_WSACancelAsyncRequest(HANDLE a0)
{
    _PrintEnter("WSACancelAsyncRequest(%p)\n", a0);

    int rv = 0;
    __try {
        rv = Real_WSACancelAsyncRequest(a0);
    } __finally {
        _PrintExit("WSACancelAsyncRequest() -> %x\n", rv);
    };
    return rv;
}

int WINAPI Mine_WSACancelBlockingCall(void)
{
    _PrintEnter("WSACancelBlockingCall()\n");

    int rv = 0;
    __try {
        rv = Real_WSACancelBlockingCall();
    } __finally {
        _PrintExit("WSACancelBlockingCall() -> %x\n", rv);
    };
    return rv;
}

int WINAPI Mine_WSACleanup(void)
{
    _PrintEnter("WSACleanup()\n");

    int rv = 0;
    __try {
        rv = Real_WSACleanup();
    } __finally {
        _PrintExit("WSACleanup() -> %x\n", rv);
    };
    return rv;
}

BOOL WINAPI Mine_WSACloseEvent(HANDLE a0)
{
    _PrintEnter("WSACloseEvent(%p)\n", a0);

    BOOL rv = 0;
    __try {
        rv = Real_WSACloseEvent(a0);
    } __finally {
        _PrintExit("WSACloseEvent() -> %x\n", rv);
    };
    return rv;
}

int WINAPI Mine_WSAConnect(SOCKET a0,
                           sockaddr* a1,
                           int a2,
                           LPWSABUF a3,
                           LPWSABUF a4,
                           LPQOS a5,
                           LPQOS a6)
{
    int rv = 0;
    __try {
        rv = Real_WSAConnect(a0, a1, a2, a3, a4, a5, a6);
    } __finally {
        _PrintEnter("%p: WSAConnect(,%p,%x,%p,%p,%p,%p) -> %x\n",
                    a0, a1, a2, a3, a4, a5, a6, rv);
        _PrintExit(NULL);
    };
    return rv;
}

HANDLE WINAPI Mine_WSACreateEvent(void)
{
    _PrintEnter("WSACreateEvent()\n");

    HANDLE rv = 0;
    __try {
        rv = Real_WSACreateEvent();
    } __finally {
        _PrintExit("WSACreateEvent() -> %p\n", rv);
    };
    return rv;
}

int WINAPI Mine_WSADuplicateSocketA(SOCKET a0,
                                    DWORD a1,
                                    LPWSAPROTOCOL_INFOA a2)
{
    _PrintEnter("%p: WSADuplicateSocketA(,%x,%p)\n", a0, a1, a2);

    int rv = 0;
    __try {
        rv = Real_WSADuplicateSocketA(a0, a1, a2);
    } __finally {
        _PrintExit("%p: WSADuplicateSocketA(,,) -> %x\n", a0, rv);
    };
    return rv;
}

int WINAPI Mine_WSADuplicateSocketW(SOCKET a0,
                                    DWORD a1,
                                    LPWSAPROTOCOL_INFOW a2)
{
    _PrintEnter("%p: WSADuplicateSocketW(,%x,%p)\n", a0, a1, a2);

    int rv = 0;
    __try {
        rv = Real_WSADuplicateSocketW(a0, a1, a2);
    } __finally {
        _PrintExit("%p: WSADuplicateSocketW(,,) -> %x\n", a0, rv);
    };
    return rv;
}

INT WINAPI Mine_WSAEnumNameSpaceProvidersA(LPDWORD a0,
                                           LPWSANAMESPACE_INFOA a1)
{
    _PrintEnter("WSAEnumNameSpaceProvidersA(%p,%p)\n", a0, a1);

    INT rv = 0;
    __try {
        rv = Real_WSAEnumNameSpaceProvidersA(a0, a1);
    } __finally {
        _PrintExit("WSAEnumNameSpaceProvidersA(,) -> %x\n", rv);
    };
    return rv;
}

INT WINAPI Mine_WSAEnumNameSpaceProvidersW(LPDWORD a0,
                                           LPWSANAMESPACE_INFOW a1)
{
    _PrintEnter("WSAEnumNameSpaceProvidersW(%p,%p)\n", a0, a1);

    INT rv = 0;
    __try {
        rv = Real_WSAEnumNameSpaceProvidersW(a0, a1);
    } __finally {
        _PrintExit("WSAEnumNameSpaceProvidersW(,) -> %x\n", rv);
    };
    return rv;
}

int WINAPI Mine_WSAEnumNetworkEvents(SOCKET a0,
                                     HANDLE a1,
                                     LPWSANETWORKEVENTS a2)
{
    _PrintEnter("%p: WSAEnumNetworkEvents(,%p,%p)\n", a0, a1, a2);

    int rv = 0;
    __try {
        rv = Real_WSAEnumNetworkEvents(a0, a1, a2);
    } __finally {
        _PrintExit("%p: WSAEnumNetworkEvents(,,) -> %x\n", a0, rv);
    };
    return rv;
}

int WINAPI Mine_WSAEnumProtocolsA(LPINT a0,
                                  LPWSAPROTOCOL_INFOA a1,
                                  LPDWORD a2)
{
    _PrintEnter("WSAEnumProtocolsA(%p,%p,%p)\n", a0, a1, a2);

    int rv = 0;
    __try {
        rv = Real_WSAEnumProtocolsA(a0, a1, a2);
    } __finally {
        _PrintExit("WSAEnumProtocolsA(,,) -> %x\n", rv);
    };
    return rv;
}

int WINAPI Mine_WSAEnumProtocolsW(LPINT a0,
                                  LPWSAPROTOCOL_INFOW a1,
                                  LPDWORD a2)
{
    _PrintEnter("WSAEnumProtocolsW(%p,%p,%p)\n", a0, a1, a2);

    int rv = 0;
    __try {
        rv = Real_WSAEnumProtocolsW(a0, a1, a2);
    } __finally {
        _PrintExit("WSAEnumProtocolsW(,,) -> %x\n", rv);
    };
    return rv;
}

BOOL WINAPI Mine_WSAGetOverlappedResult(SOCKET a0,
                                        LPWSAOVERLAPPED a1,
                                        LPDWORD a2,
                                        BOOL a3,
                                        LPDWORD a4)
{
    _PrintEnter("%p: WSAGetOverlappedResult(,%p,%p,%x,%p)\n", a0, a1, a2, a3, a4);

    BOOL rv = 0;
    __try {
        rv = Real_WSAGetOverlappedResult(a0, a1, a2, a3, a4);
    } __finally {
        _PrintExit("%p: WSAGetOverlappedResult(,,,,) -> %x\n", a0, rv);
    };
    return rv;
}

BOOL WINAPI Mine_WSAIsBlocking(void)
{
    _PrintEnter("WSAIsBlocking()\n");

    BOOL rv = 0;
    __try {
        rv = Real_WSAIsBlocking();
    } __finally {
        _PrintExit("WSAIsBlocking() -> %x\n", rv);
    };
    return rv;
}

SOCKET WINAPI Mine_WSAJoinLeaf(SOCKET a0,
                               sockaddr* a1,
                               int a2,
                               LPWSABUF a3,
                               LPWSABUF a4,
                               LPQOS a5,
                               LPQOS a6,
                               DWORD a7)
{
    _PrintEnter("%p: WSAJoinLeaf(,%p,%x,%p,%p,%p,%p,%x)\n", a0, a1, a2, a3, a4, a5, a6, a7);

    SOCKET rv = 0;
    __try {
        rv = Real_WSAJoinLeaf(a0, a1, a2, a3, a4, a5, a6, a7);
    } __finally {
        _PrintExit("%p: WSAJoinLeaf(,,,,,,,) -> %p\n", a0, rv);
    };
    return rv;
}

INT WINAPI Mine_WSALookupServiceBeginA(
                                       LPWSAQUERYSETA lpqsRestrictions,
                                       DWORD          dwControlFlags,
                                       LPHANDLE       lphLookup
                                      )
{
    _PrintEnter("WSALookupServiceBeginA(%p,%x,%p)\n",
                lpqsRestrictions,
                dwControlFlags,
                lphLookup);

    INT rv = 0;
    __try {
        __try {
            _Print("service: [%hs]", lpqsRestrictions->lpszServiceInstanceName);
            _Print("queries: [%hs]", lpqsRestrictions->lpszQueryString);
        }
        __finally {
        }

        rv = Real_WSALookupServiceBeginA(
                                          lpqsRestrictions,
                                          dwControlFlags,
                                          lphLookup);
    } __finally {
        _PrintExit("WSALookupServiceBeginA(,,) -> %x\n", rv);
    };
    return rv;

}

INT WINAPI Mine_WSALookupServiceBeginW(
                                       LPWSAQUERYSETW lpqsRestrictions,
                                       DWORD          dwControlFlags,
                                       LPHANDLE       lphLookup
                               )
{
    _PrintEnter("WSALookupServiceBeginW(%p,%p,%p)\n",
                lpqsRestrictions,
                dwControlFlags,
                lphLookup);
    __debugbreak();

    INT rv = 0;
    __try {
        __try {
            _Print("service: [%ls]", lpqsRestrictions->lpszServiceInstanceName);
            _Print("queries: [%ls]", lpqsRestrictions->lpszQueryString);
        }
        __finally {
        }

        rv = Real_WSALookupServiceBeginW(
                                         lpqsRestrictions,
                                         dwControlFlags,
                                         lphLookup);
    } __finally {
        _PrintExit("WSALookupServiceBeginW(,,) -> %x\n", rv);
    };
    return rv;
}

INT WINAPI Mine_WSALookupServiceNextA(
                               HANDLE           hLookup,
                               DWORD            dwControlFlags,
                               LPDWORD       lpdwBufferLength,
                               LPWSAQUERYSETA lpqsResults
                              )
{
    _PrintEnter("WSALookupServiceNextA(%p,%p,%p,%p)\n",
                hLookup,
                dwControlFlags,
                lpdwBufferLength,
                lpqsResults);

    INT rv = 0;
    __try {
        rv = Real_WSALookupServiceNextA(
                                        hLookup,
                                        dwControlFlags,
                                        lpdwBufferLength,
                                        lpqsResults);
    } __finally {
        _PrintExit("WSALookupServiceNextA(,,) -> %x\n", rv);
    };
    return rv;
}

INT WINAPI Mine_WSALookupServiceNextW(
                               HANDLE         hLookup,
                               DWORD          dwControlFlags,
                               LPDWORD        lpdwBufferLength,
                               LPWSAQUERYSETW lpqsResults
                              )
{
    _PrintEnter("WSALookupServiceNextW(%p,%p,%p,%p)\n",
                hLookup,
                dwControlFlags,
                lpdwBufferLength,
                lpqsResults);

    INT rv = 0;
    __try {
        rv = Real_WSALookupServiceNextW(
                                        hLookup,
                                        dwControlFlags,
                                        lpdwBufferLength,
                                        lpqsResults);
    } __finally {
        _PrintExit("WSALookupServiceNextW(,,) -> %x\n", rv);
    };
    return rv;
}

INT WINAPI Mine_WSALookupServiceEnd(
                                        HANDLE  hLookup
                                    )
{
    _PrintEnter("WSALookupServiceEnd(%p)\n", hLookup);

    INT rv = 0;
    __try {
        rv = Real_WSALookupServiceEnd(hLookup);
    } __finally {
        _PrintExit("WSALookupServiceEnd(,,) -> %x\n", rv);
    };
    return rv;
}

INT WINAPI Mine_WSAProviderConfigChange(LPHANDLE a0,
                                        LPWSAOVERLAPPED a1,
                                        LPWSAOVERLAPPED_COMPLETION_ROUTINE a2)
{
    _PrintEnter("WSAProviderConfigChange(%p,%p,%p)\n", a0, a1, a2);

    INT rv = 0;
    __try {
        rv = Real_WSAProviderConfigChange(a0, a1, a2);
    } __finally {
        _PrintExit("WSAProviderConfigChange(,,) -> %x\n", rv);
    };
    return rv;
}

int WINAPI Mine_WSARecv(SOCKET a0,
                        LPWSABUF a1,
                        DWORD a2,
                        LPDWORD a3,
                        LPDWORD a4,
                        LPWSAOVERLAPPED a5,
                        LPWSAOVERLAPPED_COMPLETION_ROUTINE a6)
{
    int rv = -1;
    __try {
        rv = Real_WSARecv(a0, a1, a2, a3, a4, a5, a6);
    } __finally {
        if (rv == 0) {
            _PrintEnter("%p: WSARecv(,%p,%x,%p,%p,%p,%p)\n",
                        a0, a1, a2, a3, a4, a5, a6);
            _PrintDump(a0, a1[0].buf, a1[0].len < *a3 ? a1[0].len : *a3);
            _PrintExit("%p: WSARecv(,,,,,,) -> %x\n", a0, rv);
        }
    };
    return rv;
}

int WINAPI Mine_WSARecvDisconnect(SOCKET a0,
                                  LPWSABUF a1)
{
    _PrintEnter("%p: WSARecvDisconnect(,%p)\n", a0, a1);

    int rv = 0;
    __try {
        rv = Real_WSARecvDisconnect(a0, a1);
    } __finally {
        _PrintExit("%p: WSARecvDisconnect(,) -> %x\n", a0, rv);
    };
    return rv;
}

int WINAPI Mine_WSARecvFrom(SOCKET a0,
                            LPWSABUF a1,
                            DWORD a2,
                            LPDWORD a3,
                            LPDWORD a4,
                            sockaddr* a5,
                            LPINT a6,
                            LPWSAOVERLAPPED a7,
                            LPWSAOVERLAPPED_COMPLETION_ROUTINE a8)
{
    _PrintEnter("%p: WSARecvFrom(,%p,%x,%p,%p,%p,%p,%p,%p)\n", a0, a1, a2, a3, a4, a5, a6, a7, a8);

    int rv = 0;
    __try {
        rv = Real_WSARecvFrom(a0, a1, a2, a3, a4, a5, a6, a7, a8);
    } __finally {
        _PrintExit("%p: WSARecvFrom(,,,,,,,,) -> %x\n", a0, rv);
    };
    return rv;
}

BOOL WINAPI Mine_WSAResetEvent(HANDLE a0)
{
    _PrintEnter("WSAResetEvent(%p)\n", a0);

    BOOL rv = 0;
    __try {
        rv = Real_WSAResetEvent(a0);
    } __finally {
        _PrintExit("WSAResetEvent() -> %x\n", rv);
    };
    return rv;
}

int WINAPI Mine_WSASend(SOCKET a0,
                        LPWSABUF a1,
                        DWORD a2,
                        LPDWORD a3,
                        DWORD a4,
                        LPWSAOVERLAPPED a5,
                        LPWSAOVERLAPPED_COMPLETION_ROUTINE a6)
{
    _PrintEnter("%p: WSASend(,%p,%x,%p,%x,%p,%p)\n", a0, a1, a2, a3, a4, a5, a6);

    int rv = 0;
    __try {
        rv = Real_WSASend(a0, a1, a2, a3, a4, a5, a6);
    } __finally {
        _PrintExit("%p: WSASend(,,,,,,) -> %x\n", a0, rv);
    };
    return rv;
}

int WINAPI Mine_WSASendDisconnect(SOCKET a0,
                                  LPWSABUF a1)
{
    _PrintEnter("%p: WSASendDisconnect(,%p)\n", a0, a1);

    int rv = 0;
    __try {
        rv = Real_WSASendDisconnect(a0, a1);
    } __finally {
        _PrintExit("%p: WSASendDisconnect(,) -> %x\n", a0, rv);
    };
    return rv;
}

int WINAPI Mine_WSASendTo(SOCKET a0,
                          LPWSABUF a1,
                          DWORD a2,
                          LPDWORD a3,
                          DWORD a4,
                          sockaddr* a5,
                          int a6,
                          LPWSAOVERLAPPED a7,
                          LPWSAOVERLAPPED_COMPLETION_ROUTINE a8)
{
    _PrintEnter("%p: WSASendTo(,%p,%x,%p,%x,%p,%x,%p,%p)\n", a0, a1, a2, a3, a4, a5, a6, a7, a8);

    int rv = 0;
    __try {
        rv = Real_WSASendTo(a0, a1, a2, a3, a4, a5, a6, a7, a8);
    } __finally {
        _PrintExit("%p: WSASendTo(,,,,,,,,) -> %x\n", a0, rv);
    };
    return rv;
}

INT WINAPI Mine_WSAStringToAddressA(LPSTR a0,
                                    INT a1,
                                    LPWSAPROTOCOL_INFOA a2,
                                    LPSOCKADDR a3,
                                    LPINT a4)
{
    _PrintEnter("WSAStringToAddressA(%p,%x,%p,%p,%p)\n", a0, a1, a2, a3, a4);

    INT rv = 0;
    __try {
        rv = Real_WSAStringToAddressA(a0, a1, a2, a3, a4);
    } __finally {
        _PrintExit("WSAStringToAddressA(,,,,) -> %x\n", rv);
    };
    return rv;
}

INT WINAPI Mine_WSAStringToAddressW(LPWSTR a0,
                                    INT a1,
                                    LPWSAPROTOCOL_INFOW a2,
                                    LPSOCKADDR a3,
                                    LPINT a4)
{
    _PrintEnter("WSAStringToAddressW(%p,%x,%p,%p,%p)\n", a0, a1, a2, a3, a4);

    INT rv = 0;
    __try {
        rv = Real_WSAStringToAddressW(a0, a1, a2, a3, a4);
    } __finally {
        _PrintExit("WSAStringToAddressW(,,,,) -> %x\n", rv);
    };
    return rv;
}

DWORD WINAPI Mine_WSAWaitForMultipleEvents(DWORD a0,
                                           void** a1,
                                           BOOL a2,
                                           DWORD a3,
                                           BOOL a4)
{
    _PrintEnter("WSAWaitForMultipleEvents(%x,%p,%x,%x,%x)\n", a0, a1, a2, a3, a4);

    DWORD rv = 0;
    __try {
        rv = Real_WSAWaitForMultipleEvents(a0, a1, a2, a3, a4);
    } __finally {
        _PrintExit("WSAWaitForMultipleEvents(,,,,) -> %x\n", rv);
    };
    return rv;
}

SOCKET WINAPI Mine_accept(SOCKET a0,
                          sockaddr* a1,
                          int* a2)
{
    SOCKET rv = 0;
    __try {
        rv = Real_accept(a0, a1, a2);
    } __finally {
        WCHAR wzAddress[512] = L"";
        int err = WSAGetLastError();
        if (rv != INVALID_SOCKET) {
            DWORD nAddress = ARRAYSIZE(wzAddress);
            if (Real_WSAAddressToStringW(a1, *a2, NULL, wzAddress, &nAddress) != 0) {
                wzAddress[0] = 0;
            }
        }
        WSASetLastError(err);

        if (wzAddress[0]) {
            _PrintEnter("%p: accept(,%ls,%p) -> %p\n", a0, wzAddress, a2, rv);
        }
        else {
            _PrintEnter("%p: accept(,%p,%p) -> %p\n", a0, a1, a2, rv);
        }
        _PrintExit(NULL);
    };
    return rv;
}

int WINAPI Mine_closesocket(SOCKET a0)
{
    int rv = 0;
    __try {
        rv = Real_closesocket(a0);
    } __finally {
        _PrintEnter("%p: closesocket() -> %x\n", a0, rv);
        _PrintExit(NULL);
    };
    return rv;
}

int WINAPI Mine_connect(SOCKET a0,
                        sockaddr* name,
                        int namelen)
{
    int rv = 0;
    __try {
        rv = Real_connect(a0, name, namelen);
    } __finally {
        WCHAR wzAddress[512];
        DWORD nAddress = ARRAYSIZE(wzAddress);
        int err = WSAGetLastError();
        if (Real_WSAAddressToStringW(name, namelen, NULL, wzAddress, &nAddress) == 0) {
            if (rv == SOCKET_ERROR) {
                _PrintEnter("%p: connect(,%p:%ls,%x) -> %x [%d]\n",
                            a0, name, wzAddress, namelen, rv, err);
            }
            else {
                _PrintEnter("%p: connect(,%p:%ls,%x) -> %x\n",
                            a0, name, wzAddress, namelen, rv);
            }
        }
        else {
            if (rv == SOCKET_ERROR) {
                _PrintEnter("%p: connect(,%p,%x) -> %x [%d]\n",
                            a0, name, namelen, rv, err);
            }
            else {
                _PrintEnter("%p: connect(,%p,%x) -> %x\n",
                            a0, name, namelen, rv);
            }
        }
        WSASetLastError(err);
        _PrintExit(NULL);
    };
    return rv;
}

int WINAPI Mine_listen(SOCKET a0,
                       int a1)
{
    _PrintEnter("%p: listen(,%x)\n", a0, a1);

    int rv = 0;
    __try {
        rv = Real_listen(a0, a1);
    } __finally {
        _PrintExit("%p: listen(,) -> %x\n", a0, rv);
    };
    return rv;
}

int WINAPI Mine_recv(SOCKET a0,
                     char* a1,
                     int a2,
                     int a3)
{
    _PrintEnter("%p: recv(,%p,%x,%x)\n", a0, a1, a2, a3);

    int rv = 0;
    __try {
        rv = Real_recv(a0, a1, a2, a3);
    } __finally {
        _PrintExit("%p: recv(,%s,,) -> %x\n", a0, a1, rv);
    };
    return rv;
}

int WINAPI Mine_recvfrom(SOCKET a0,
                         char* a1,
                         int a2,
                         int a3,
                         sockaddr* a4,
                         int* a5)
{
    _PrintEnter("%p: recvfrom(,%p,%x,%x,%p,%p)\n", a0, a1, a2, a3, a4, a5);

    int rv = 0;
    __try {
        rv = Real_recvfrom(a0, a1, a2, a3, a4, a5);
    } __finally {
        _PrintExit("%p: recvfrom(,%s,,,,) -> %x\n", a0, a1, rv);
    };
    return rv;
}

VOID _PrintDump(SOCKET socket, PCHAR pszData, INT cbData)
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
                _Print("%p:   %hs\n", socket, szBuffer);
                nLines++;
                pszBuffer = szBuffer;
                cbBuffer = 0;
                pszData++;
                cbData--;
                continue;
            }
            else if (cbBuffer >= 80) {
                *pszBuffer++ = '\0';
                _Print("%p:   %hs\n", socket, szBuffer);
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
            _Print("%p:   %hs\n", socket, szBuffer);
        }
    }
}

int WINAPI Mine_send(SOCKET a0,
                     char* a1,
                     int a2,
                     int a3)
{
    _PrintEnter("%p: send(,%p,%x,%x)\n", a0, a1, a2, a3);
    _PrintDump(a0, a1, a2);

    int rv = 0;
    __try {
        rv = Real_send(a0, a1, a2, a3);
    } __finally {
        if (rv == SOCKET_ERROR) {
            int err = WSAGetLastError();
            _PrintExit("%p: send(,,,) -> %x (%d)\n", a0, rv, err);
        }
        else {
            _PrintExit("%p: send(,,,) -> %x\n", a0, rv);
        }
    };
    return rv;
}

int WINAPI Mine_sendto(SOCKET a0,
                       char* a1,
                       int a2,
                       int a3,
                       sockaddr* a4,
                       int a5)
{
    _PrintEnter("%p: sendto(,%s,%x,%x,%p,%x)\n", a0, a1, a2, a3, a4, a5);

    int rv = 0;
    __try {
        rv = Real_sendto(a0, a1, a2, a3, a4, a5);
    } __finally {
        _PrintExit("%p: sendto(%ls,,,,,) -> %x\n", a0, a1, rv);
    };
    return rv;
}

int WINAPI Mine_shutdown(SOCKET a0,
                         int a1)
{
    _PrintEnter("%p: shutdown(,%x)\n", a0, a1);

    int rv = 0;
    __try {
        rv = Real_shutdown(a0, a1);
    } __finally {
        _PrintExit("%p: shutdown(,) -> %x\n", a0, rv);
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

    ATTACH(CreateProcessW);
    ATTACH(WSAAccept);
    ATTACH(WSAAddressToStringA);
    ATTACH(WSAAddressToStringW);
    ATTACH(WSAAsyncGetHostByAddr);
    ATTACH(WSAAsyncGetHostByName);
    ATTACH(WSAAsyncGetProtoByName);
    ATTACH(WSAAsyncGetProtoByNumber);
    ATTACH(WSAAsyncGetServByName);
    ATTACH(WSAAsyncGetServByPort);
    ATTACH(WSACancelAsyncRequest);
    ATTACH(WSACancelBlockingCall);
    ATTACH(WSACleanup);
    ATTACH(WSACloseEvent);
    ATTACH(WSAConnect);
    ATTACH(WSACreateEvent);
    ATTACH(WSADuplicateSocketA);
    ATTACH(WSADuplicateSocketW);
    ATTACH(WSAEnumNameSpaceProvidersA);
    ATTACH(WSAEnumNameSpaceProvidersW);
    ATTACH(WSAEnumNetworkEvents);
    ATTACH(WSAEnumProtocolsA);
    ATTACH(WSAEnumProtocolsW);
    ATTACH(WSAIsBlocking);
    ATTACH(WSAJoinLeaf);
    ATTACH(WSALookupServiceBeginA);
    ATTACH(WSALookupServiceBeginW);
    ATTACH(WSALookupServiceNextA);
    ATTACH(WSALookupServiceNextW);
    ATTACH(WSALookupServiceEnd);
    ATTACH(WSAProviderConfigChange);
    ATTACH(WSARecv);
    ATTACH(WSARecvDisconnect);
    ATTACH(WSARecvFrom);
    ATTACH(WSAResetEvent);
    ATTACH(WSASend);
    ATTACH(WSASendDisconnect);
    ATTACH(WSASendTo);
    ATTACH(WSAStringToAddressA);
    ATTACH(WSAStringToAddressW);
    ATTACH(WSAWaitForMultipleEvents);
    ATTACH(accept);
    ATTACH(closesocket);
    ATTACH(connect);
    ATTACH(listen);
    ATTACH(recv);
    ATTACH(recvfrom);
    ATTACH(send);
    ATTACH(sendto);
    ATTACH(shutdown);

    return DetourTransactionCommit();
}

LONG DetachDetours(VOID)
{
    DetourTransactionBegin();
    DetourUpdateThread(GetCurrentThread());

    DETACH(CreateProcessW);
    DETACH(WSAAccept);
    DETACH(WSAAddressToStringA);
    DETACH(WSAAddressToStringW);
    DETACH(WSAAsyncGetHostByAddr);
    DETACH(WSAAsyncGetHostByName);
    DETACH(WSAAsyncGetProtoByName);
    DETACH(WSAAsyncGetProtoByNumber);
    DETACH(WSAAsyncGetServByName);
    DETACH(WSAAsyncGetServByPort);
    DETACH(WSACancelAsyncRequest);
    DETACH(WSACancelBlockingCall);
    DETACH(WSACleanup);
    DETACH(WSACloseEvent);
    DETACH(WSAConnect);
    DETACH(WSACreateEvent);
    DETACH(WSADuplicateSocketA);
    DETACH(WSADuplicateSocketW);
    DETACH(WSAEnumNameSpaceProvidersA);
    DETACH(WSAEnumNameSpaceProvidersW);
    DETACH(WSAEnumNetworkEvents);
    DETACH(WSAEnumProtocolsA);
    DETACH(WSAEnumProtocolsW);
    DETACH(WSAIsBlocking);
    DETACH(WSAJoinLeaf);
    DETACH(WSALookupServiceBeginA);
    DETACH(WSALookupServiceBeginW);
    DETACH(WSALookupServiceNextA);
    DETACH(WSALookupServiceNextW);
    DETACH(WSALookupServiceEnd);
    DETACH(WSAProviderConfigChange);
    DETACH(WSARecv);
    DETACH(WSARecvDisconnect);
    DETACH(WSARecvFrom);
    DETACH(WSAResetEvent);
    DETACH(WSASend);
    DETACH(WSASendDisconnect);
    DETACH(WSASendTo);
    DETACH(WSAStringToAddressA);
    DETACH(WSAStringToAddressW);
    DETACH(WSAWaitForMultipleEvents);
    DETACH(accept);
    DETACH(closesocket);
    DETACH(connect);
    DETACH(listen);
    DETACH(recv);
    DETACH(recvfrom);
    DETACH(send);
    DETACH(sendto);
    DETACH(shutdown);

    return DetourTransactionCommit();
}

//
//////////////////////////////////////////////////////////////////////////////

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

    WCHAR wzExeName[MAX_PATH];
    s_hInst = hDll;

    Real_GetModuleFileNameW(hDll, s_wzDllPath, ARRAYSIZE(s_wzDllPath));
    Real_GetModuleFileNameW(NULL, wzExeName, ARRAYSIZE(wzExeName));

    SyelogOpen("trctcp" DETOURS_STRINGIFY(DETOURS_BITS), SYELOG_FACILITY_APPLICATION);
    Syelog(SYELOG_SEVERITY_INFORMATION,
           "##################################################################\n");
    Syelog(SYELOG_SEVERITY_INFORMATION,
           "### %ls\n", wzExeName);
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
        printf("trctcp" DETOURS_STRINGIFY(DETOURS_BITS) ".dll: Starting.\n");
        fflush(stdout);
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
