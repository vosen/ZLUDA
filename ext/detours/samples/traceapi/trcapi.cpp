//////////////////////////////////////////////////////////////////////////////
//
//  Detours Test Program (trcapi.cpp of trcapi.dll)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
#undef WIN32_LEAN_AND_MEAN
#define _WIN32_WINNT        0x400
#define WIN32
#define NT
#define _WINSOCK_DEPRECATED_NO_WARNINGS

#define DBG_TRACE   0

#if _MSC_VER >= 1300
#include <winsock2.h>
#endif
#include <windows.h>
#include <stdio.h>
#pragma warning(push)
#if _MSC_VER > 1400
#pragma warning(disable:6102 6103) // /analyze warnings
#endif
#include <strsafe.h>
#pragma warning(pop)
#include "detours.h"
#include "syelog.h"

#if (_MSC_VER < 1299)
#define LONG_PTR    LONG
#define ULONG_PTR   ULONG
#define PLONG_PTR   PLONG
#define PULONG_PTR  PULONG
#define INT_PTR     INT
#define UINT_PTR    UINT
#define PINT_PTR    PINT
#define PUINT_PTR   PUINT
#define DWORD_PTR   DWORD
#define PDWORD_PTR  PDWORD
#endif

#pragma warning(disable:4996)   // We don't care about deprecated APIs.

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

#define UNUSED(c)    (c) = (c)

//////////////////////////////////////////////////////////////////////////////
static HMODULE s_hInst = NULL;
static WCHAR s_wzDllPath[MAX_PATH];
static CHAR s_szDllPath[MAX_PATH];

BOOL ProcessEnumerate();
BOOL InstanceEnumerate(HINSTANCE hInst);

VOID _PrintEnter(const CHAR *psz, ...);
VOID _PrintExit(const CHAR *psz, ...);
VOID _Print(const CHAR *psz, ...);
VOID _VPrint(PCSTR msg, va_list args, PCHAR pszBuf, LONG cbBuf);

VOID AssertMessage(CONST PCHAR pszMsg, CONST PCHAR pszFile, ULONG nLine);

//////////////////////////////////////////////////////////////////////////////
//
// Trampolines
//
extern "C" {
    //  Trampolines for SYELOG library.
    //
    extern HANDLE (WINAPI *Real_CreateFileW)(LPCWSTR a0, DWORD a1, DWORD a2,
                                             LPSECURITY_ATTRIBUTES a3, DWORD a4, DWORD a5,
                                             HANDLE a6);
    extern BOOL (WINAPI *Real_WriteFile)(HANDLE hFile,
                                         LPCVOID lpBuffer,
                                         DWORD nNumberOfBytesToWrite,
                                         LPDWORD lpNumberOfBytesWritten,
                                         LPOVERLAPPED lpOverlapped);
    extern BOOL (WINAPI *Real_FlushFileBuffers)(HANDLE hFile);
    extern BOOL (WINAPI *Real_CloseHandle)(HANDLE hObject);
    extern BOOL (WINAPI *Real_WaitNamedPipeW)(LPCWSTR lpNamedPipeName, DWORD nTimeOut);
    extern BOOL (WINAPI *Real_SetNamedPipeHandleState)(HANDLE hNamedPipe,
                                                       LPDWORD lpMode,
                                                       LPDWORD lpMaxCollectionCount,
                                                       LPDWORD lpCollectDataTimeout);
    extern DWORD (WINAPI *Real_GetCurrentProcessId)(VOID);
    extern VOID (WINAPI *Real_GetSystemTimeAsFileTime)(LPFILETIME lpSystemTimeAsFileTime);

    VOID ( WINAPI * Real_InitializeCriticalSection)(LPCRITICAL_SECTION lpSection)
        = InitializeCriticalSection;
    VOID ( WINAPI * Real_EnterCriticalSection)(LPCRITICAL_SECTION lpSection)
        = EnterCriticalSection;
    VOID ( WINAPI * Real_LeaveCriticalSection)(LPCRITICAL_SECTION lpSection)
        = LeaveCriticalSection;
}

#include "_win32.cpp"

////////////////////////////////////////////////////////////// Logging System.
//
static BOOL s_bLog = FALSE;
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
        *pszBuf++ = '+';
        *pszBuf = '\0';

        va_list  args;
        va_start(args, psz);

        while ((*pszBuf++ = *psz++) != 0 && pszBuf < pszEnd) {
            // Copy characters.
        }
        *pszEnd = '\0';
        SyelogV(SYELOG_SEVERITY_INFORMATION, szBuf, args);

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
        ASSERT_ALWAYS(nIndent >= 0);
        TlsSetValue(s_nTlsIndent, (PVOID)(LONG_PTR)nIndent);
    }
    if (s_nTlsThread >= 0) {
        nThread = (LONG)(LONG_PTR)TlsGetValue(s_nTlsThread);
    }

    if (s_bLog && psz) {
        CHAR szBuf[1024];
        PCHAR pszEnd = szBuf + ARRAYSIZE(szBuf) - 1;
        PCHAR pszBuf = szBuf;
        LONG nLen = (nIndent > 0) ? (nIndent < 35 ? nIndent * 2 : 70) : 0;
        *pszBuf++ = (CHAR)('0' + ((nThread / 100) % 10));
        *pszBuf++ = (CHAR)('0' + ((nThread / 10) % 10));
        *pszBuf++ = (CHAR)('0' + ((nThread / 1) % 10));
        *pszBuf++ = ' ';
        while (nLen-- > 0) {
            *pszBuf++ = ' ';
        }
        *pszBuf++ = '-';
        *pszBuf = '\0';

        va_list  args;
        va_start(args, psz);

        while ((*pszBuf++ = *psz++) != 0 && pszBuf < pszEnd) {
            // Copy characters.
        }
        *pszEnd = '\0';
        SyelogV(SYELOG_SEVERITY_INFORMATION, szBuf, args);

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
        PCHAR pszEnd = szBuf + ARRAYSIZE(szBuf) - 1;
        PCHAR pszBuf = szBuf;
        LONG nLen = (nIndent > 0) ? (nIndent < 35 ? nIndent * 2 : 70) : 0;
        *pszBuf++ = (CHAR)('0' + ((nThread / 100) % 10));
        *pszBuf++ = (CHAR)('0' + ((nThread / 10) % 10));
        *pszBuf++ = (CHAR)('0' + ((nThread / 1) % 10));
        *pszBuf++ = ' ';
        while (nLen-- > 0) {
            *pszBuf++ = ' ';
        }
        *pszBuf = '\0';

        va_list  args;
        va_start(args, psz);

        while ((*pszBuf++ = *psz++) != 0 && pszBuf < pszEnd) {
            // Copy characters.
        }
        *pszEnd = '\0';
        SyelogV(SYELOG_SEVERITY_INFORMATION, szBuf, args);

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
PIMAGE_NT_HEADERS NtHeadersForInstance(HINSTANCE hInst)
{
    PIMAGE_DOS_HEADER pDosHeader = (PIMAGE_DOS_HEADER)hInst;
    __try {
        if (pDosHeader->e_magic != IMAGE_DOS_SIGNATURE) {
            SetLastError(ERROR_BAD_EXE_FORMAT);
            return NULL;
        }

        PIMAGE_NT_HEADERS pNtHeader = (PIMAGE_NT_HEADERS)((PBYTE)pDosHeader +
                                                          pDosHeader->e_lfanew);
        if (pNtHeader->Signature != IMAGE_NT_SIGNATURE) {
            SetLastError(ERROR_INVALID_EXE_SIGNATURE);
            return NULL;
        }
        if (pNtHeader->FileHeader.SizeOfOptionalHeader == 0) {
            SetLastError(ERROR_EXE_MARKED_INVALID);
            return NULL;
        }
        return pNtHeader;
    } __except(EXCEPTION_EXECUTE_HANDLER) {
    }
    SetLastError(ERROR_EXE_MARKED_INVALID);

    return NULL;
}

BOOL InstanceEnumerate(HINSTANCE hInst)
{
    WCHAR wzDllName[MAX_PATH];

    PIMAGE_NT_HEADERS pinh = NtHeadersForInstance(hInst);
    if (pinh && Real_GetModuleFileNameW(hInst, wzDllName, ARRAYSIZE(wzDllName))) {
        Syelog(SYELOG_SEVERITY_INFORMATION, "### %p: %ls\n", hInst, wzDllName);
        return TRUE;
    }
    return FALSE;
}

BOOL ProcessEnumerate()
{
    Syelog(SYELOG_SEVERITY_INFORMATION,
           "######################################################### Binaries\n");

    PBYTE pbNext;
    for (PBYTE pbRegion = (PBYTE)0x10000;; pbRegion = pbNext) {
        MEMORY_BASIC_INFORMATION mbi;
        ZeroMemory(&mbi, sizeof(mbi));

        if (VirtualQuery((PVOID)pbRegion, &mbi, sizeof(mbi)) <= 0) {
            break;
        }
        pbNext = (PBYTE)mbi.BaseAddress + mbi.RegionSize;

        // Skip free regions, reserver regions, and guard pages.
        //
        if (mbi.State == MEM_FREE || mbi.State == MEM_RESERVE) {
            continue;
        }
        if (mbi.Protect & PAGE_GUARD || mbi.Protect & PAGE_NOCACHE) {
            continue;
        }
        if (mbi.Protect == PAGE_NOACCESS) {
            continue;
        }

        // Skip over regions from the same allocation...
        {
            MEMORY_BASIC_INFORMATION mbiStep;

            while (VirtualQuery((PVOID)pbNext, &mbiStep, sizeof(mbiStep)) > 0) {
                if ((PBYTE)mbiStep.AllocationBase != pbRegion) {
                    break;
                }
                pbNext = (PBYTE)mbiStep.BaseAddress + mbiStep.RegionSize;
                mbi.Protect |= mbiStep.Protect;
            }
        }

        WCHAR wzDllName[MAX_PATH];
        PIMAGE_NT_HEADERS pinh = NtHeadersForInstance((HINSTANCE)pbRegion);

        if (pinh &&
            Real_GetModuleFileNameW((HINSTANCE)pbRegion,wzDllName,ARRAYSIZE(wzDllName))) {

            Syelog(SYELOG_SEVERITY_INFORMATION,
                   "### %p..%p: %ls\n", pbRegion, pbNext, wzDllName);
        }
        else {
            Syelog(SYELOG_SEVERITY_INFORMATION,
                   "### %p..%p: State=%04x, Protect=%08x\n",
                   pbRegion, pbNext, mbi.State, mbi.Protect);
        }
    }
    Syelog(SYELOG_SEVERITY_INFORMATION, "###\n");

    LPVOID lpvEnv = Real_GetEnvironmentStrings();
    Syelog(SYELOG_SEVERITY_INFORMATION, "### Env= %08x [%08x %08x]\n",
           lpvEnv, ((PVOID*)lpvEnv)[0], ((PVOID*)lpvEnv)[1]);

    return TRUE;
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
    ThreadAttach(hDll);

    WCHAR wzExeName[MAX_PATH];

    s_hInst = hDll;
    Real_GetModuleFileNameW(hDll, s_wzDllPath, ARRAYSIZE(s_wzDllPath));
    Real_GetModuleFileNameW(NULL, wzExeName, ARRAYSIZE(wzExeName));
    StringCchPrintfA(s_szDllPath, ARRAYSIZE(s_szDllPath), "%ls", s_wzDllPath);

    SyelogOpen("trcapi" DETOURS_STRINGIFY(DETOURS_BITS), SYELOG_FACILITY_APPLICATION);
    ProcessEnumerate();

    LONG error = AttachDetours();
    if (error != NO_ERROR) {
        Syelog(SYELOG_SEVERITY_FATAL, "### Error attaching detours: %d\n", error);
    }

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
    BOOL ret;

    if (DetourIsHelperProcess()) {
        return TRUE;
    }

    switch (dwReason) {
      case DLL_PROCESS_ATTACH:
        DetourRestoreAfterWith();
        OutputDebugStringA("trcapi" DETOURS_STRINGIFY(DETOURS_BITS) ".dll:"
                           " DllMain DLL_PROCESS_ATTACH\n");
        return ProcessAttach(hModule);
      case DLL_PROCESS_DETACH:
        ret = ProcessDetach(hModule);
        OutputDebugStringA("trcapi" DETOURS_STRINGIFY(DETOURS_BITS) ".dll:"
                           " DllMain DLL_PROCESS_DETACH\n");
        return ret;
      case DLL_THREAD_ATTACH:
        OutputDebugStringA("trcapi" DETOURS_STRINGIFY(DETOURS_BITS) ".dll:"
                           " DllMain DLL_THREAD_ATTACH\n");
        return ThreadAttach(hModule);
      case DLL_THREAD_DETACH:
        OutputDebugStringA("trcapi" DETOURS_STRINGIFY(DETOURS_BITS) ".dll:"
                           " DllMain DLL_THREAD_DETACH\n");
        return ThreadDetach(hModule);
    }
    return TRUE;
}
//
///////////////////////////////////////////////////////////////// End of File.
