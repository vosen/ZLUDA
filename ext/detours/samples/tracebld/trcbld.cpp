/////////////////////////////////////////////////////////////////////////////
//
//  Detours Test Program (trcbld.cpp of trcbld.dll)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//

#define _WIN32_WINNT        0x0500
#define WIN32
#define NT

#define DBG_TRACE   0

#include <windows.h>
#include <stdio.h>
#pragma warning(push)
#if _MSC_VER > 1400
#pragma warning(disable:6102 6103) // /analyze warnings
#endif
#include <strsafe.h>
#pragma warning(pop)
#include "detours.h"
#include "tracebld.h"

#define PULONG_PTR          PVOID
#define PLONG_PTR           PVOID
#define ULONG_PTR           PVOID

//////////////////////////////////////////////////////////////////////////////

#pragma warning(disable:4127)   // Many of our asserts are constants.

#define DEBUG_BREAK() DebugBreak()

#define ASSERT_ALWAYS(x)   \
    do {                                                        \
        if (!(x)) {                                             \
            AssertFailed(#x, __FILE__, __LINE__);               \
            DebugBreak();                                       \
        }                                                       \
    } while (0)

#ifndef NDEBUG
#define ASSERT(x)           ASSERT_ALWAYS(x)
#else
#define ASSERT(x)
#endif

#define UNUSED(c)       (c) = (c)

//////////////////////////////////////////////////////////////////////////////
static HMODULE s_hInst = NULL;
static HMODULE s_hKernel32 = NULL;
static CHAR s_szDllPath[MAX_PATH];
static TBLOG_PAYLOAD s_Payload;
static TBLOG_PAYLOAD s_ChildPayload;
static CRITICAL_SECTION s_csChildPayload;
static DWORD s_nTraceProcessId = 0;
static LONG s_nChildCnt = 0;

static CRITICAL_SECTION s_csPipe;                       // Guards access to hPipe.
static HANDLE           s_hPipe = INVALID_HANDLE_VALUE;
static TBLOG_MESSAGE    s_rMessage;

// Logging Functions.
//
VOID Tblog(PCSTR pszMsgf, ...);
VOID TblogV(PCSTR pszMsgf, va_list args);

VOID VSafePrintf(PCSTR pszMsg, va_list args, PCHAR pszBuffer, LONG cbBuffer);
PCHAR SafePrintf(PCHAR pszBuffer, LONG cbBuffer, PCSTR pszMsg, ...);

LONG EnterFunc();
VOID ExitFunc();
VOID Print(PCSTR psz, ...);
VOID NoteRead(PCSTR psz);
VOID NoteRead(PCWSTR pwz);
VOID NoteWrite(PCSTR psz);
VOID NoteWrite(PCWSTR pwz);
VOID NoteDelete(PCSTR psz);
VOID NoteDelete(PCWSTR pwz);
VOID NoteCleanup(PCSTR psz);
VOID NoteCleanup(PCWSTR pwz);

PBYTE LoadFile(HANDLE hFile, DWORD cbFile);
static PCHAR RemoveReturns(PCHAR pszBuffer);
static PWCHAR RemoveReturns(PWCHAR pwzBuffer);

VOID AssertFailed(CONST PCHAR pszMsg, CONST PCHAR pszFile, ULONG nLine);

int WINAPI Mine_EntryPoint(VOID);
VOID WINAPI Mine_ExitProcess(UINT a0);

//////////////////////////////////////////////////////////////////////////////
//
int (WINAPI * Real_EntryPoint)(VOID)
    = NULL;

BOOL (WINAPI * Real_CreateDirectoryW)(LPCWSTR a0,
                                      LPSECURITY_ATTRIBUTES a1)
    = CreateDirectoryW;

BOOL (WINAPI * Real_CreateDirectoryExW)(LPCWSTR a0,
                                        LPCWSTR a1,
                                        LPSECURITY_ATTRIBUTES a2)
    = CreateDirectoryExW;

HANDLE (WINAPI * Real_CreateFileW)(LPCWSTR a0,
                                   DWORD a1,
                                   DWORD a2,
                                   LPSECURITY_ATTRIBUTES a3,
                                   DWORD a4,
                                   DWORD a5,
                                   HANDLE a6)
    = CreateFileW;

HANDLE (WINAPI * Real_CreateFileMappingW)(HANDLE hFile,
                                         LPSECURITY_ATTRIBUTES lpAttributes,
                                         DWORD flProtect,
                                         DWORD dwMaximumSizeHigh,
                                         DWORD dwMaximumSizeLow,
                                         LPCWSTR lpName
                                        )
    = CreateFileMappingW;

BOOL (WINAPI * Real_CreatePipe)(PHANDLE hReadPipe,
                                PHANDLE hWritePipe,
                                LPSECURITY_ATTRIBUTES lpPipeAttributes,
                                DWORD nSize)
    = CreatePipe;

BOOL (WINAPI * Real_CloseHandle)(HANDLE a0)
    = CloseHandle;

BOOL (WINAPI * Real_DuplicateHandle)(HANDLE hSourceProcessHandle,
                                     HANDLE hSourceHandle,
                                     HANDLE hTargetProcessHandle,
                                     LPHANDLE lpTargetHandle,
                                     DWORD dwDesiredAccess,
                                     BOOL bInheritHandle,
                                     DWORD dwOptions)
    = DuplicateHandle;

BOOL (WINAPI * Real_CreateProcessW)(LPCWSTR lpApplicationName,
                                    LPWSTR lpCommandLine,
                                    LPSECURITY_ATTRIBUTES lpProcessAttributes,
                                    LPSECURITY_ATTRIBUTES lpThreadAttributes,
                                    BOOL bInheritHandles,
                                    DWORD dwCreationFlags,
                                    LPVOID lpEnvironment,
                                    LPCWSTR lpCurrentDirectory,
                                    LPSTARTUPINFOW lpStartupInfo,
                                    LPPROCESS_INFORMATION lpProcessInformation)
    = CreateProcessW;

BOOL (WINAPI * Real_CreateProcessA)(LPCSTR lpApplicationName,
                                    LPSTR lpCommandLine,
                                    LPSECURITY_ATTRIBUTES lpProcessAttributes,
                                    LPSECURITY_ATTRIBUTES lpThreadAttributes,
                                    BOOL bInheritHandles,
                                    DWORD dwCreationFlags,
                                    LPVOID lpEnvironment,
                                    LPCSTR lpCurrentDirectory,
                                    LPSTARTUPINFOA lpStartupInfo,
                                    LPPROCESS_INFORMATION lpProcessInformation)
    = CreateProcessA;

BOOL (WINAPI * Real_DeleteFileW)(LPCWSTR a0)
    = DeleteFileW;
BOOL (WINAPI * Real_DeviceIoControl)(HANDLE a0,
                                     DWORD dwIoControlCode,
                                     LPVOID lpInBuffer,
                                     DWORD nInBufferSize,
                                     LPVOID lpOutBuffer,
                                     DWORD nOutBufferSize,
                                     LPDWORD lpBytesReturned,
                                     LPOVERLAPPED lpOverlapped)
    = DeviceIoControl;

DWORD (WINAPI * Real_GetFileAttributesW)(LPCWSTR a0)
    = GetFileAttributesW;

BOOL (WINAPI * Real_MoveFileWithProgressW)(LPCWSTR lpExistingFileName,
                                           LPCWSTR lpNewFileName,
                                           LPPROGRESS_ROUTINE lpProgressRoutine,
                                           LPVOID lpData,
                                           DWORD dwFlags)
    = MoveFileWithProgressW;

BOOL (WINAPI * Real_MoveFileA)(LPCSTR a0,
                               LPCSTR a1)
    = MoveFileA;

BOOL (WINAPI * Real_MoveFileW)(LPCWSTR a0,
                               LPCWSTR a12)
    = MoveFileW;

BOOL (WINAPI * Real_MoveFileExA)(LPCSTR a0,
                                 LPCSTR a1,
                                 DWORD a2)
    = MoveFileExA;

BOOL (WINAPI * Real_MoveFileExW)(LPCWSTR a0,
                                 LPCWSTR a1,
                                 DWORD a2)
    = MoveFileExW;

BOOL (WINAPI * Real_CopyFileExA)(LPCSTR a0,
                                 LPCSTR a1,
                                 LPPROGRESS_ROUTINE a2,
                                 LPVOID a4,
                                 LPBOOL a5,
                                 DWORD a6)
    = CopyFileExA;

BOOL (WINAPI * Real_CopyFileExW)(LPCWSTR a0,
                                 LPCWSTR a1,
                                 LPPROGRESS_ROUTINE a2,
                                 LPVOID a4,
                                 LPBOOL a5,
                                 DWORD a6)
    = CopyFileExW;

BOOL (WINAPI * Real_PrivCopyFileExW)(LPCWSTR  lpExistingFileName,
                                     LPCWSTR  lpNewFileName,
                                     LPPROGRESS_ROUTINE  lpProgressRoutine,
                                     LPVOID  lpData,
                                     LPBOOL  pbCancel,
                                     DWORD  dwCopyFlags)
    = NULL;

BOOL (WINAPI * Real_CreateHardLinkA)(LPCSTR a0,
                                     LPCSTR a1,
                                     LPSECURITY_ATTRIBUTES a2)
    = CreateHardLinkA;

BOOL (WINAPI * Real_CreateHardLinkW)(LPCWSTR a0,
                                     LPCWSTR a1,
                                     LPSECURITY_ATTRIBUTES a2)
    = CreateHardLinkW;

BOOL (WINAPI * Real_SetStdHandle)(DWORD a0,
                                  HANDLE a1)
    = SetStdHandle;

HMODULE (WINAPI * Real_LoadLibraryA)(LPCSTR a0)
    = LoadLibraryA;

HMODULE (WINAPI * Real_LoadLibraryW)(LPCWSTR a0)
    = LoadLibraryW;

HMODULE (WINAPI * Real_LoadLibraryExA)(LPCSTR a0,
                                       HANDLE a1,
                                       DWORD a2)
    = LoadLibraryExA;

HMODULE (WINAPI * Real_LoadLibraryExW)(LPCWSTR a0,
                                       HANDLE a1,
                                       DWORD a2)
    = LoadLibraryExW;

DWORD (WINAPI * Real_SetFilePointer)(HANDLE hFile,
                                     LONG lDistanceToMove,
                                     PLONG lpDistanceToMoveHigh,
                                     DWORD dwMoveMethod)
    = SetFilePointer;

BOOL (WINAPI * Real_SetFilePointerEx)(HANDLE hFile,
                                      LARGE_INTEGER liDistanceToMove,
                                      PLARGE_INTEGER lpNewFilePointer,
                                      DWORD dwMoveMethod)
    = SetFilePointerEx;

BOOL (WINAPI * Real_ReadFile)(HANDLE a0,
                                 LPVOID a1,
                                 DWORD a2,
                                 LPDWORD a3,
                                 LPOVERLAPPED a4)
    = ReadFile;

BOOL (WINAPI * Real_ReadFileEx)(HANDLE a0,
                                   LPVOID a1,
                                   DWORD a2,
                                   LPOVERLAPPED a3,
                                   LPOVERLAPPED_COMPLETION_ROUTINE a4)
    = ReadFileEx;

BOOL (WINAPI * Real_WriteFile)(HANDLE a0,
                                  LPCVOID a1,
                                  DWORD a2,
                                  LPDWORD a3,
                                  LPOVERLAPPED a4)
    = WriteFile;

BOOL (WINAPI * Real_WriteFileEx)(HANDLE a0,
                                    LPCVOID a1,
                                    DWORD a2,
                                    LPOVERLAPPED a3,
                                    LPOVERLAPPED_COMPLETION_ROUTINE a4)
    = WriteFileEx;

BOOL (WINAPI * Real_WriteConsoleA)(HANDLE a0,
                                      const VOID* a1,
                                      DWORD a2,
                                      LPDWORD a3,
                                      LPVOID a4)
    = WriteConsoleA;

BOOL (WINAPI * Real_WriteConsoleW)(HANDLE a0,
                                      const VOID* a1,
                                      DWORD a2,
                                      LPDWORD a3,
                                      LPVOID a4)
    = WriteConsoleW;

VOID (WINAPI * Real_ExitProcess)(UINT a0)
    = ExitProcess;

DWORD (WINAPI * Real_ExpandEnvironmentStringsA)(PCSTR lpSrc, PCHAR lpDst, DWORD nSize)
    = ExpandEnvironmentStringsA;

DWORD (WINAPI * Real_ExpandEnvironmentStringsW)(PCWSTR lpSrc, PWCHAR lpDst, DWORD nSize)
    = ExpandEnvironmentStringsW;

DWORD (WINAPI * Real_GetEnvironmentVariableA)(PCSTR lpName, PCHAR lpBuffer, DWORD nSize)
    = GetEnvironmentVariableA;

DWORD (WINAPI * Real_GetEnvironmentVariableW)(PCWSTR lpName, PWCHAR lpBuffer, DWORD nSize)
    = GetEnvironmentVariableW;

PCWSTR (CDECL * Real_wgetenv)(PCWSTR var) = NULL;
PCSTR (CDECL * Real_getenv)(PCSTR var) = NULL;
DWORD (CDECL * Real_getenv_s)(DWORD *pValue, PCHAR pBuffer, DWORD cBuffer, PCSTR varname) = NULL;
DWORD (CDECL * Real_wgetenv_s)(DWORD *pValue, PWCHAR pBuffer, DWORD cBuffer, PCWSTR varname) = NULL;
DWORD (CDECL * Real_dupenv_s)(PCHAR *ppBuffer, DWORD *pcBuffer, PCSTR varname) = NULL;
DWORD (CDECL * Real_wdupenv_s)(PWCHAR *ppBuffer, DWORD *pcBuffer, PCWSTR varname) = NULL;

//////////////////////////////////////////////////////////////////////////////
//
static VOID Copy(PWCHAR pwzDst, PCWSTR pwzSrc)
{
    while (*pwzSrc) {
        *pwzDst++ = *pwzSrc++;
    }
    *pwzDst = '\0';
}

static DWORD Size(PCWSTR pwzSrc)
{
    DWORD c = 0;
    while (pwzSrc[c]) {
        c++;
    }
    return c;
}

static PCWSTR Save(PCWSTR pwzSrc)
{
    DWORD c = (Size(pwzSrc) + 1) * sizeof(WCHAR);
    PWCHAR pwzDst = (PWCHAR)GlobalAlloc(GPTR, c);
    CopyMemory(pwzDst, pwzSrc, c);

    return pwzDst;
}

static BOOL HasSpace(PCWSTR pwz)
{
    for (; *pwz; pwz++) {
        if (*pwz == ' ' || *pwz == '\t' || *pwz == '\r' || *pwz == '\n') {
            return TRUE;
        }
    }
    return FALSE;
}

static BOOL HasChar(PCWSTR pwz, WCHAR w)
{
    for (; *pwz; pwz++) {
        if (*pwz == w) {
            return TRUE;
        }
    }
    return FALSE;
}

static DWORD Compare(PCWSTR pwzA, PCWSTR pwzB)
{
    for (;;) {
        WCHAR cA = *pwzA++;
        WCHAR cB = *pwzB++;

        if (cA >= 'A' && cA <= 'Z') {
            cA += ('a' - 'A');
        }
        if (cB >= 'A' && cB <= 'Z') {
            cB += ('a' - 'A');
        }

        if (cA == 0 && cB == 0) {
            return 0;
        }
        if (cA != cB) {
            return cA - cB;
        }
    }
}

static DWORD Compare(PCWSTR pwzA, PCSTR pszB)
{
    for (;;) {
        WCHAR cA = *pwzA++;
        WCHAR cB = *pszB++;

        if (cA >= 'A' && cA <= 'Z') {
            cA += ('a' - 'A');
        }
        if (cB >= 'A' && cB <= 'Z') {
            cB += ('a' - 'A');
        }

        if (cA == 0 && cB == 0) {
            return 0;
        }
        if (cA != cB) {
            return cA - cB;
        }
    }
}

static DWORD Compare(PCSTR pszA, PCSTR pszB)
{
    for (;;) {
        CHAR cA = *pszA++;
        CHAR cB = *pszB++;

        if (cA >= 'A' && cA <= 'Z') {
            cA += ('a' - 'A');
        }
        if (cB >= 'A' && cB <= 'Z') {
            cB += ('a' - 'A');
        }

        if (cA == 0 && cB == 0) {
            return 0;
        }
        if (cA != cB) {
            return cA - cB;
        }
    }
}

//////////////////////////////////////////////////////////////////////////////

static PCSTR s_rpszMsvcrNames[] = {
    "msvcr80.dll",
    "msvcr80d.dll",
    "msvcr71.dll",
    "msvcr71d.dll",
    "msvcr70.dll",
    "msvcr70d.dll",
    NULL
};

HMODULE s_hMsvcr = NULL;
PCSTR s_pszMsvcr = NULL;

static BOOL WINAPI ImportFileCallback(PVOID pContext, HMODULE hFile, PCSTR pszFile)
{
    UNUSED(pContext);

    if (pszFile != NULL) {
        for (int i = 0; s_rpszMsvcrNames[i]; i++) {
            if (Compare(pszFile, s_rpszMsvcrNames[i]) == 0) {
                s_hMsvcr = hFile;
                s_pszMsvcr = s_rpszMsvcrNames[i];
                return FALSE;
            }
        }
    }
    return TRUE;
}

BOOL FindMsvcr()
{
    DetourEnumerateImports(NULL, NULL, ImportFileCallback, NULL);

    if (s_hMsvcr != NULL) {
        return TRUE;
    }

    return FALSE;
}

BOOL FindProc(PVOID * ppvCode, PCSTR pwzFunc)
{
    PVOID pv = GetProcAddress(s_hMsvcr, pwzFunc);
    if (pv != NULL) {
        *ppvCode = pv;
        return TRUE;
    }
    else {
        *ppvCode = NULL;
        return FALSE;
    }
}

//////////////////////////////////////////////////////////////////////////////
//
struct EnvInfo
{
    DWORD   m_nHash;
    DWORD   m_nIndex;
    PCWSTR  m_pwzVar;
    PCWSTR  m_pwzVal;
    BOOL    m_fDefined;
    BOOL    m_fUsed;
    BOOL    m_fOriginal;
};

//////////////////////////////////////////////////////////////////////////////
//
class EnvVars
{
  private:
    static CRITICAL_SECTION s_csLock;
    static DWORD            s_nVars;
    static DWORD            s_nCapacity;
    static EnvInfo **       s_pVars;

  private:
    static DWORD Hash(PCWSTR pwzVar)
    {
        DWORD hash = 5381;
        while (*pwzVar != 0) {
            WCHAR c = *pwzVar++;
            if (c >= 'A' && c <= 'Z') {
                c += ('a' - 'A');
            }
            hash = ((hash << 5) + hash) + c;
        }
        return hash;
    }

    static VOID LockAcquire()
    {
        EnterCriticalSection(&s_csLock);
    }

    static VOID LockRelease()
    {
        LeaveCriticalSection(&s_csLock);
    }

    static VOID Resize(DWORD nCapacity);
    static VOID Set(EnvInfo *info);
    static EnvInfo * Find(PCWSTR pwzVar);

  public:
    static BOOL Equal(PCWSTR pwzA, PCWSTR pwzB)
    {
        return (Compare(pwzA, pwzB) == 0);
    }

  public:
    static VOID Initialize();
    static VOID Dump();

    static VOID Add(PCWSTR pwzVar, PCWSTR pwzVal);

    static VOID Used(PCWSTR pwzVar);
    static VOID Used(PCSTR pszVar);
};

CRITICAL_SECTION    EnvVars::s_csLock;
DWORD               EnvVars::s_nVars = 0;
DWORD               EnvVars::s_nCapacity = 0;
EnvInfo **          EnvVars::s_pVars = NULL;

VOID EnvVars::Initialize()
{
    InitializeCriticalSection(&s_csLock);

    Resize(919);
}

VOID EnvVars::Resize(DWORD nCapacity)
{
    if (nCapacity > s_nCapacity) {
        DWORD nOld = s_nCapacity;
        EnvInfo ** pOld = s_pVars;

        // DEBUG_BREAK();

        s_pVars = (EnvInfo **)GlobalAlloc(GPTR, nCapacity * sizeof(EnvInfo *));
        s_nCapacity = nCapacity;

        if (pOld != NULL) {
            for (DWORD n = 0; n < nOld; n++) {
                if (pOld[n] != NULL) {
                    Set(pOld[n]);
                }
            }
            GlobalFree((HGLOBAL)pOld);
            pOld = NULL;
        }
    }
}

VOID EnvVars::Set(EnvInfo *info)
{
    DWORD hash = info->m_nHash;
    DWORD slot = hash % s_nCapacity;
    DWORD death = 0;

    // Find an empty slot.
    for (; s_pVars[slot] != NULL; slot = (slot + 1) % s_nCapacity) {
        if (++death > s_nCapacity) {
            // We should have dropped out at some point...
            DEBUG_BREAK();
        }
    }

    s_pVars[slot] = info;
}

EnvInfo * EnvVars::Find(PCWSTR pwzVar)
{
    DWORD hash = Hash(pwzVar);
    DWORD slot = hash % s_nCapacity;

    LockAcquire();

    // Find the the matching slot, or an empty one.
    for (; s_pVars[slot] != NULL; slot = (slot + 1) % s_nCapacity) {
        if (Equal(s_pVars[slot]->m_pwzVar, pwzVar)) {
            LockRelease();
            return s_pVars[slot];
        }
    }
    LockRelease();
    return NULL;
}

VOID EnvVars::Add(PCWSTR pwzVar, PCWSTR pwzVal)
{
    if (pwzVar == NULL) {
        return;
    }

    WCHAR wzVar[MAX_PATH];
    PWCHAR pwzDst = wzVar;
    while (*pwzVar) {
        if (*pwzVar >= 'a' && *pwzVar <= 'z') {
            *pwzDst++ = *pwzVar - ('a' - 'A');
        }
        else {
            *pwzDst++ = *pwzVar;
        }
        pwzVar++;
    }
    *pwzDst = '\0';
    pwzVar = wzVar;

    WCHAR wzVal[] = L"";
    if (pwzVal != NULL) {
        while (*pwzVal == ' ' || *pwzVal == '\t') {
            *pwzVal++;
        }
    }
    else {
        pwzVal = wzVal;
    }

    // Tblog("<!-- ::Add var=[%le] val=[%le] -->\n", pwzVar, pwzVal);
    LockAcquire();

    // DEBUG_BREAK();

    DWORD hash = Hash(pwzVar);
    DWORD slot = hash % s_nCapacity;
    EnvInfo *info = NULL;
    DWORD death = 0;

    // Find the the matching slot, or an empty one.
    for (; s_pVars[slot] != NULL; slot = (slot + 1) % s_nCapacity) {
        if (Equal(s_pVars[slot]->m_pwzVar, pwzVar)) {
            LockRelease();
            return;
        }
        if (++death > s_nCapacity) {
            // We should have dropped out at some point...
            DEBUG_BREAK();
        }
    }

    // Add the var to list of known vars.
    info = (EnvInfo *)GlobalAlloc(GPTR, sizeof(EnvInfo));
    info->m_nHash = hash;
    info->m_nIndex = s_nVars++;
    info->m_pwzVar = Save(pwzVar);
    info->m_pwzVal = Save(pwzVal);
    if (pwzVal[0] == '\0') {
        info->m_fDefined = FALSE;
        info->m_fUsed = TRUE;
    }
    else {
        info->m_fDefined = TRUE;
    }
    s_pVars[slot] = info;

    // Check if we should grow the table.
    if (s_nVars > (s_nCapacity / 2)) {
        Resize(s_nCapacity * 2 - 1);
    }

    LockRelease();
}

VOID EnvVars::Used(PCWSTR pwzVar)
{
    if (pwzVar != NULL) {
        // Tblog("<!-- Used [%le] -->\n", pwzVar);
        EnvInfo *pInfo = Find(pwzVar);
        if (pInfo) {
            pInfo->m_fUsed = TRUE;
        }
#if 0
        else {
            Add(pwzVar, NULL);
        }
#endif
    }
}

VOID EnvVars::Used(PCSTR pszVar)
{
    if (pszVar != NULL) {
        WCHAR wzVar[MAX_PATH];
        PWCHAR pwzVar = wzVar;
        while (*pszVar) {
            *pwzVar++ = *pszVar++;
        }
        *pwzVar = '\0';

        Used(wzVar);
    }
}

VOID EnvVars::Dump()
{
    if (s_nVars == 0) {
        return;
    }

    LockAcquire();

    Tblog("<t:Vars>\n");

    // Remove any variables that match the original environment.
    PCWSTR pwzz = s_Payload.wzzEnvironment;
    while (*pwzz) {
        WCHAR wzVar[MAX_PATH];
        PWCHAR pwzVar = wzVar;

        while (*pwzz && *pwzz != '=') {
            *pwzVar++ = *pwzz++;
        }
        *pwzVar = '\0';
        if (*pwzz == '=') {
            pwzz++;
        }

        EnvInfo *pInfo = Find(wzVar);
        if (pInfo) {
            if (Compare(pwzz, pInfo->m_pwzVal) == 0) {
                pInfo->m_fUsed = FALSE;
            }
        }
        pwzz += Size(pwzz) + 1;
    }


    EnvInfo ** pSorted = (EnvInfo **)GlobalAlloc(GPTR, s_nVars * sizeof(EnvInfo *));

    for (DWORD n = 0; n < s_nCapacity; n++) {
        if (s_pVars[n] != NULL) {
            if (s_pVars[n]->m_nIndex > s_nVars) {
                DEBUG_BREAK();
            }
            pSorted[s_pVars[n]->m_nIndex] = s_pVars[n];
        }
    }

    for (DWORD n = 0; n < s_nVars; n++) {
        EnvInfo *pInfo = pSorted[n];

        if (pInfo == NULL) {
            Print("<!-- Warning: Missing %d of %d -->\n", n, s_nVars);
            continue;
        }

        if (pInfo->m_fUsed && pInfo->m_pwzVal[0]) {
            Print("<t:Var var=\"%le\">%le</t:Var>\n", pInfo->m_pwzVar, pInfo->m_pwzVal);
        }
    }
    GlobalFree((HGLOBAL)pSorted);

    Tblog("</t:Vars>\n");

    LockRelease();
}

void SaveEnvironment()
{
    LPWCH pwStrings = GetEnvironmentStringsW();
    PCWSTR pwEnv = (PCWSTR)pwStrings;

    while (*pwEnv != '\0') {
        WCHAR wzVar[MAX_PATH];
        PWCHAR pwzDst = wzVar;
        PCWSTR pwzVal = NULL;

        if (*pwEnv == '=') {
            *pwzDst++ = *pwEnv++;
        }
        while (*pwEnv != '\0' && *pwEnv != '=') {
            *pwzDst++ = *pwEnv++;
        }
        *pwzDst++ = '\0';

        if (*pwEnv == '=') {
            pwEnv++;
        }

        pwzVal = pwEnv;
        while (*pwEnv != '\0') {
            pwEnv++;
        }
        if (*pwEnv == '\0') {
            pwEnv++;
        }
        if (wzVar[0] != '=') {
            EnvVars::Add(wzVar, pwzVal);
        }
    }
    FreeEnvironmentStringsW(pwStrings);
}

//////////////////////////////////////////////////////////////////////////////
//
struct ProcInfo
{
    HANDLE  m_hProc;
    DWORD   m_nProcId;
    DWORD   m_nProc;
};

class Procs
{
  private:
    static CRITICAL_SECTION s_csLock;
    static DWORD            s_nProcs;
    static ProcInfo         s_rProcs[4049];

  private:
    static ProcInfo& HashToSlot(HANDLE handle)
    {
        return s_rProcs[((DWORD_PTR)handle) % ARRAYSIZE(s_rProcs)];
    }

    static VOID LockAcquire()
    {
        EnterCriticalSection(&s_csLock);
    }

    static VOID LockRelease()
    {
        LeaveCriticalSection(&s_csLock);
    }

  public:
    static VOID Initialize();
    static ProcInfo * Create(HANDLE hProc, DWORD nProcId);
    static BOOL Close(HANDLE hProc);
};

CRITICAL_SECTION    Procs::s_csLock;
DWORD               Procs::s_nProcs = 0;
ProcInfo            Procs::s_rProcs[4049];

VOID Procs::Initialize()
{
    InitializeCriticalSection(&s_csLock);
    for (DWORD i = 0; i < ARRAYSIZE(s_rProcs); i++) {
        s_rProcs[i].m_hProc = INVALID_HANDLE_VALUE;
    }
}

ProcInfo * Procs::Create(HANDLE hProc, DWORD nProcId)
{
    LockAcquire();
    s_nProcs++;
    ProcInfo& slot = HashToSlot(hProc);
    slot.m_hProc = hProc;
    slot.m_nProcId = nProcId;
    slot.m_nProc = s_nProcs;
    Print("<!-- CreateProcess (%d)-->\n", slot.m_nProc);
    LockRelease();

    return &slot;
}

BOOL Procs::Close(HANDLE hProc)
{
    BOOL first = false;

    LockAcquire();
    ProcInfo& slot = HashToSlot(hProc);
    if (slot.m_hProc == hProc) {
        first = true;
        Print("<!-- CloseProcess (%d)-->\n", slot.m_nProc);
        slot.m_hProc = INVALID_HANDLE_VALUE;
        slot.m_nProcId = 0;
        slot.m_nProc = 0;
        s_nProcs--;
    }
    LockRelease();

    return first;
}

//////////////////////////////////////////////////////////////////////////////
//
struct FileInfo
{
    DWORD   m_nHash;
    DWORD   m_nIndex;

    BOOL    m_fCantRead;        // Set for file that are opened Create
    BOOL    m_fRead;
    BOOL    m_fWrite;

    BOOL    m_fDelete;
    BOOL    m_fCleanup;
    BOOL    m_fSystemPath;
    BOOL    m_fTemporaryPath;
    BOOL    m_fTemporaryFile;

    DWORD   m_cbRead;
    DWORD   m_cbWrite;

    BOOL    m_fAppend;
    BOOL    m_fAbsorbed;        // Absorbed by TraceBld.
    BOOL    m_fDirectory;

    PCWSTR  m_pwzPath;
    PBYTE   m_pbContent;
    DWORD   m_cbContent;

};

//////////////////////////////////////////////////////////////////////////////
//
class FileNames
{
  private:
    static CRITICAL_SECTION s_csLock;
    static DWORD            s_nFiles;
    static DWORD            s_nCapacity;
    static FileInfo **      s_pFiles;

  public:
    static WCHAR            s_wzSysPath[MAX_PATH];
    static WCHAR            s_wzS64Path[MAX_PATH];
    static WCHAR            s_wzTmpPath[MAX_PATH];
    static WCHAR            s_wzExePath[MAX_PATH];
    static DWORD            s_wcSysPath;
    static DWORD            s_wcS64Path;
    static DWORD            s_wcTmpPath;
    static DWORD            s_wcExePath;

  private:
    static DWORD Hash(PCWSTR pwzFile)
    {
        DWORD hash = 5381;
        while (*pwzFile != 0) {
            WCHAR c = *pwzFile++;
            if (c >= 'A' && c <= 'Z') {
                c += ('a' - 'A');
            }
            hash = ((hash << 5) + hash) + c;
        }
        return hash;
    }

    static VOID LockAcquire()
    {
        EnterCriticalSection(&s_csLock);
    }

    static VOID LockRelease()
    {
        LeaveCriticalSection(&s_csLock);
    }

    static VOID Resize(DWORD nCapacity);
    static VOID Set(FileInfo *info);
    static VOID Replace(PWCHAR pwzBuffer, PWCHAR pwzDstEnd, DWORD cwOld, PCWSTR pwzNew);

  public:
    static BOOL Equal(PCWSTR pwzA, PCWSTR pwzB)
    {
        return (Compare(pwzA, pwzB) == 0);
    }

    static BOOL PrefixMatch(PCWSTR pwzFile, PCWSTR pwzPrefix)
    {
        for (;;) {
            WCHAR cFile = *pwzFile++;
            WCHAR cPrefix = *pwzPrefix++;

            if (cFile >= 'A' && cFile <= 'Z') {
                cFile += ('a' - 'A');
            }
            if (cPrefix >= 'A' && cPrefix <= 'Z') {
                cPrefix += ('a' - 'A');
            }

            if (cPrefix == 0) {
                return TRUE;
            }
            if (cFile != cPrefix) {
                return FALSE;
            }
        }
    }

    static BOOL SuffixMatch(PCWSTR pwzFile, PCWSTR pwzSuffix)
    {
        // Move both pointers to the end of the strings.
        PCWSTR pwzFileBeg = pwzFile;
        while (*pwzFile) {
            pwzFile++;
        }

        PCWSTR pwzSuffixBeg = pwzSuffix;
        while (*pwzSuffix) {
            pwzSuffix++;
        }

        // Now walk backwards comparing strings.
        for (;;) {
            WCHAR cFile = (pwzFile > pwzFileBeg) ? *--pwzFile : 0;
            WCHAR cSuffix = (pwzSuffix > pwzSuffixBeg) ? *--pwzSuffix : 0;

            if (cFile >= 'A' && cFile <= 'Z') {
                cFile += ('a' - 'A');
            }
            if (cSuffix >= 'A' && cSuffix <= 'Z') {
                cSuffix += ('a' - 'A');
            }

            if (cSuffix == 0) {
                return TRUE;
            }
            if (cFile != cSuffix) {
                return FALSE;
            }
        }
    }

    static VOID EndInSlash(PWCHAR pwzPath)
    {
        if (*pwzPath) {
            while (*pwzPath) {
                pwzPath++;
            }
            if (pwzPath[-1] != '\\') {
                *pwzPath++ = '\\';
                *pwzPath = '\0';
            }
        }
    }

  public:
    static VOID Initialize();
    static VOID Dump();
    static FileInfo * FindPartial(PCWSTR pwzPath);
    static FileInfo * FindPartial(PCSTR pszPath);
    static FileInfo * FindFull(PCWSTR pwzPath);
    static PCWSTR ParameterizeName(PWCHAR pwzDst, DWORD cMaxDst, PCWSTR pwzPath);
    static PCWSTR ParameterizeName(PWCHAR pwzDst, DWORD cMaxDst, FileInfo *pInfo);
    static VOID ParameterizeLine(PWCHAR pwzDst, PWCHAR pwzDstEnd);
};

CRITICAL_SECTION    FileNames::s_csLock;
DWORD               FileNames::s_nFiles = 0;
DWORD               FileNames::s_nCapacity = 0;
FileInfo **         FileNames::s_pFiles;
WCHAR               FileNames::s_wzSysPath[MAX_PATH];
WCHAR               FileNames::s_wzS64Path[MAX_PATH];
WCHAR               FileNames::s_wzTmpPath[MAX_PATH];
WCHAR               FileNames::s_wzExePath[MAX_PATH];
DWORD               FileNames::s_wcSysPath;
DWORD               FileNames::s_wcS64Path;
DWORD               FileNames::s_wcTmpPath;
DWORD               FileNames::s_wcExePath;

VOID FileNames::Initialize()
{
    InitializeCriticalSection(&s_csLock);

    s_wzSysPath[0] = '\0';
    GetSystemDirectoryW(s_wzSysPath, ARRAYSIZE(s_wzSysPath));
    EndInSlash(s_wzSysPath);

    s_wzS64Path[0] = '\0';
    GetWindowsDirectoryW(s_wzS64Path, ARRAYSIZE(s_wzS64Path));
    EndInSlash(s_wzS64Path);
    Copy(s_wzS64Path + Size(s_wzS64Path), L"SysWOW64\\");

    s_wzTmpPath[0] = '\0';
    GetTempPathW(ARRAYSIZE(s_wzTmpPath), s_wzTmpPath);
    EndInSlash(s_wzTmpPath);

    s_wzExePath[0] = '\0';
    GetModuleFileNameW(NULL, s_wzExePath, ARRAYSIZE(s_wzExePath));
    PWCHAR pwzLast = s_wzExePath;
    for (PWCHAR pwz = s_wzExePath; *pwz; pwz++) {
        if (*pwz == '\\') {
            pwzLast = pwz;
        }
    }
    if (*pwzLast == '\\') {
        *++pwzLast = '\0';
    }

    s_wcSysPath = Size(s_wzSysPath);
    s_wcS64Path = Size(s_wzS64Path);
    s_wcTmpPath = Size(s_wzTmpPath);
    s_wcExePath = Size(s_wzExePath);

    Resize(4049);
}

VOID FileNames::Resize(DWORD nCapacity)
{
    if (nCapacity > s_nCapacity) {
        DWORD nOld = s_nCapacity;
        FileInfo ** pOld = s_pFiles;

        s_pFiles = (FileInfo **)GlobalAlloc(GPTR, nCapacity * sizeof(FileInfo *));
        s_nCapacity = nCapacity;

        if (pOld != NULL) {
            for (DWORD n = 0; n < nOld; n++) {
                if (pOld[n] != NULL) {
                    Set(pOld[n]);
                }
            }
            GlobalFree((HGLOBAL)pOld);
            pOld = NULL;
        }
        s_nCapacity = nCapacity;
    }
}

VOID FileNames::Set(FileInfo *info)
{
    DWORD hash = info->m_nHash;
    DWORD slot = hash % s_nCapacity;
    DWORD death = 0;

    // Find an empty slot.
    for (; s_pFiles[slot] != NULL; slot = (slot + 1) % s_nCapacity) {
        if (++death > s_nCapacity) {
            // We should have dropped out at some point...
            DEBUG_BREAK();
        }
    }

    s_pFiles[slot] = info;
}

FileInfo * FileNames::FindFull(PCWSTR pwzPath)
{
    if (pwzPath == NULL) {
        return NULL;
    }

    LockAcquire();

    DWORD hash = Hash(pwzPath);
    DWORD slot = hash % s_nCapacity;
    FileInfo *info = NULL;
    DWORD death = 0;

    // Find the the matching slot, or an empty one.
    for (; s_pFiles[slot] != NULL; slot = (slot + 1) % s_nCapacity) {
        if (Equal(s_pFiles[slot]->m_pwzPath, pwzPath)) {
            info = s_pFiles[slot];
            goto succeed;
        }
        if (++death > s_nCapacity) {
            // We should have dropped out at some point...
            DEBUG_BREAK();
        }
    }

    // Add the file to list of known files.
    info = (FileInfo *)GlobalAlloc(GPTR, sizeof(FileInfo));
    info->m_nHash = hash;
    info->m_nIndex = s_nFiles++;
    info->m_pwzPath = Save(pwzPath);
    info->m_fSystemPath = (PrefixMatch(info->m_pwzPath, s_wzSysPath) ||
                           PrefixMatch(info->m_pwzPath, s_wzS64Path));
    info->m_fTemporaryPath = PrefixMatch(info->m_pwzPath, s_wzTmpPath);
    info->m_fTemporaryFile = SuffixMatch(info->m_pwzPath, L".tmp");

    s_pFiles[slot] = info;

    // Check if we should grow the table.
    if (s_nFiles > (s_nCapacity / 2)) {
        Resize(s_nCapacity * 2 - 1);
    }

  succeed:
    LockRelease();

    return info;
}

FileInfo * FileNames::FindPartial(PCWSTR pwzPath)
{
    WCHAR wzPath[MAX_PATH];
    PWCHAR pwzFile = NULL;

    if (!GetFullPathNameW(pwzPath, ARRAYSIZE(wzPath), wzPath, &pwzFile)) {
        return FindFull(pwzPath);
    }
    else {
        return FindFull(wzPath);
    }
}

FileInfo * FileNames::FindPartial(PCSTR pwzPath)
{
    WCHAR wzPath[MAX_PATH];
    PWCHAR pwzFile = wzPath;

    while (*pwzPath) {
        *pwzFile++ = *pwzPath++;
    }
    *pwzFile = '\0';

    return FindPartial(wzPath);
}

PCWSTR FileNames::ParameterizeName(PWCHAR pwzDst, DWORD cMaxDst, FileInfo *pInfo)
{
    return ParameterizeName(pwzDst, cMaxDst, pInfo->m_pwzPath);
}

PCWSTR FileNames::ParameterizeName(PWCHAR pwzDst, DWORD cMaxDst, PCWSTR pwzPath)
{
    if (PrefixMatch(pwzPath, s_wzSysPath)) {
        Copy(pwzDst, L"%SYSDIR%\\");
        Copy(pwzDst + Size(pwzDst), pwzPath + s_wcSysPath);
        goto finish;
    }
    else if (PrefixMatch(pwzPath, s_wzS64Path)) {
        Copy(pwzDst, L"%SYSDIR%\\");
        Copy(pwzDst + Size(pwzDst), pwzPath + s_wcS64Path);
        goto finish;
    }
    else if (PrefixMatch(pwzPath, s_wzTmpPath)) {
        Copy(pwzDst, L"%TMPDIR%\\");
        Copy(pwzDst + Size(pwzDst), pwzPath + s_wcTmpPath);
        goto finish;
    }
    else {
        Copy(pwzDst, pwzPath);

      finish:
#if 0 // to convert to all lower case.
        for (PWCHAR pwz = pwzDst; *pwz && pwz < pwzDst + cMaxDst; pwz++) {
            if (*pwz >= 'A' && *pwz <= 'Z') {
                *pwz = 'a' + (*pwz - 'A');
            }
        }
#else
        (void)cMaxDst;
#endif
        return pwzDst;
    }
}

VOID FileNames::Replace(PWCHAR pwzDst, PWCHAR pwzDstEnd, DWORD cwOld, PCWSTR pwzNew)
{
    DWORD cwNew = Size(pwzNew);
    DWORD cwDst = Size(pwzDst);

    if (cwOld < cwNew) {        // We have to insert.
        if ((cwDst + cwNew - cwOld) >= (DWORD)(pwzDstEnd - pwzDst)) {
            // Won't fit, so abort.
            return;
        }

        PWCHAR pwzTo = pwzDst + cwDst + (cwNew - cwOld);
        PWCHAR pwzFm = pwzDst + cwDst;

        while (pwzTo >= pwzDst) {
            *pwzTo-- = *pwzFm--;
        }
    }
    else if (cwOld > cwNew) {  // We have to remove.
        PWCHAR pwzTo = pwzDst + cwNew;
        PWCHAR pwzFm = pwzDst + cwOld;

        while (*pwzFm) {
            *pwzTo++ = *pwzFm++;
        }
        *pwzTo = '\0';
    }

    // Now write the new string.
    while (*pwzNew) {
        *pwzDst++ = *pwzNew++;
    }
}

VOID FileNames::ParameterizeLine(PWCHAR pwzDst, PWCHAR pwzDstEnd)
{
    for (; *pwzDst != '\0'; pwzDst++) {
        if (PrefixMatch(pwzDst, s_wzSysPath)) {
            Replace(pwzDst, pwzDstEnd, s_wcSysPath, L"%SYSDIR%\\");
        }
        else if (PrefixMatch(pwzDst, s_wzS64Path)) {
            Replace(pwzDst, pwzDstEnd, s_wcS64Path, L"%SYSDIR%\\");
        }
        else if (PrefixMatch(pwzDst, s_wzTmpPath)) {
            Replace(pwzDst, pwzDstEnd, s_wcTmpPath, L"%TMPDIR%\\");
        }
    }
}

VOID FileNames::Dump()
{
    WCHAR wzPath[MAX_PATH];

    if (s_nFiles == 0) {
        return;
    }

    LockAcquire();

    Tblog("<t:Files>\n");

    FileInfo ** pSorted = (FileInfo **)GlobalAlloc(GPTR, s_nFiles * sizeof(FileInfo *));

    for (DWORD n = 0; n < s_nCapacity; n++) {
        if (s_pFiles[n] != NULL) {
            if (s_pFiles[n]->m_nIndex > s_nFiles) {
                DEBUG_BREAK();
            }
            pSorted[s_pFiles[n]->m_nIndex] = s_pFiles[n];
        }
    }

    for (DWORD n = 0; n < s_nFiles; n++) {
        FileInfo *pInfo = pSorted[n];

        if (pInfo == NULL) {
            Print("<!-- Warning: Missing %d of %d -->\n", n, s_nFiles);
            continue;
        }

        BOOL fRead = pInfo->m_fRead;
        BOOL fWrite = pInfo->m_fWrite;
        BOOL fDelete = (pInfo->m_fDelete);
        BOOL fCleanup = (pInfo->m_fCleanup);
        BOOL fAppend = (pInfo->m_fAppend);

#if 0
        if (fDelete && !fRead && !fWrite) {
            Print("<!-- Discarding: %ls -->\n", pInfo->m_pwzPath);
            // Discard pipe files only passed to children.
            continue;
        }
#endif
        if (pInfo->m_fAbsorbed) {
            // Discard response fles
            continue;
        }

        if (PrefixMatch(pInfo->m_pwzPath, s_wzExePath) ||
            PrefixMatch(pInfo->m_pwzPath, s_wzSysPath) ||
            PrefixMatch(pInfo->m_pwzPath, s_wzS64Path)) {
            // Discard files from exec directory (because considered internal to code).
            continue;
        }

#if 1 // Ignore PIPEs.
        if (FileNames::PrefixMatch(pInfo->m_pwzPath, L"\\\\.\\PIPE\\")) {
            continue;
        }
#endif
        if (FileNames::SuffixMatch(pInfo->m_pwzPath, L"\\conout$")) {
            continue;
        }
        if (FileNames::SuffixMatch(pInfo->m_pwzPath, L"\\conin$")) {
            continue;
        }
        if (FileNames::SuffixMatch(pInfo->m_pwzPath, L"\\nul")) {
            continue;
        }

        ParameterizeName(wzPath, ARRAYSIZE(wzPath), pInfo);

        if (pInfo->m_fDirectory) {
            Print("<t:File mkdir=\"true\">%ls</t:File>\n", wzPath);
            continue;
        }

        if (!fRead && !fWrite && !fDelete && !fCleanup) {
            // Discard do "none" files.
            continue;
        }

        if (pInfo->m_pbContent == NULL ||
            pInfo->m_fDelete ||
            pInfo->m_fCleanup ||
            pInfo->m_fWrite) {

            Print("<t:File%s%s%s%s%s>%ls</t:File>\n",
                  fRead ? " read=\"true\"" : "",
                  fWrite ? " write=\"true\"" : "",
                  fDelete ? " delete=\"true\"" : "",
                  fCleanup ? " cleanup=\"true\"" : "",
                  fAppend ? " append=\"true\"" : "",
                  // size=\"%d\" pInfo->m_cbContent,
                  wzPath);
        }
        else if ((pInfo->m_pbContent)[0] == 0xff && (pInfo->m_pbContent)[1] == 0xfe) {
            // Unicode
            Print("<t:File%s%s%s%s%s>%ls<t:Data>%le</t:Data></t:File>\n",
                  fRead ? " read=\"true\"" : "",
                  fWrite ? " write=\"true\"" : "",
                  fDelete ? " delete=\"true\"" : "",
                  fCleanup ? " cleanup=\"true\"" : "",
                  fAppend ? " append=\"true\"" : "",
                  //  size=\"%d\" pInfo->m_cbContent,
                  wzPath,
                  RemoveReturns((PWCHAR)pInfo->m_pbContent));
        }
        else {
            // Ascii
            Print("<t:File%s%s%s%s%s>%ls<t:Data>%he</t:Data></t:File>\n",
                  fRead ? " read=\"true\"" : "",
                  fWrite ? " write=\"true\"" : "",
                  fDelete ? " delete=\"true\"" : "",
                  fCleanup ? " cleanup=\"true\"" : "",
                  fAppend ? " append=\"true\"" : "",
                  //  size=\"%d\" pInfo->m_cbContent,
                  wzPath,
                  RemoveReturns((PCHAR)pInfo->m_pbContent));
        }

        if (pInfo->m_pbContent != NULL) {
            GlobalFree((HGLOBAL)pInfo->m_pbContent);
            pInfo->m_pbContent = NULL;
        }
    }
    GlobalFree((HGLOBAL)pSorted);

    Tblog("</t:Files>\n");

    LockRelease();
}


//////////////////////////////////////////////////////////////////////////////
//
class OpenFiles
{
  private:
    struct SLOT
    {
        HANDLE      m_hHandle;
        FileInfo *  m_pFile;
        ProcInfo *  m_pProc;
    };

  private:
    static CRITICAL_SECTION s_csLock;
    static DWORD            s_nHandles;
    static SLOT             s_rHandles[4049];

  private:
    static SLOT& HashToSlot(HANDLE handle)
    {
        return s_rHandles[((DWORD_PTR)handle) % ARRAYSIZE(s_rHandles)];
    }

    static VOID LockAcquire()
    {
        EnterCriticalSection(&s_csLock);
    }

    static VOID LockRelease()
    {
        LeaveCriticalSection(&s_csLock);
    }

  public:
    static VOID Initialize();

    static VOID SetWrite(HANDLE hFile, DWORD cbData)
    {
        SLOT& slot = HashToSlot(hFile);
        if (slot.m_hHandle == hFile) {
            slot.m_pFile->m_fWrite = TRUE;
            slot.m_pFile->m_cbWrite += cbData;
        }
    }

    static VOID SetRead(HANDLE hFile, DWORD cbData)
    {
        SLOT& slot = HashToSlot(hFile);
        if (slot.m_hHandle == hFile) {
            slot.m_pFile->m_fRead = TRUE;
            slot.m_pFile->m_cbRead += cbData;
        }
    }

    static BOOL Forget(HANDLE handle);
    static BOOL Remember(HANDLE hFile, FileInfo *pInfo);
    static BOOL Remember(HANDLE hProc, ProcInfo *pInfo);
    static FileInfo * RecallFile(HANDLE hFile);
    static ProcInfo * RecallProc(HANDLE hProc);
};

CRITICAL_SECTION    OpenFiles::s_csLock;  // Guards access to OpenFile stuctures.
DWORD               OpenFiles::s_nHandles = 0;
OpenFiles::SLOT     OpenFiles::s_rHandles[4049];

VOID OpenFiles::Initialize()
{
    InitializeCriticalSection(&s_csLock);
    for (DWORD n = 0; n < ARRAYSIZE(s_rHandles); n++) {
        s_rHandles[n].m_hHandle = INVALID_HANDLE_VALUE;
        s_rHandles[n].m_pFile = NULL;
        s_rHandles[n].m_pProc = NULL;
    }
}

BOOL OpenFiles::Forget(HANDLE handle)
{
    LockAcquire();
    OpenFiles::SLOT& slot = HashToSlot(handle);

    if (slot.m_hHandle == handle    ) {
        slot.m_hHandle = INVALID_HANDLE_VALUE;
        slot.m_pFile = NULL;
        slot.m_pProc = NULL;
        s_nHandles--;
    }
    LockRelease();
    return FALSE;
}

BOOL OpenFiles::Remember(HANDLE hFile, FileInfo *pFile)
{
    LockAcquire();

    OpenFiles::SLOT& slot = HashToSlot(hFile);
    if (slot.m_hHandle != hFile && slot.m_hHandle != INVALID_HANDLE_VALUE) {
        // hash collision
        DEBUG_BREAK();
    }

    slot.m_hHandle = hFile;
    slot.m_pFile = pFile;
    slot.m_pProc = NULL;
    s_nHandles++;

    LockRelease();

    return TRUE;
}

BOOL OpenFiles::Remember(HANDLE hProc, ProcInfo *pProc)
{
    LockAcquire();

    OpenFiles::SLOT& slot = HashToSlot(hProc);
    if (slot.m_hHandle != hProc && slot.m_hHandle != INVALID_HANDLE_VALUE) {
        // hash collision
        DEBUG_BREAK();
    }

    slot.m_hHandle = hProc;
    slot.m_pProc = pProc;
    slot.m_pFile = NULL;
    s_nHandles++;

    LockRelease();

    return TRUE;
}

FileInfo * OpenFiles::RecallFile(HANDLE hFile)
{
    LockAcquire();

    OpenFiles::SLOT& slot = HashToSlot(hFile);

    if (slot.m_hHandle == hFile) {
        LockRelease();
        return slot.m_pFile;
    }
    LockRelease();
    return NULL;
}

ProcInfo * OpenFiles::RecallProc(HANDLE hProc)
{
    LockAcquire();

    OpenFiles::SLOT& slot = HashToSlot(hProc);

    if (slot.m_hHandle == hProc) {
        LockRelease();
        return slot.m_pProc;
    }
    LockRelease();
    return NULL;
}

///////////////////////////////////////////////////////////////////// VPrintf.
//
// Completely side-effect free printf replacement (but no FP numbers).
//
static PCHAR do_base(PCHAR pszOut, UINT64 nValue, UINT nBase, PCSTR pszDigits)
{
    CHAR szTmp[96];
    int nDigit = sizeof(szTmp)-2;
    for (; nDigit >= 0; nDigit--) {
        szTmp[nDigit] = pszDigits[nValue % nBase];
        nValue /= nBase;
    }
    for (nDigit = 0; nDigit < sizeof(szTmp) - 2 && szTmp[nDigit] == '0'; nDigit++) {
        // skip leading zeros.
    }
    for (; nDigit < sizeof(szTmp) - 1; nDigit++) {
        *pszOut++ = szTmp[nDigit];
    }
    *pszOut = '\0';
    return pszOut;
}

static PCHAR do_str(PCHAR pszOut, PCHAR pszEnd, PCSTR pszIn)
{
    while (*pszIn && pszOut < pszEnd) {
        *pszOut++ = *pszIn++;
    }
    *pszOut = '\0';
    return pszOut;
}

static PCHAR do_wstr(PCHAR pszOut, PCHAR pszEnd, PCWSTR pszIn)
{
    while (*pszIn && pszOut < pszEnd) {
        *pszOut++ = (CHAR)*pszIn++;
    }
    *pszOut = '\0';
    return pszOut;
}

static PCHAR do_estr(PCHAR pszOut, PCHAR pszEnd, PCSTR pszIn)
{
    while (*pszIn && pszOut < pszEnd) {
        if (*pszIn == '<') {
            if (pszOut + 4 > pszEnd) {
                break;
            }
            pszIn++;
            *pszOut++ = '&';
            *pszOut++ = 'l';
            *pszOut++ = 't';
            *pszOut++ = ';';
        }
        else if (*pszIn == '>') {
            if (pszOut + 4 > pszEnd) {
                break;
            }
            pszIn++;
            *pszOut++ = '&';
            *pszOut++ = 'g';
            *pszOut++ = 't';
            *pszOut++ = ';';
        }
        else if (*pszIn == '&') {
            if (pszOut + 5 > pszEnd) {
                break;
            }
            pszIn++;
            *pszOut++ = '&';
            *pszOut++ = 'a';
            *pszOut++ = 'm';
            *pszOut++ = 'p';
            *pszOut++ = ';';
        }
        else if (*pszIn == '\"') {
            if (pszOut + 6 > pszEnd) {
                break;
            }
            pszIn++;
            *pszOut++ = '&';
            *pszOut++ = 'q';
            *pszOut++ = 'u';
            *pszOut++ = 'o';
            *pszOut++ = 't';
            *pszOut++ = ';';
        }
        else if (*pszIn == '\'') {
            if (pszOut + 6 > pszEnd) {
                break;
            }
            pszIn++;
            *pszOut++ = '&';
            *pszOut++ = 'a';
            *pszOut++ = 'p';
            *pszOut++ = 'o';
            *pszOut++ = 's';
            *pszOut++ = ';';
        }
        else if (*pszIn  < ' ') {
            BYTE c = (BYTE)(*pszIn++);
            if (c < 10 && pszOut + 4 <= pszEnd) {
                *pszOut++ = '&';
                *pszOut++ = '#';
                *pszOut++ = '0' + (c % 10);
                *pszOut++ = ';';
            }
            else if (c < 100 && pszOut + 5 <= pszEnd) {
                *pszOut++ = '&';
                *pszOut++ = '#';
                *pszOut++ = '0' + ((c / 10) % 10);
                *pszOut++ = '0' + (c % 10);
                *pszOut++ = ';';
            }
            else if (c < 1000 && pszOut + 6 <= pszEnd) {
                *pszOut++ = '&';
                *pszOut++ = '#';
                *pszOut++ = '0' + ((c / 100) % 10);
                *pszOut++ = '0' + ((c / 10) % 10);
                *pszOut++ = '0' + (c % 10);
                *pszOut++ = ';';
            }
            else {
                break;
            }
        }
        else {
            *pszOut++ = *pszIn++;
        }
    }
    *pszOut = '\0';
    return pszOut;
}

static PCHAR do_ewstr(PCHAR pszOut, PCHAR pszEnd, PCWSTR pszIn)
{
    while (*pszIn && pszOut < pszEnd) {
        if (*pszIn == '<') {
            if (pszOut + 4 > pszEnd) {
                break;
            }
            pszIn++;
            *pszOut++ = '&';
            *pszOut++ = 'l';
            *pszOut++ = 't';
            *pszOut++ = ';';
        }
        else if (*pszIn == '>') {
            if (pszOut + 4 > pszEnd) {
                break;
            }
            pszIn++;
            *pszOut++ = '&';
            *pszOut++ = 'g';
            *pszOut++ = 't';
            *pszOut++ = ';';
        }
        else if (*pszIn == '&') {
            if (pszOut + 5 > pszEnd) {
                break;
            }
            pszIn++;
            *pszOut++ = '&';
            *pszOut++ = 'a';
            *pszOut++ = 'm';
            *pszOut++ = 'p';
            *pszOut++ = ';';
        }
        else if (*pszIn == '\"') {
            if (pszOut + 6 > pszEnd) {
                break;
            }
            pszIn++;
            *pszOut++ = '&';
            *pszOut++ = 'q';
            *pszOut++ = 'u';
            *pszOut++ = 'o';
            *pszOut++ = 't';
            *pszOut++ = ';';
        }
        else if (*pszIn == '\'') {
            if (pszOut + 6 > pszEnd) {
                break;
            }
            pszIn++;
            *pszOut++ = '&';
            *pszOut++ = 'a';
            *pszOut++ = 'p';
            *pszOut++ = 'o';
            *pszOut++ = 's';
            *pszOut++ = ';';
        }
        else if (*pszIn  < ' ' || *pszIn > 127) {
            WCHAR c = *pszIn++;
            if (c < 10 && pszOut + 4 <= pszEnd) {
                *pszOut++ = '&';
                *pszOut++ = '#';
                *pszOut++ = '0' + (CHAR)(c % 10);
                *pszOut++ = ';';
            }
            else if (c < 100 && pszOut + 5 <= pszEnd) {
                *pszOut++ = '&';
                *pszOut++ = '#';
                *pszOut++ = '0' + (CHAR)((c / 10) % 10);
                *pszOut++ = '0' + (CHAR)(c % 10);
                *pszOut++ = ';';
            }
            else if (c < 1000 && pszOut + 6 <= pszEnd) {
                *pszOut++ = '&';
                *pszOut++ = '#';
                *pszOut++ = '0' + (CHAR)((c / 100) % 10);
                *pszOut++ = '0' + (CHAR)((c / 10) % 10);
                *pszOut++ = '0' + (CHAR)(c % 10);
                *pszOut++ = ';';
            }
            else {
                break;
            }
        }
        else {
            *pszOut++ = (CHAR)*pszIn++;
        }
    }
    *pszOut = '\0';
    return pszOut;
}

#if _MSC_VER >= 1900
#pragma warning(push)
#pragma warning(disable:4456) // declaration hides previous local declaration
#endif

VOID VSafePrintf(PCSTR pszMsg, va_list args, PCHAR pszBuffer, LONG cbBuffer)
{
    PCHAR pszOut = pszBuffer;
    PCHAR pszEnd = pszBuffer + cbBuffer - 1;
    pszBuffer[0] = '\0';

    __try {
        while (*pszMsg && pszOut < pszEnd) {
            if (*pszMsg == '%') {
                CHAR szHead[4] = "";
                INT nLen;
                INT nWidth = 0;
                INT nPrecision = 0;
                BOOL fLeft = FALSE;
                BOOL fPositive = FALSE;
                BOOL fPound = FALSE;
                BOOL fBlank = FALSE;
                BOOL fZero = FALSE;
                BOOL fDigit = FALSE;
                BOOL fSmall = FALSE;
                BOOL fLarge = FALSE;
                BOOL f64Bit = FALSE;
                PCSTR pszArg = pszMsg;

                pszMsg++;

                for (; (*pszMsg == '-' ||
                        *pszMsg == '+' ||
                        *pszMsg == '#' ||
                        *pszMsg == ' ' ||
                        *pszMsg == '0'); pszMsg++) {
                    switch (*pszMsg) {
                      case '-': fLeft = TRUE; break;
                      case '+': fPositive = TRUE; break;
                      case '#': fPound = TRUE; break;
                      case ' ': fBlank = TRUE; break;
                      case '0': fZero = TRUE; break;
                    }
                }

                if (*pszMsg == '*') {
                    nWidth = va_arg(args, INT);
                    pszMsg++;
                }
                else {
                    while (*pszMsg >= '0' && *pszMsg <= '9') {
                        nWidth = nWidth * 10 + (*pszMsg++ - '0');
                    }
                }
                if (*pszMsg == '.') {
                    pszMsg++;
                    fDigit = TRUE;
                    if (*pszMsg == '*') {
                        nPrecision = va_arg(args, INT);
                        pszMsg++;
                    }
                    else {
                        while (*pszMsg >= '0' && *pszMsg <= '9') {
                            nPrecision = nPrecision * 10 + (*pszMsg++ - '0');
                        }
                    }
                }

                if (*pszMsg == 'h') {
                    fSmall = TRUE;
                    pszMsg++;
                }
                else if (*pszMsg == 'l') {
                    fLarge = TRUE;
                    pszMsg++;
                }
                else if (*pszMsg == 'I' && pszMsg[1] == '6' && pszMsg[2] == '4') {
                    f64Bit = TRUE;
                    pszMsg += 3;
                }

                if (*pszMsg == 's' || *pszMsg == 'e' || *pszMsg == 'c') {
                    // We ignore the length, precision, and alignment
                    // to avoid using a temporary buffer.

                    if (*pszMsg == 's') { // [GalenH] need to not use temp.
                        PVOID pvData = va_arg(args, PVOID);

                        pszMsg++;

                        if (fSmall) {
                            fLarge = FALSE;
                        }

                        __try {
                            if (pvData == NULL) {
                                pszOut = do_str(pszOut, pszEnd, "-NULL-");
                            }
                            else if (fLarge) {
                                pszOut = do_wstr(pszOut, pszEnd, (PWCHAR)pvData);
                            }
                            else {
                                pszOut = do_str(pszOut, pszEnd, (PCHAR)pvData);
                            }
                        } __except(EXCEPTION_EXECUTE_HANDLER) {
                            pszOut = do_str(pszOut, pszEnd, "-");
                            pszOut = do_base(pszOut, (UINT64)pvData, 16,
                                             "0123456789ABCDEF");
                            pszOut = do_str(pszOut, pszEnd, "-");
                        }
                    }
                    else if (*pszMsg == 'e')    {   // Escape the string.
                        PVOID pvData = va_arg(args, PVOID);

                        pszMsg++;

                        if (fSmall) {
                            fLarge = FALSE;
                        }

                        __try {
                            if (pvData == NULL) {
                                pszOut = do_str(pszOut, pszEnd, "-NULL-");
                            }
                            else if (fLarge) {
                                pszOut = do_ewstr(pszOut, pszEnd, (PWCHAR)pvData);
                            }
                            else {
                                pszOut = do_estr(pszOut, pszEnd, (PCHAR)pvData);
                            }
                        } __except(EXCEPTION_EXECUTE_HANDLER) {
                            pszOut = do_str(pszOut, pszEnd, "-");
                            pszOut = do_base(pszOut, (UINT64)pvData, 16,
                                             "0123456789ABCDEF");
                            pszOut = do_str(pszOut, pszEnd, "-");
                        }
                    }
                    else {
                        CHAR szTemp[2];
                        pszMsg++;

                        szTemp[0] = (CHAR)va_arg(args, INT);
                        szTemp[1] = '\0';
                        pszOut = do_str(pszOut, pszEnd, szTemp);
                    }
                }
                else if (*pszMsg == 'd' || *pszMsg == 'i' || *pszMsg == 'o' ||
                         *pszMsg == 'x' || *pszMsg == 'X' || *pszMsg == 'b' ||
                         *pszMsg == 'u') {
                    CHAR szTemp[128];
                    UINT64 value;
                    if (f64Bit) {
                        value = va_arg(args, UINT64);
                    }
                    else {
                        value = va_arg(args, UINT);
                    }

                    if (*pszMsg == 'x') {
                        pszMsg++;
                        nLen = (int)(do_base(szTemp, value, 16, "0123456789abcdef") - szTemp);
                        if (fPound && value) {
                            do_str(szHead, szHead + sizeof(szHead) - 1, "0x");
                        }
                    }
                    else if (*pszMsg == 'X') {
                        pszMsg++;
                        nLen = (int)(do_base(szTemp, value, 16, "0123456789ABCDEF") - szTemp);
                        if (fPound && value) {
                            do_str(szHead, szHead + sizeof(szHead) - 1, "0X");
                        }
                    }
                    else if (*pszMsg == 'd') {
                        pszMsg++;
                        if ((INT64)value < 0) {
                            value = -(INT64)value;
                            do_str(szHead, szHead + sizeof(szHead) - 1, "-");
                        }
                        else if (fPositive) {
                            if (value > 0) {
                                do_str(szHead, szHead + sizeof(szHead) - 1, "+");
                            }
                        }
                        else if (fBlank) {
                            if (value > 0) {
                                do_str(szHead, szHead + sizeof(szHead) - 1, " ");
                            }
                        }
                        nLen = (int)(do_base(szTemp, value, 10, "0123456789") - szTemp);
                        nPrecision = 0;
                    }
                    else if (*pszMsg == 'u') {
                        pszMsg++;
                        nLen = (int)(do_base(szTemp, value, 10, "0123456789") - szTemp);
                        nPrecision = 0;
                    }
                    else if (*pszMsg == 'o') {
                        pszMsg++;
                        nLen = (int)(do_base(szTemp, value, 8, "01234567") - szTemp);
                        nPrecision = 0;

                        if (fPound && value) {
                            do_str(szHead, szHead + sizeof(szHead) - 1, "0");
                        }
                    }
                    else if (*pszMsg == 'b') {
                        pszMsg++;
                        nLen = (int)(do_base(szTemp, value, 2, "01") - szTemp);
                        nPrecision = 0;

                        if (fPound && value) {
                            do_str(szHead, szHead + sizeof(szHead) - 1, "0b");
                        }
                    }
                    else {
                        pszMsg++;
                        if ((INT64)value < 0) {
                            value = -(INT64)value;
                            do_str(szHead, szHead + sizeof(szHead) - 1, "-");
                        }
                        else if (fPositive) {
                            if (value > 0) {
                                do_str(szHead, szHead + sizeof(szHead) - 1, "+");
                            }
                        }
                        else if (fBlank) {
                            if (value > 0) {
                                do_str(szHead, szHead + sizeof(szHead) - 1, " ");
                            }
                        }
                        nLen = (int)(do_base(szTemp, value, 10, "0123456789") - szTemp);
                        nPrecision = 0;
                    }

                    INT nHead = 0;
                    for (; szHead[nHead]; nHead++) {
                        // Count characters in head string.
                    }

                    if (fLeft) {
                        if (nHead) {
                            pszOut = do_str(pszOut, pszEnd, szHead);
                            nLen += nHead;
                        }
                        pszOut = do_str(pszOut, pszEnd, szTemp);
                        for (; nLen < nWidth && pszOut < pszEnd; nLen++) {
                            *pszOut++ = ' ';
                        }
                    }
                    else if (fZero) {
                        if (nHead) {
                            pszOut = do_str(pszOut, pszEnd, szHead);
                            nLen += nHead;
                        }
                        for (; nLen < nWidth && pszOut < pszEnd; nLen++) {
                            *pszOut++ = '0';
                        }
                        pszOut = do_str(pszOut, pszEnd, szTemp);
                    }
                    else {
                        if (nHead) {
                            nLen += nHead;
                        }
                        for (; nLen < nWidth && pszOut < pszEnd; nLen++) {
                            *pszOut++ = ' ';
                        }
                        if (nHead) {
                            pszOut = do_str(pszOut, pszEnd, szHead);
                        }
                        pszOut = do_str(pszOut, pszEnd, szTemp);
                    }
                }
                else if (*pszMsg == 'p') {
                    CHAR szTemp[64];
                    ULONG_PTR value;
                    value = va_arg(args, ULONG_PTR);

                    if (*pszMsg == 'p') {
                        pszMsg++;
                        nLen = (int)(do_base(szTemp, (UINT64)value, 16, "0123456789abcdef") - szTemp);
                        if (fPound && value) {
                            do_str(szHead, szHead + sizeof(szHead) - 1, "0x");
                        }
                    }
                    else {
                        pszMsg++;
                        nLen = (int)(do_base(szTemp, (UINT64)value, 16, "0123456789ABCDEF") - szTemp);
                        if (fPound && value) {
                            do_str(szHead, szHead + sizeof(szHead) - 1, "0x");
                        }
                    }

                    INT nHead = 0;
                    for (; szHead[nHead]; nHead++) {
                        // Count characters in head string.
                    }

                    if (nHead) {
                        pszOut = do_str(pszOut, pszEnd, szHead);
                        nLen += nHead;
                    }
                    for (; nLen < nWidth && pszOut < pszEnd; nLen++) {
                        *pszOut++ = '0';
                    }
                    pszOut = do_str(pszOut, pszEnd, szTemp);
                }
                else {
                    pszMsg++;
                    while (pszArg < pszMsg && pszOut < pszEnd) {
                        *pszOut++ = *pszArg++;
                    }
                }
            }
            else {
                if (pszOut < pszEnd) {
                    *pszOut++ = *pszMsg++;
                }
            }
        }
        *pszOut = '\0';
        pszBuffer[cbBuffer - 1] = '\0';
    } __except(EXCEPTION_EXECUTE_HANDLER) {
        PCHAR pszOut = pszBuffer;
        *pszOut = '\0';
        pszOut = do_str(pszOut, pszEnd, "-exception:");
        pszOut = do_base(pszOut, (UINT64)GetExceptionCode(), 10, "0123456789");
        pszOut = do_str(pszOut, pszEnd, "-");
    }
}

#if _MSC_VER >= 1900
#pragma warning(pop)
#endif

PCHAR SafePrintf(PCHAR pszBuffer, LONG cbBuffer, PCSTR pszMsg, ...)
{
    va_list args;
    va_start(args, pszMsg);
    VSafePrintf(pszMsg, args, pszBuffer, cbBuffer);
    va_end(args);

    while (*pszBuffer) {
        pszBuffer++;
    }
    return pszBuffer;
}

//////////////////////////////////////////////////////////////////////////////
//
BOOL TblogOpen()
{
    EnterCriticalSection(&s_csPipe);

    WCHAR wzPipe[256];
    StringCchPrintfW(wzPipe, ARRAYSIZE(wzPipe), L"%ls.%d", TBLOG_PIPE_NAMEW, s_nTraceProcessId);

    for (int retries = 0; retries < 10; retries++) {
        WaitNamedPipeW(wzPipe, 10000); // Wait up to 10 seconds for a pipe to appear.

        s_hPipe = Real_CreateFileW(wzPipe, GENERIC_WRITE, 0, NULL, OPEN_EXISTING, 0, NULL);
        if (s_hPipe != INVALID_HANDLE_VALUE) {
            DWORD dwMode = PIPE_READMODE_MESSAGE;
            if (SetNamedPipeHandleState(s_hPipe, &dwMode, NULL, NULL)) {
                LeaveCriticalSection(&s_csPipe);
                return TRUE;
            }
        }
    }

    LeaveCriticalSection(&s_csPipe);

    // Couldn't open pipe.
    DEBUG_BREAK();
    Real_ExitProcess(9990);
    return FALSE;
}

VOID TblogV(PCSTR pszMsgf, va_list args)
{
    if (s_hPipe == INVALID_HANDLE_VALUE) {
        return;
    }

    EnterCriticalSection(&s_csPipe);

    DWORD cbWritten = 0;

    PCHAR pszBuf = s_rMessage.szMessage;
    VSafePrintf(pszMsgf, args,
                pszBuf, (int)(s_rMessage.szMessage + sizeof(s_rMessage.szMessage) - pszBuf));

    PCHAR pszEnd = s_rMessage.szMessage;
    for (; *pszEnd; pszEnd++) {
        // no internal contents.
    }
    s_rMessage.nBytes = (DWORD)(pszEnd - ((PCSTR)&s_rMessage));

    // If the write fails, then we abort
    if (s_hPipe != INVALID_HANDLE_VALUE) {
        if (!Real_WriteFile(s_hPipe, &s_rMessage, s_rMessage.nBytes, &cbWritten, NULL)) {
            Real_ExitProcess(9991);
        }
    }

    LeaveCriticalSection(&s_csPipe);
}

VOID Tblog(PCSTR pszMsgf, ...)
{
    if (s_hPipe == INVALID_HANDLE_VALUE) {
        return;
    }

    va_list args;
    va_start(args, pszMsgf);
    TblogV(pszMsgf, args);
    va_end(args);
}

VOID TblogClose()
{
    EnterCriticalSection(&s_csPipe);

    if (s_hPipe != INVALID_HANDLE_VALUE) {
        DWORD cbWritten = 0;

        s_rMessage.nBytes = 0;

        Real_WriteFile(s_hPipe, &s_rMessage, 4, &cbWritten, NULL);
        FlushFileBuffers(s_hPipe);
        Real_CloseHandle(s_hPipe);
        s_hPipe = INVALID_HANDLE_VALUE;
    }

    LeaveCriticalSection(&s_csPipe);
}

/////////////////////////////////////////////////////////////
// Detours
//
static BOOL IsInherited(HANDLE hHandle)
{
    DWORD dwFlags;

    if (GetHandleInformation(hHandle, &dwFlags)) {
        return (dwFlags & HANDLE_FLAG_INHERIT) ? TRUE : FALSE;
    }
    return FALSE;
}

static void SaveStdHandleName(HANDLE hFile, PWCHAR pwzBuffer, BOOL *fAppend)
{
    pwzBuffer[0] = '\0';

    if ((hFile != INVALID_HANDLE_VALUE) && IsInherited(hFile)) {
        FileInfo * pInfo = OpenFiles::RecallFile(hFile);
        if (pInfo) {
            Copy(pwzBuffer, pInfo->m_pwzPath);
            if (pInfo->m_fAppend && fAppend != NULL) {
                *fAppend = TRUE;
            }
        }
    }
}

static void LoadStdHandleName(DWORD id, PCWSTR pwzBuffer, BOOL fAppend)
{
    HANDLE hFile = GetStdHandle(id);

    if ((hFile != INVALID_HANDLE_VALUE) && pwzBuffer[0] != '\0') {
        FileInfo *pInfo = FileNames::FindPartial(pwzBuffer);
        if (fAppend) {
            pInfo->m_fAppend = TRUE;
        }
        OpenFiles::Remember(hFile, pInfo);
    }
}

BOOL CreateProcessInternals(HANDLE hProcess, DWORD nProcessId, PCHAR pszId,
                            HANDLE hStdin, HANDLE hStdout, HANDLE hStderr)
{
    EnterCriticalSection(&s_csChildPayload);

    ProcInfo *proc = Procs::Create(hProcess, nProcessId);
    OpenFiles::Remember(hProcess, proc);

    ZeroMemory(&s_ChildPayload, sizeof(s_ChildPayload));
    CopyMemory(&s_ChildPayload, &s_Payload, sizeof(s_ChildPayload));

    s_ChildPayload.nParentProcessId = GetCurrentProcessId();
    s_ChildPayload.rGeneology[s_ChildPayload.nGeneology]
        = (DWORD)InterlockedIncrement(&s_nChildCnt);
    s_ChildPayload.nGeneology++;

    SaveStdHandleName(hStdin, s_ChildPayload.wzStdin, NULL);
    SaveStdHandleName(hStdout, s_ChildPayload.wzStdout, &s_ChildPayload.fStdoutAppend);
    SaveStdHandleName(hStderr, s_ChildPayload.wzStderr, &s_ChildPayload.fStderrAppend);

    DetourCopyPayloadToProcess(hProcess, s_guidTrace, &s_ChildPayload, sizeof(s_ChildPayload));

    for (DWORD i = 0; i < s_ChildPayload.nGeneology; i++) {
        pszId = SafePrintf(pszId, 16, "%d.", s_ChildPayload.rGeneology[i]);
    }
    *pszId = '\0';

    LeaveCriticalSection(&s_csChildPayload);

    return TRUE;
}

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
    EnterFunc();

    if (lpCommandLine == NULL) {
        lpCommandLine = (LPWSTR)lpApplicationName;
    }

    CHAR szProc[MAX_PATH];
    BOOL rv = 0;
    __try {
        LPPROCESS_INFORMATION ppi = lpProcessInformation;
        PROCESS_INFORMATION pi;
        if (ppi == NULL) {
            ppi = &pi;
        }

        rv = DetourCreateProcessWithDllExW(lpApplicationName,
                                           lpCommandLine,
                                           lpProcessAttributes,
                                           lpThreadAttributes,
                                           bInheritHandles,
                                           dwCreationFlags | CREATE_SUSPENDED,
                                           lpEnvironment,
                                           lpCurrentDirectory,
                                           lpStartupInfo,
                                           ppi,
                                           s_szDllPath,
                                           Real_CreateProcessW);

        if (rv) {
            HANDLE hStdin = GetStdHandle(STD_INPUT_HANDLE);
            HANDLE hStdout = GetStdHandle(STD_OUTPUT_HANDLE);
            HANDLE hStderr = GetStdHandle(STD_ERROR_HANDLE);

            if (lpStartupInfo != NULL && (lpStartupInfo->dwFlags & STARTF_USESTDHANDLES) != 0) {
                hStdin = lpStartupInfo->hStdInput;
                hStdout = lpStartupInfo->hStdOutput;
                hStderr = lpStartupInfo->hStdError;
            }
            CreateProcessInternals(ppi->hProcess, ppi->dwProcessId,
                                   szProc, hStdin, hStdout, hStderr);

            Print("<t:Child id=\"::%hs::\">\n", szProc);

            WCHAR wzPath[MAX_PATH];
            FileInfo *pInfo = NULL;
            if (lpApplicationName == NULL) {
                PWCHAR pwzDst = wzPath;
                PWCHAR pwzSrc = lpCommandLine;

                if (*pwzSrc == '\"') {
                    WCHAR cQuote = *pwzSrc++;

                    while (*pwzSrc && *pwzSrc != cQuote) {
                        *pwzDst++ = *pwzSrc++;
                    }
                    *pwzDst++ = '\0';
                }
                else {
                    while (*pwzSrc && *pwzSrc != ' ' && *pwzSrc != '\t') {
                        if (*pwzSrc == '\t') {
                            *pwzSrc = ' ';
                        }
                        *pwzDst++ = *pwzSrc++;
                    }
                    *pwzDst++ = '\0';
                }
                pInfo = FileNames::FindPartial(wzPath);
            }
            else {
                pInfo = FileNames::FindPartial(lpApplicationName);
            }

            Print("<t:Executable>%ls</t:Executable>\n",
                  FileNames::ParameterizeName(wzPath, ARRAYSIZE(wzPath), pInfo));
            Print("<t:Line>%le</t:Line>\n", lpCommandLine);
            Print("</t:Child>\n");

            if (pInfo) {
                pInfo->m_fAbsorbed = true;
            }

            if (!(dwCreationFlags & CREATE_SUSPENDED)) {
                ResumeThread(ppi->hThread);
            }

            if (ppi == &pi) {
                Real_CloseHandle(ppi->hThread);
                Real_CloseHandle(ppi->hProcess);
            }
        }
    } __finally {
        ExitFunc();
        if (!rv) {
            Print("<!-- Warning: CreateProcessW failed %d: %ls; %ls -->\n",
                  GetLastError(), lpApplicationName, lpCommandLine);
        }
    }
    return rv;
}

BOOL WINAPI Mine_CreateProcessA(LPCSTR lpApplicationName,
                                LPSTR lpCommandLine,
                                LPSECURITY_ATTRIBUTES lpProcessAttributes,
                                LPSECURITY_ATTRIBUTES lpThreadAttributes,
                                BOOL bInheritHandles,
                                DWORD dwCreationFlags,
                                LPVOID lpEnvironment,
                                LPCSTR lpCurrentDirectory,
                                LPSTARTUPINFOA lpStartupInfo,
                                LPPROCESS_INFORMATION lpProcessInformation)
{
    EnterFunc();

    if (lpCommandLine == NULL) {
        lpCommandLine = (LPSTR)lpApplicationName;
    }

    CHAR szProc[MAX_PATH];
    BOOL rv = 0;
    __try {
        LPPROCESS_INFORMATION ppi = lpProcessInformation;
        PROCESS_INFORMATION pi;
        if (ppi == NULL) {
            ppi = &pi;
        }

        rv = DetourCreateProcessWithDllExA(lpApplicationName,
                                           lpCommandLine,
                                           lpProcessAttributes,
                                           lpThreadAttributes,
                                           bInheritHandles,
                                           dwCreationFlags | CREATE_SUSPENDED,
                                           lpEnvironment,
                                           lpCurrentDirectory,
                                           lpStartupInfo,
                                           ppi,
                                           s_szDllPath,
                                           Real_CreateProcessA);

        if (rv) {
            HANDLE hStdin = GetStdHandle(STD_INPUT_HANDLE);
            HANDLE hStdout = GetStdHandle(STD_OUTPUT_HANDLE);
            HANDLE hStderr = GetStdHandle(STD_ERROR_HANDLE);

            if (lpStartupInfo != NULL && (lpStartupInfo->dwFlags & STARTF_USESTDHANDLES) != 0) {
                hStdin = lpStartupInfo->hStdInput;
                hStdout = lpStartupInfo->hStdOutput;
                hStderr = lpStartupInfo->hStdError;
            }
            CreateProcessInternals(ppi->hProcess, ppi->dwProcessId,
                                   szProc, hStdin, hStdout, hStderr);

            Print("<t:Child id=\"::%hs::\">\n", szProc);

            WCHAR wzPath[MAX_PATH];
            FileInfo *pInfo = NULL;
            if (lpApplicationName == NULL) {
                PCHAR pszDst = szProc;
                PCHAR pszSrc = lpCommandLine;

                if (*pszSrc == '\"') {
                    CHAR cQuote = *pszSrc++;

                    while (*pszSrc && *pszSrc != cQuote) {
                        *pszDst++ = *pszSrc++;
                    }
                    *pszDst++ = '\0';
                }
                else {
                    while (*pszSrc && *pszSrc != ' ' && *pszSrc != '\t') {
                        if (*pszSrc == '\t') {
                            *pszSrc = ' ';
                        }
                        *pszDst++ = *pszSrc++;
                    }
                    *pszDst++ = '\0';
                }
                pInfo = FileNames::FindPartial(szProc);
            }
            else {
                pInfo = FileNames::FindPartial(lpApplicationName);
            }

            Print("<t:Executable>%ls</t:Executable>\n",
                  FileNames::ParameterizeName(wzPath, ARRAYSIZE(wzPath), pInfo));
            Print("<t:Line>%he</t:Line>\n", lpCommandLine);
            Print("</t:Child>\n");

            if (pInfo) {
                pInfo->m_fAbsorbed = true;
            }

            if (!(dwCreationFlags & CREATE_SUSPENDED)) {
                ResumeThread(ppi->hThread);
            }
            if (ppi == &pi) {
                Real_CloseHandle(ppi->hThread);
                Real_CloseHandle(ppi->hProcess);
            }
        }
    } __finally {
        ExitFunc();
        if (!rv) {
            Print("<!-- Warning: CreateProcessA failed %d: %hs; %hs -->\n",
                  GetLastError(), lpApplicationName, lpCommandLine);
        }
    }
    return rv;
}

//
//////////////////////////////////////////////////////////////////////////////

BOOL WINAPI Mine_CopyFileExA(LPCSTR a0,
                             LPCSTR a1,
                             LPPROGRESS_ROUTINE a2,
                             LPVOID a3,
                             LPBOOL a4,
                             DWORD a5)
{
    EnterFunc();

    BOOL rv = 0;
    __try {
        rv = Real_CopyFileExA(a0, a1, a2, a3, a4, a5);
    } __finally {
        ExitFunc();
        if (rv) {
#if 0
            Print("<!-- CopyFileExA %he to %he -->\n", a0, a1);
#endif
            NoteRead(a0);
            NoteWrite(a1);
        }
    };
    return rv;
}

BOOL WINAPI Mine_CopyFileExW(LPCWSTR a0,
                             LPCWSTR a1,
                             LPPROGRESS_ROUTINE a2,
                             LPVOID a3,
                             LPBOOL a4,
                             DWORD a5)
{
    EnterFunc();

    BOOL rv = 0;
    __try {
#if 0
        Print("\n");
        Print("<!-- CopyFileExW %le to %le before -->\n", a0, a1);
#endif
        rv = Real_CopyFileExW(a0, a1, a2, a3, a4, a5);
    } __finally {
        ExitFunc();
        if (rv) {
#if 0
            Print("<!-- CopyFileExW %le to %le -->\n", a0, a1);
#endif
            NoteRead(a0);
            NoteWrite(a1);
        }
    };
    return rv;
}

BOOL WINAPI Mine_PrivCopyFileExW(LPCWSTR a0,
                                 LPCWSTR a1,
                                 LPPROGRESS_ROUTINE a2,
                                 LPVOID a3,
                                 LPBOOL a4,
                                 DWORD a5)
{
    EnterFunc();

    BOOL rv = 0;
    __try {
        rv = Real_PrivCopyFileExW(a0, a1, a2, a3, a4, a5);
    } __finally {
        ExitFunc();
        if (rv) {
#if 0
            Print("<!-- PrivCopyFileExW %le to %le -->\n", a0, a1);
#endif
            NoteRead(a0);
            NoteWrite(a1);
        }
    };
    return rv;
}

BOOL WINAPI Mine_CreateHardLinkA(LPCSTR a0,
                                 LPCSTR a1,
                                 LPSECURITY_ATTRIBUTES a2)
{
    EnterFunc();

    BOOL rv = 0;
    __try {
        rv = Real_CreateHardLinkA(a0, a1, a2);
    } __finally {
        ExitFunc();
        if (rv) {
#if 0
            Print("<!-- CreateHardLinkA %he to %he -->\n", a0, a1);
#endif
            NoteRead(a1);
            NoteWrite(a0);
        }
    };
    return rv;
}

BOOL WINAPI Mine_CreateHardLinkW(LPCWSTR a0,
                                 LPCWSTR a1,
                                 LPSECURITY_ATTRIBUTES a2)
{
    EnterFunc();

    BOOL rv = 0;
    __try {
        rv = Real_CreateHardLinkW(a0, a1, a2);
    } __finally {
        ExitFunc();
        if (rv) {
#if 0
            Print("<!-- CreateHardLinkW %le to %le -->\n", a0, a1);
#endif
            NoteRead(a1);
            NoteWrite(a0);
        }
    };
    return rv;
}

BOOL WINAPI Mine_CloseHandle(HANDLE a0)
{
    /*int nIndent =*/ EnterFunc();

    BOOL rv = 0;
    __try {
        ProcInfo * pProc = OpenFiles::RecallProc(a0);
        if (pProc != NULL) {
            Procs::Close(pProc->m_hProc);
        }

        FileInfo * pFile = OpenFiles::RecallFile(a0);
        if (pFile != NULL) {
            DWORD dwErr = GetLastError();
            pFile->m_cbContent = GetFileSize(a0, NULL);
            if (pFile->m_cbContent == INVALID_FILE_SIZE) {
                pFile->m_cbContent = 0;
            }

            if (pFile->m_fCantRead) {
                if (pFile->m_fRead) {
#if 0
                    Print("<!-- Warning: Removing read from %le -->\n", pFile->m_pwzPath);
#endif
                    pFile->m_fRead = FALSE;
                }
            }

            // Here we should think about reading the file contents as appropriate.
            if (pFile->m_fTemporaryPath && pFile->m_fRead && !pFile->m_fAbsorbed &&
                !pFile->m_fDelete && !pFile->m_fCleanup && !pFile->m_fWrite &&
                pFile->m_pbContent == NULL &&
                pFile->m_cbContent < 16384) {

                pFile->m_pbContent = LoadFile(a0, pFile->m_cbContent);
            }

            SetLastError(dwErr);
        }
        rv = Real_CloseHandle(a0);
    } __finally {
        ExitFunc();
        if (rv /* && nIndent == 0*/) {
            OpenFiles::Forget(a0);
        }
    };
    return rv;
}

BOOL WINAPI Mine_DuplicateHandle(HANDLE hSourceProcessHandle,
                                 HANDLE hSourceHandle,
                                 HANDLE hTargetProcessHandle,
                                 LPHANDLE lpTargetHandle,
                                 DWORD dwDesiredAccess,
                                 BOOL bInheritHandle,
                                 DWORD dwOptions)
{
    HANDLE hTemp = INVALID_HANDLE_VALUE;
    EnterFunc();

    BOOL rv = 0;
    __try {
        if (lpTargetHandle == NULL) {
            lpTargetHandle = &hTemp;
        }
        *lpTargetHandle = INVALID_HANDLE_VALUE;

        rv = Real_DuplicateHandle(hSourceProcessHandle,
                                  hSourceHandle,
                                  hTargetProcessHandle,
                                  lpTargetHandle,
                                  dwDesiredAccess,
                                  bInheritHandle,
                                  dwOptions);
    } __finally {
        ExitFunc();
        if (*lpTargetHandle != INVALID_HANDLE_VALUE) {
            FileInfo *pInfo = OpenFiles::RecallFile(hSourceHandle);
            if (pInfo) {
                OpenFiles::Remember(*lpTargetHandle, pInfo);
            }
        }
    };
    return rv;
}

static LONG s_nPipeCnt = 0;

BOOL WINAPI Mine_CreatePipe(PHANDLE hReadPipe,
                            PHANDLE hWritePipe,
                            LPSECURITY_ATTRIBUTES lpPipeAttributes,
                            DWORD nSize)
{
    HANDLE hRead = INVALID_HANDLE_VALUE;
    HANDLE hWrite = INVALID_HANDLE_VALUE;

    if (hReadPipe == NULL) {
        hReadPipe = &hRead;
    }
    if (hWritePipe == NULL) {
        hWritePipe = &hWrite;
    }

    /*int nIndent = */ EnterFunc();
    BOOL rv = 0;
    __try {
        rv = Real_CreatePipe(hReadPipe, hWritePipe, lpPipeAttributes, nSize);
    } __finally {
        ExitFunc();
        if (rv) {
            CHAR szPipe[128];

            SafePrintf(szPipe, ARRAYSIZE(szPipe), "\\\\.\\PIPE\\Temp.%d.%d",
                       GetCurrentProcessId(),
                       InterlockedIncrement(&s_nPipeCnt));

            FileInfo *pInfo = FileNames::FindPartial(szPipe);

            pInfo->m_fCleanup = TRUE;
            OpenFiles::Remember(*hReadPipe, pInfo);
            OpenFiles::Remember(*hWritePipe, pInfo);
        }
    };
    return rv;
}

BOOL WINAPI Mine_CreateDirectoryW(LPCWSTR a0,
                                  LPSECURITY_ATTRIBUTES a1)
{
    /* int nIndent = */ EnterFunc();
    BOOL rv = 0;
    __try {
        rv = Real_CreateDirectoryW(a0, a1);
    } __finally {
        ExitFunc();
        if (rv) {
            FileInfo *pInfo = FileNames::FindPartial(a0);
            pInfo->m_fDirectory = TRUE;
        }
    };
    return rv;
}

BOOL WINAPI Mine_CreateDirectoryExW(LPCWSTR a0,
                                    LPCWSTR a1,
                                    LPSECURITY_ATTRIBUTES a2)
{
    /* int nIndent = */ EnterFunc();
    BOOL rv = 0;
    __try {
        rv = Real_CreateDirectoryExW(a0, a1, a2);
    } __finally {
        ExitFunc();
        if (rv) {
            FileInfo *pInfo = FileNames::FindPartial(a1);
            pInfo->m_fDirectory = TRUE;
        }
    };
    return rv;
}

HANDLE WINAPI Mine_CreateFileW(LPCWSTR a0,
                               DWORD access,
                               DWORD share,
                               LPSECURITY_ATTRIBUTES a3,
                               DWORD create,
                               DWORD flags,
                               HANDLE a6)
{
    /* int nIndent = */ EnterFunc();
    HANDLE rv = 0;
    __try {
        rv = Real_CreateFileW(a0, access, share, a3, create, flags, a6);
    } __finally {
        ExitFunc();
#if 0
            Print("<!-- CreateFileW(%le, ac=%08x, cr=%08x, fl=%08x -->\n",
                  a0,
                  access,
                  create,
                  flags);
#endif

        if (access != 0 && /* nIndent == 0 && */ rv != INVALID_HANDLE_VALUE) {

            FileInfo *pInfo = FileNames::FindPartial(a0);

            // FILE_FLAG_WRITE_THROUGH              0x80000000
            // FILE_FLAG_OVERLAPPED                 0x40000000
            // FILE_FLAG_NO_BUFFERING               0x20000000
            // FILE_FLAG_RANDOM_ACCESS              0x10000000
            // FILE_FLAG_SEQUENTIAL_SCAN            0x08000000
            // FILE_FLAG_DELETE_ON_CLOSE            0x04000000
            // FILE_FLAG_BACKUP_SEMANTICS           0x02000000
            // FILE_FLAG_POSIX_SEMANTICS            0x01000000
            // FILE_FLAG_OPEN_REPARSE_POINT         0x00200000
            // FILE_FLAG_OPEN_NO_RECALL             0x00100000
            // FILE_FLAG_FIRST_PIPE_INSTANCE        0x00080000
            // FILE_ATTRIBUTE_ENCRYPTED             0x00004000
            // FILE_ATTRIBUTE_NOT_CONTENT_INDEXED   0x00002000
            // FILE_ATTRIBUTE_OFFLINE               0x00001000
            // FILE_ATTRIBUTE_COMPRESSED            0x00000800
            // FILE_ATTRIBUTE_REPARSE_POINT         0x00000400
            // FILE_ATTRIBUTE_SPARSE_FILE           0x00000200
            // FILE_ATTRIBUTE_TEMPORARY             0x00000100
            // FILE_ATTRIBUTE_NORMAL                0x00000080
            // FILE_ATTRIBUTE_DEVICE                0x00000040
            // FILE_ATTRIBUTE_ARCHIVE               0x00000020
            // FILE_ATTRIBUTE_DIRECTORY             0x00000010
            // FILE_ATTRIBUTE_SYSTEM                0x00000004
            // FILE_ATTRIBUTE_HIDDEN                0x00000002
            // FILE_ATTRIBUTE_READONLY              0x00000001

            // CREATE_NEW          1
            // CREATE_ALWAYS       2
            // OPEN_EXISTING       3
            // OPEN_ALWAYS         4
            // TRUNCATE_EXISTING   5

            if (create == CREATE_NEW ||
                create == CREATE_ALWAYS ||
                create == TRUNCATE_EXISTING) {

                if (!pInfo->m_fRead) {
                    pInfo->m_fCantRead = TRUE;
                }
            }
            else if (create == OPEN_EXISTING) {
            }
            else if (create == OPEN_ALWAYS) {
                // pInfo->m_fAppend = TRUE;    // !!!
            }

            if ((flags & FILE_FLAG_DELETE_ON_CLOSE)) {
                pInfo->m_fCleanup = TRUE;
            }

            OpenFiles::Remember(rv, pInfo);
        }
    };
    return rv;
}

HANDLE WINAPI Mine_CreateFileMappingW(HANDLE hFile,
                                      LPSECURITY_ATTRIBUTES a1,
                                      DWORD flProtect,
                                      DWORD a3,
                                      DWORD a4,
                                      LPCWSTR a5)
{
    /* int nIndent = */ EnterFunc();
    HANDLE rv = 0;
    __try {
        rv = Real_CreateFileMappingW(hFile, a1, flProtect, a3, a4, a5);
    } __finally {
        ExitFunc();
        if (rv != INVALID_HANDLE_VALUE) {

            FileInfo *pInfo = OpenFiles::RecallFile(hFile);

            if (pInfo != NULL) {
                switch (flProtect) {
                  case PAGE_READONLY:
                    pInfo->m_fRead = TRUE;
                    break;
                  case PAGE_READWRITE:
                    pInfo->m_fRead = TRUE;
                    pInfo->m_fWrite = TRUE;
                    break;
                  case PAGE_WRITECOPY:
                    pInfo->m_fRead = TRUE;
                    break;
                  case PAGE_EXECUTE_READ:
                    pInfo->m_fRead = TRUE;
                    break;
                  case PAGE_EXECUTE_READWRITE:
                    pInfo->m_fRead = TRUE;
                    pInfo->m_fWrite = TRUE;
                    break;
                }
            }
        }
    };
    return rv;
}

BOOL WINAPI Mine_DeleteFileW(LPCWSTR a0)
{
    EnterFunc();

    BOOL rv = 0;
    __try {
        rv = Real_DeleteFileW(a0);
    } __finally {
        ExitFunc();
#if 0
        Print("<!-- DeleteFileW(%le -->\n", a0);
#endif
        NoteDelete(a0);
    };
    return rv;
}

static VOID Dump(LPVOID pvData, DWORD cbData)
{
    CHAR szBuffer[128];
    PBYTE pbData = (PBYTE)pvData;

    for (DWORD i = 0; i < cbData; i += 16) {
        PCHAR psz = szBuffer;
        psz = SafePrintf(psz, (LONG)(szBuffer + ARRAYSIZE(szBuffer) - psz), "%4d: ", i);

        for (DWORD j = i; j < i + 16; j++) {
            if (j < cbData) {
                psz = SafePrintf(psz, (LONG)(szBuffer + ARRAYSIZE(szBuffer) - psz),
                                 "%02x", pbData[j]);
            }
            else {
                psz = SafePrintf(psz, (LONG)(szBuffer + ARRAYSIZE(szBuffer) - psz), "  ");
            }
        }

        for (DWORD j = i; j < i + 16; j++) {
            if (j < cbData) {
                if (pbData[j] >= ' ' && pbData[j] <= 127) {
                    psz = SafePrintf(psz, (LONG)(szBuffer + ARRAYSIZE(szBuffer) - psz),
                                     "%c", pbData[j]);
                }
                else {
                    psz = SafePrintf(psz, (LONG)(szBuffer + ARRAYSIZE(szBuffer) - psz), ".");
                }
            }
            else {
                psz = SafePrintf(psz, (LONG)(szBuffer + ARRAYSIZE(szBuffer) - psz), " ");
            }
        }
        Print("%s\n", szBuffer);
    }
}

BOOL WINAPI Mine_DeviceIoControl(HANDLE a0,
                                 DWORD a1,
                                 LPVOID a2,
                                 DWORD a3,
                                 LPVOID a4,
                                 DWORD a5,
                                 LPDWORD a6,
                                 LPOVERLAPPED a7)
{
    EnterFunc();
    DWORD d6 = 0;
    if (a6 == NULL) {
        a6 = &d6;

    }

    BOOL rv = 0;
    __try {
        rv = Real_DeviceIoControl(a0, a1, a2, a3, a4, a5, a6, a7);
    } __finally {
        ExitFunc();
        OpenFiles::SetRead(a0, 0);
        OpenFiles::SetWrite(a0, 0);
        if (rv && a1 != 0x390008 && a1 != 0x4d0008 && a1 != 0x6d0008) {
            FileInfo *pInfo = OpenFiles::RecallFile(a0);

            DWORD DeviceType    = (a1 & 0xffff0000) >> 16;
            DWORD Access        = (a1 & 0x0000c000) >> 14;
            DWORD Function      = (a1 & 0x00003ffc) >> 2;
            DWORD Method        = (a1 & 0x00000003) >> 0;

            if (pInfo) {
                Print("<!-- DeviceIoControl %x [dev=%x,acc=%x,fun=%x,mth=%x] on %ls! -->\n",
                      a1, DeviceType, Access, Function, Method, pInfo->m_pwzPath);
            }
            else {
                Print("<!-- DeviceIoControl %x [dev=%x,acc=%x,fun=%x,mth=%x,in=%d,out=%d/%d] on (%x)! -->\n",
                      a1, DeviceType, Access, Function, Method, a3, *a6, a5, a0);

                if (a3 > 0) {
                    Dump(a2, a3);
                }
                if (a5 > 0) {
                    Dump(a4, (*a6 < a5) ? *a6 : a5);
                }
            }
        }
    };
    return rv;
}

DWORD WINAPI Mine_GetFileAttributesW(LPCWSTR a0)
{
    EnterFunc();

    DWORD rv = 0;
    __try {
        rv = Real_GetFileAttributesW(a0);
    } __finally {
        ExitFunc();
    };
    return rv;
}

BOOL WINAPI Mine_MoveFileWithProgressW(LPCWSTR a0,
                                       LPCWSTR a1,
                                       LPPROGRESS_ROUTINE a2,
                                       LPVOID a3,
                                       DWORD a4)
{
    EnterFunc();

    BOOL rv = 0;
    __try {
        rv = Real_MoveFileWithProgressW(a0, a1, a2, a3, a4);
    } __finally {
        ExitFunc();
        if (rv) {
            NoteRead(a0);
            NoteWrite(a1);
        }
    };
    return rv;
}

BOOL WINAPI Mine_MoveFileA(LPCSTR a0,
                           LPCSTR a1)
{
    EnterFunc();

    BOOL rv = 0;
    __try {
        rv = Real_MoveFileA(a0, a1);
    } __finally {
        ExitFunc();
        if (rv) {
            NoteRead(a0);
            NoteCleanup(a0);
            NoteWrite(a1);
        }
    };
    return rv;
}

BOOL WINAPI Mine_MoveFileW(LPCWSTR a0,
                           LPCWSTR a1)
{
    EnterFunc();

    BOOL rv = 0;
    __try {
        rv = Real_MoveFileW(a0, a1);
    } __finally {
        ExitFunc();
        if (rv) {
            NoteRead(a0);
            NoteCleanup(a0);
            NoteWrite(a1);
        }
    };
    return rv;
}

BOOL WINAPI Mine_MoveFileExA(LPCSTR a0,
                             LPCSTR a1,
                             DWORD a2)
{
    EnterFunc();

    BOOL rv = 0;
    __try {
        rv = Real_MoveFileExA(a0, a1, a2);
    } __finally {
        ExitFunc();
        if (rv) {
            NoteRead(a0);
            NoteCleanup(a0);
            NoteWrite(a1);
        }
    };
    return rv;
}

BOOL WINAPI Mine_MoveFileExW(LPCWSTR a0,
                             LPCWSTR a1,
                             DWORD a2)
{
    EnterFunc();

    BOOL rv = 0;
    __try {
        rv = Real_MoveFileExW(a0, a1, a2);
    } __finally {
        ExitFunc();
        if (rv) {
            NoteRead(a0);
            NoteCleanup(a0);
            NoteWrite(a1);
        }
    };
    return rv;
}

void SetHandle(PCSTR pszName, HANDLE h)
{
#if 0
    FileInfo *pInfo = OpenFiles::RecallFile(h);

    if (pInfo != NULL) {
        Tblog("<!-- hset: %hs (%x) %ls -->\n", pszName, h, pInfo->m_pwzPath);
    }
    else {
        Tblog("<!-- hset: %hs (%x) ***Unknown*** -->\n", pszName, h);
    }
#else
    (void)pszName;
    (void)h;
#endif
}


BOOL WINAPI Mine_SetStdHandle(DWORD a0,
                              HANDLE a1)
{
    EnterFunc();

    BOOL rv = 0;
    __try {
        rv = Real_SetStdHandle(a0, a1);
        if (rv && a1 != 0) {
            switch (a0) {
              case STD_INPUT_HANDLE:
                SetHandle("stdin", a1);
                break;
              case STD_OUTPUT_HANDLE:
                SetHandle("stdout", a1);
                break;
              case STD_ERROR_HANDLE:
                SetHandle("stderr", a1);
                break;
            }
        }
    } __finally {
        ExitFunc();
    };
    return rv;
}

HMODULE WINAPI Mine_LoadLibraryA(LPCSTR a0)
{
    EnterFunc();

    HMODULE rv = 0;
    __try {
        rv = Real_LoadLibraryA(a0);
    } __finally {
        ExitFunc();
    };
    return rv;
}

HMODULE WINAPI Mine_LoadLibraryW(LPCWSTR a0)
{
    EnterFunc();

    HMODULE rv = 0;
    __try {
        rv = Real_LoadLibraryW(a0);
    } __finally {
        ExitFunc();
    };
    return rv;
}

HMODULE WINAPI Mine_LoadLibraryExA(LPCSTR a0,
                                   HANDLE a1,
                                   DWORD a2)
{
    EnterFunc();

    HMODULE rv = 0;
    __try {
        rv = Real_LoadLibraryExA(a0, a1, a2);
    } __finally {
        ExitFunc();
    };
    return rv;
}

HMODULE WINAPI Mine_LoadLibraryExW(LPCWSTR a0,
                                   HANDLE a1,
                                   DWORD a2)
{
    EnterFunc();

    HMODULE rv = 0;
    __try {
        rv = Real_LoadLibraryExW(a0, a1, a2);
    } __finally {
        ExitFunc();
    };
    return rv;
}

DWORD WINAPI Mine_SetFilePointer(HANDLE hFile,
                                 LONG lDistanceToMove,
                                 PLONG lpDistanceToMoveHigh,
                                 DWORD dwMoveMethod)
{
    EnterFunc();

    DWORD rv = 0;
    __try {
        rv = Real_SetFilePointer(hFile,
                                 lDistanceToMove,
                                 lpDistanceToMoveHigh,
                                 dwMoveMethod);
    } __finally {
        LONG high = 0;
        if (lpDistanceToMoveHigh == NULL) {
            lpDistanceToMoveHigh = &high;
        }

        FileInfo * pInfo = OpenFiles::RecallFile(hFile);
        if (pInfo != NULL) {
            if (dwMoveMethod == FILE_END && lDistanceToMove == 0xffffffff) {
#if 0
                Print("<!-- SetFilePointer(APPEND, %le) -->\n",
                      pInfo->m_pwzPath);
#endif
                pInfo->m_fAppend = TRUE;
            }
#if 0
            else if (dwMoveMethod == FILE_END) {
                Print("<!-- SetFilePointer(END:%08x:%08x, %le) -->\n",
                      (int)lDistanceToMove,
                      *lpDistanceToMoveHigh,
                      pInfo->m_pwzPath);
            }
            else if (dwMoveMethod == FILE_BEGIN) {
                Print("<!-- SetFilePointer(BEG:%08x:%08x, %le) -->\n",
                      (int)lDistanceToMove,
                      *lpDistanceToMoveHigh,
                      pInfo->m_pwzPath);
            }
            else if (dwMoveMethod == FILE_CURRENT) {
                Print("<!-- SetFilePointer(CUR:%08x:%08x, %le) -->\n",
                      (int)lDistanceToMove,
                      *lpDistanceToMoveHigh,
                      pInfo->m_pwzPath);
            }
#endif
        }
        ExitFunc();
    };
    return rv;
}

BOOL WINAPI Mine_SetFilePointerEx(HANDLE hFile,
                                  LARGE_INTEGER liDistanceToMove,
                                  PLARGE_INTEGER lpNewFilePointer,
                                  DWORD dwMoveMethod)
{
    EnterFunc();

    BOOL rv = 0;
    __try {
        rv = Real_SetFilePointerEx(hFile,
                                   liDistanceToMove,
                                   lpNewFilePointer,
                                   dwMoveMethod);
    } __finally {
#if 0
        FileInfo * pInfo = OpenFiles::RecallFile(hFile);
        if (pInfo != NULL) {
            if (dwMoveMethod == FILE_END) {
                Print("<!-- SetFilePointerEx(END:%I64d, %le) -->\n",
                      liDistanceToMove.QuadPart,
                      pInfo->m_pwzPath);
            }
            else if (dwMoveMethod == FILE_BEGIN) {
                Print("<!-- SetFilePointerEx(BEG:%I64d, %le) -->\n",
                      liDistanceToMove.QuadPart,
                      pInfo->m_pwzPath);
            }
            else if (dwMoveMethod == FILE_CURRENT) {
                Print("<!-- SetFilePointerEx(CUR:%I64d, %le) -->\n",
                      liDistanceToMove.QuadPart,
                      pInfo->m_pwzPath);
            }
        }
#endif
        ExitFunc();
    };
    return rv;
}

BOOL WINAPI Mine_ReadFile(HANDLE a0,
                          LPVOID a1,
                          DWORD a2,
                          LPDWORD a3,
                          LPOVERLAPPED a4)
{
    EnterFunc();

    BOOL rv = 0;
    __try {
        rv = Real_ReadFile(a0, a1, a2, a3, a4);
    } __finally {
        if (rv) {
            OpenFiles::SetRead(a0, a2);
        }
        ExitFunc();
    };
    return rv;
}

BOOL WINAPI Mine_ReadFileEx(HANDLE a0,
                            LPVOID a1,
                            DWORD a2,
                            LPOVERLAPPED a3,
                            LPOVERLAPPED_COMPLETION_ROUTINE a4)
{
    EnterFunc();

    BOOL rv = 0;
    __try {
        rv = Real_ReadFileEx(a0, a1, a2, a3, a4);
    } __finally {
        if (rv) {
            OpenFiles::SetRead(a0, a2);
        }
        ExitFunc();
    };
    return rv;
}

BOOL WINAPI Mine_WriteFile(HANDLE a0,
                           LPCVOID a1,
                           DWORD a2,
                           LPDWORD a3,
                           LPOVERLAPPED a4)
{
    EnterFunc();

    BOOL rv = 0;
    __try {
        rv = Real_WriteFile(a0, a1, a2, a3, a4);
    } __finally {
        OpenFiles::SetWrite(a0, a2);
        ExitFunc();
    };
    return rv;
}

BOOL WINAPI Mine_WriteFileEx(HANDLE a0,
                             LPCVOID a1,
                             DWORD a2,
                             LPOVERLAPPED a3,
                             LPOVERLAPPED_COMPLETION_ROUTINE a4)
{
    EnterFunc();

    BOOL rv = 0;
    __try {
        rv = Real_WriteFileEx(a0, a1, a2, a3, a4);
    } __finally {
        OpenFiles::SetWrite(a0, a2);
        ExitFunc();
    };
    return rv;
}

BOOL WINAPI Mine_WriteConsoleA(HANDLE a0,
                                  const VOID* a1,
                                  DWORD a2,
                                  LPDWORD a3,
                                  LPVOID a4)
{
    EnterFunc();

    BOOL rv = 0;
    __try {
        rv = Real_WriteConsoleA(a0, a1, a2, a3, a4);
    } __finally {
        OpenFiles::SetWrite(a0, a2);
        ExitFunc();
    };
    return rv;
}

BOOL WINAPI Mine_WriteConsoleW(HANDLE a0,
                                  const VOID* a1,
                                  DWORD a2,
                                  LPDWORD a3,
                                  LPVOID a4)
{
    EnterFunc();

    BOOL rv = 0;
    __try {
        rv = Real_WriteConsoleW(a0, a1, a2, a3, a4);
    } __finally {
        OpenFiles::SetWrite(a0, a2);
        ExitFunc();
    };
    return rv;
}

//////////////////////////////////////////////////////////////////////////////
//
DWORD WINAPI Mine_ExpandEnvironmentStringsA(PCSTR lpSrc, PCHAR lpDst, DWORD nSize)
{
    EnterFunc();
    DWORD rv = 0;
    __try {
        rv = Real_ExpandEnvironmentStringsA(lpSrc, lpDst, nSize);
    }
    __finally {
        if (rv > 0) {
#if 0
            Print("<!-- ExpandEnvironmentStringsA(%he) -->\n", lpSrc);
#endif
        }
        ExitFunc();
    };
    return rv;
}

DWORD WINAPI Mine_ExpandEnvironmentStringsW(PCWSTR lpSrc, PWCHAR lpDst, DWORD nSize)
{
    EnterFunc();
    DWORD rv = 0;
    __try {
        rv = Real_ExpandEnvironmentStringsW(lpSrc, lpDst, nSize);
    }
    __finally {
        if (rv > 0) {
#if 0
            Print("<!-- ExpandEnvironmentStringsW(%le) -->\n", lpSrc);
#endif
        }
        ExitFunc();
    };
    return rv;
}

DWORD WINAPI Mine_GetEnvironmentVariableA(PCSTR lpName, PCHAR lpBuffer, DWORD nSize)
{
    EnterFunc();
    DWORD rv = 0;
    __try {
        rv = Real_GetEnvironmentVariableA(lpName, lpBuffer, nSize);
        //        if (rv > 0 && rv < nSize && lpBuffer != NULL) {
        //            EnvVars::Used(lpName);
        //        }
    }
    __finally {
        EnvVars::Used(lpName);
        ExitFunc();
    };
    return rv;
}

DWORD WINAPI Mine_GetEnvironmentVariableW(PCWSTR lpName, PWCHAR lpBuffer, DWORD nSize)
{
    EnterFunc();
    DWORD rv = 0;
    __try {
        rv = Real_GetEnvironmentVariableW(lpName, lpBuffer, nSize);
        //        if (rv > 0 && rv < nSize && lpBuffer != NULL) {
        //            EnvVars::Used(lpName);
        //        }
    }
    __finally {
        EnvVars::Used(lpName);
        ExitFunc();
    };
    return rv;
}

PCWSTR CDECL Mine_wgetenv(PCWSTR var)
{
    EnterFunc();
    PCWSTR rv = 0;
    __try {
        rv = Real_wgetenv(var);
        //        if (rv != NULL) {
        //            EnvVars::Used(var);
        //        }
    }
    __finally {
        EnvVars::Used(var);
        ExitFunc();
    }
    return rv;
}

PCSTR CDECL Mine_getenv(PCSTR var)
{
    EnterFunc();
    PCSTR rv = 0;
    __try {
        rv = Real_getenv(var);
        //        if (rv) {
        //            EnvVars::Used(var);
        //        }
    }
    __finally {
        EnvVars::Used(var);
        ExitFunc();
    }
    return rv;
}

DWORD CDECL Mine_getenv_s(DWORD *pValue, PCHAR pBuffer, DWORD cBuffer, PCSTR varname)
{
    EnterFunc();
    DWORD rv = 0;
    __try {
        DWORD value;
        if (pValue == NULL) {
            pValue = &value;
        }
        rv = Real_getenv_s(pValue, pBuffer, cBuffer, varname);
        //        if (rv == 0 && *pValue > 0) {
        //            EnvVars::Used(varname);
        //        }
    }
    __finally {
        EnvVars::Used(varname);
        ExitFunc();
    }
    return rv;
}

DWORD CDECL Mine_wgetenv_s(DWORD *pValue, PWCHAR pBuffer, DWORD cBuffer, PCWSTR varname)
{
    EnterFunc();
    DWORD rv = 0;
    __try {
        DWORD value;
        if (pValue == NULL) {
            pValue = &value;
        }
        rv = Real_wgetenv_s(pValue, pBuffer, cBuffer, varname);
        //        if (rv == 0 && *pValue > 0) {
        //            EnvVars::Used(varname);
        //        }
    }
    __finally {
        EnvVars::Used(varname);
        ExitFunc();
    }
    return rv;
}

DWORD CDECL Mine_dupenv_s(PCHAR *ppBuffer, DWORD *pcBuffer, PCSTR varname)
{
    EnterFunc();
    DWORD rv = 0;
    __try {
        PCHAR pb;
        DWORD cb;
        if (ppBuffer == NULL) {
            ppBuffer = &pb;
        }
        if (pcBuffer == NULL) {
            pcBuffer = &cb;
        }
        rv = Real_dupenv_s(ppBuffer, pcBuffer, varname);
        //        if (rv == 0 && *pcBuffer > 0 && *ppBuffer != NULL) {
        //            EnvVars::Used(varname);
        //        }
    }
    __finally {
        EnvVars::Used(varname);
        ExitFunc();
    }
    return rv;
}

DWORD CDECL Mine_wdupenv_s(PWCHAR *ppBuffer, DWORD *pcBuffer, PCWSTR varname)
{
    EnterFunc();
    DWORD rv = 0;
    __try {
        PWCHAR pb;
        DWORD cb;
        if (ppBuffer == NULL) {
            ppBuffer = &pb;
        }
        if (pcBuffer == NULL) {
            pcBuffer = &cb;
        }
        rv = Real_wdupenv_s(ppBuffer, pcBuffer, varname);
        //        if (rv == 0 && *pcBuffer > 0 && *ppBuffer != NULL) {
        //            EnvVars::Used(varname);
        //        }
    }
    __finally {
        EnvVars::Used(varname);
        ExitFunc();
    }
    return rv;
}


/////////////////////////////////////////////////////////////
// AttachDetours
//
LONG AttachDetours(VOID)
{
    DetourTransactionBegin();
    DetourUpdateThread(GetCurrentThread());

    DetourAttach(&(PVOID&)Real_EntryPoint, Mine_EntryPoint);
    DetourAttach(&(PVOID&)Real_ExitProcess, Mine_ExitProcess);
    DetourAttach(&(PVOID&)Real_CopyFileExA, Mine_CopyFileExA);
    DetourAttach(&(PVOID&)Real_CopyFileExW, Mine_CopyFileExW);
    DetourAttach(&(PVOID&)Real_PrivCopyFileExW, Mine_PrivCopyFileExW);
    DetourAttach(&(PVOID&)Real_CreateHardLinkA, Mine_CreateHardLinkA);
    DetourAttach(&(PVOID&)Real_CreateHardLinkW, Mine_CreateHardLinkW);
    DetourAttach(&(PVOID&)Real_CreateDirectoryW, Mine_CreateDirectoryW);
    DetourAttach(&(PVOID&)Real_CreateDirectoryExW, Mine_CreateDirectoryExW);
    DetourAttach(&(PVOID&)Real_CreateFileW, Mine_CreateFileW);
    DetourAttach(&(PVOID&)Real_CreatePipe, Mine_CreatePipe);
    DetourAttach(&(PVOID&)Real_CreateFileMappingW, Mine_CreateFileMappingW);
    DetourAttach(&(PVOID&)Real_CloseHandle, Mine_CloseHandle);
    DetourAttach(&(PVOID&)Real_DuplicateHandle, Mine_DuplicateHandle);
    DetourAttach(&(PVOID&)Real_CreateProcessW, Mine_CreateProcessW);
    DetourAttach(&(PVOID&)Real_CreateProcessA, Mine_CreateProcessA);
    DetourAttach(&(PVOID&)Real_DeleteFileW, Mine_DeleteFileW);
    DetourAttach(&(PVOID&)Real_DeviceIoControl, Mine_DeviceIoControl);
    DetourAttach(&(PVOID&)Real_GetFileAttributesW, Mine_GetFileAttributesW);
    DetourAttach(&(PVOID&)Real_MoveFileA, Mine_MoveFileA);
    DetourAttach(&(PVOID&)Real_MoveFileW, Mine_MoveFileW);
    DetourAttach(&(PVOID&)Real_MoveFileExA, Mine_MoveFileExA);
    DetourAttach(&(PVOID&)Real_MoveFileExW, Mine_MoveFileExW);
    DetourAttach(&(PVOID&)Real_MoveFileWithProgressW, Mine_MoveFileWithProgressW);
    DetourAttach(&(PVOID&)Real_SetStdHandle, Mine_SetStdHandle);
    DetourAttach(&(PVOID&)Real_LoadLibraryA, Mine_LoadLibraryA);
    DetourAttach(&(PVOID&)Real_LoadLibraryW, Mine_LoadLibraryW);
    DetourAttach(&(PVOID&)Real_LoadLibraryExA, Mine_LoadLibraryExA);
    DetourAttach(&(PVOID&)Real_LoadLibraryExW, Mine_LoadLibraryExW);
    DetourAttach(&(PVOID&)Real_SetFilePointer, Mine_SetFilePointer);
    DetourAttach(&(PVOID&)Real_SetFilePointerEx, Mine_SetFilePointerEx);
    DetourAttach(&(PVOID&)Real_ReadFile, Mine_ReadFile);
    DetourAttach(&(PVOID&)Real_ReadFileEx, Mine_ReadFileEx);
    DetourAttach(&(PVOID&)Real_WriteFile, Mine_WriteFile);
    DetourAttach(&(PVOID&)Real_WriteFileEx, Mine_WriteFileEx);
    DetourAttach(&(PVOID&)Real_WriteConsoleA, Mine_WriteConsoleA);
    DetourAttach(&(PVOID&)Real_WriteConsoleW, Mine_WriteConsoleW);
    DetourAttach(&(PVOID&)Real_ExpandEnvironmentStringsA, Mine_ExpandEnvironmentStringsA);
    DetourAttach(&(PVOID&)Real_ExpandEnvironmentStringsW, Mine_ExpandEnvironmentStringsW);
    DetourAttach(&(PVOID&)Real_GetEnvironmentVariableA, Mine_GetEnvironmentVariableA);
    DetourAttach(&(PVOID&)Real_GetEnvironmentVariableW, Mine_GetEnvironmentVariableW);

    return DetourTransactionCommit();
}

LONG DetachDetours(VOID)
{
    DetourTransactionBegin();
    DetourUpdateThread(GetCurrentThread());

    DetourDetach(&(PVOID&)Real_EntryPoint, Mine_EntryPoint);
    DetourAttach(&(PVOID&)Real_ExitProcess, Mine_ExitProcess);
    DetourDetach(&(PVOID&)Real_CopyFileExA, Mine_CopyFileExA);
    DetourDetach(&(PVOID&)Real_CopyFileExW, Mine_CopyFileExW);
    DetourDetach(&(PVOID&)Real_PrivCopyFileExW, Mine_PrivCopyFileExW);
    DetourDetach(&(PVOID&)Real_CreateHardLinkA, Mine_CreateHardLinkA);
    DetourDetach(&(PVOID&)Real_CreateHardLinkW, Mine_CreateHardLinkW);
    DetourDetach(&(PVOID&)Real_CreateDirectoryW, Mine_CreateDirectoryW);
    DetourDetach(&(PVOID&)Real_CreateDirectoryExW, Mine_CreateDirectoryExW);
    DetourDetach(&(PVOID&)Real_CreateFileW, Mine_CreateFileW);
    DetourDetach(&(PVOID&)Real_CreatePipe, Mine_CreatePipe);
    DetourDetach(&(PVOID&)Real_CreateFileMappingW, Mine_CreateFileMappingW);
    DetourDetach(&(PVOID&)Real_CloseHandle, Mine_CloseHandle);
    DetourDetach(&(PVOID&)Real_DuplicateHandle, Mine_DuplicateHandle);
    DetourDetach(&(PVOID&)Real_CreateProcessW, Mine_CreateProcessW);
    DetourDetach(&(PVOID&)Real_CreateProcessA, Mine_CreateProcessA);
    DetourDetach(&(PVOID&)Real_DeleteFileW, Mine_DeleteFileW);
    DetourDetach(&(PVOID&)Real_DeviceIoControl, Mine_DeviceIoControl);
    DetourDetach(&(PVOID&)Real_GetFileAttributesW, Mine_GetFileAttributesW);
    DetourDetach(&(PVOID&)Real_MoveFileA, Mine_MoveFileA);
    DetourDetach(&(PVOID&)Real_MoveFileW, Mine_MoveFileW);
    DetourDetach(&(PVOID&)Real_MoveFileExA, Mine_MoveFileExA);
    DetourDetach(&(PVOID&)Real_MoveFileExW, Mine_MoveFileExW);
    DetourDetach(&(PVOID&)Real_MoveFileWithProgressW, Mine_MoveFileWithProgressW);
    DetourDetach(&(PVOID&)Real_SetStdHandle, Mine_SetStdHandle);
    DetourDetach(&(PVOID&)Real_LoadLibraryA, Mine_LoadLibraryA);
    DetourDetach(&(PVOID&)Real_LoadLibraryW, Mine_LoadLibraryW);
    DetourDetach(&(PVOID&)Real_LoadLibraryExA, Mine_LoadLibraryExA);
    DetourDetach(&(PVOID&)Real_LoadLibraryExW, Mine_LoadLibraryExW);
    DetourDetach(&(PVOID&)Real_SetFilePointer, Mine_SetFilePointer);
    DetourDetach(&(PVOID&)Real_SetFilePointerEx, Mine_SetFilePointerEx);
    DetourDetach(&(PVOID&)Real_ReadFile, Mine_ReadFile);
    DetourDetach(&(PVOID&)Real_ReadFileEx, Mine_ReadFileEx);
    DetourDetach(&(PVOID&)Real_WriteFile, Mine_WriteFile);
    DetourDetach(&(PVOID&)Real_WriteFileEx, Mine_WriteFileEx);
    DetourDetach(&(PVOID&)Real_WriteConsoleA, Mine_WriteConsoleA);
    DetourDetach(&(PVOID&)Real_WriteConsoleW, Mine_WriteConsoleW);
    DetourDetach(&(PVOID&)Real_ExpandEnvironmentStringsA, Mine_ExpandEnvironmentStringsA);
    DetourDetach(&(PVOID&)Real_ExpandEnvironmentStringsW, Mine_ExpandEnvironmentStringsW);
    DetourDetach(&(PVOID&)Real_GetEnvironmentVariableA, Mine_GetEnvironmentVariableA);
    DetourDetach(&(PVOID&)Real_GetEnvironmentVariableW, Mine_GetEnvironmentVariableW);

    if (Real_getenv) { DetourDetach(&(PVOID&)Real_getenv, Mine_getenv); }
    if (Real_getenv_s) { DetourDetach(&(PVOID&)Real_getenv_s, Mine_getenv_s); }
    if (Real_wgetenv) { DetourDetach(&(PVOID&)Real_wgetenv, Mine_wgetenv); }
    if (Real_wgetenv_s) { DetourDetach(&(PVOID&)Real_wgetenv, Mine_wgetenv_s); }
    if (Real_dupenv_s) { DetourDetach(&(PVOID&)Real_dupenv_s, Mine_dupenv_s); }
    if (Real_wdupenv_s) { DetourDetach(&(PVOID&)Real_wdupenv_s, Mine_wdupenv_s); }

    return DetourTransactionCommit();
}
//
//////////////////////////////////////////////////////////////////////////////

VOID NoteRead(PCSTR psz)
{
    FileInfo *pInfo = FileNames::FindPartial(psz);
    pInfo->m_fRead = TRUE;
}

VOID NoteRead(PCWSTR pwz)
{
    FileInfo *pInfo = FileNames::FindPartial(pwz);
    pInfo->m_fRead = TRUE;
}

VOID NoteWrite(PCSTR psz)
{
    FileInfo *pInfo = FileNames::FindPartial(psz);
    pInfo->m_fWrite = TRUE;
    if (!pInfo->m_fRead) {
        pInfo->m_fCantRead = TRUE;
    }
}

VOID NoteWrite(PCWSTR pwz)
{
    FileInfo *pInfo = FileNames::FindPartial(pwz);
    pInfo->m_fWrite = TRUE;
    if (!pInfo->m_fRead) {
        pInfo->m_fCantRead = TRUE;
    }
}

VOID NoteDelete(PCSTR psz)
{
    FileInfo *pInfo = FileNames::FindPartial(psz);
    if (pInfo->m_fWrite || pInfo->m_fRead) {
        pInfo->m_fCleanup = TRUE;
    }
    else {
        pInfo->m_fDelete = TRUE;
    }
    if (!pInfo->m_fRead) {
        pInfo->m_fCantRead = TRUE;
    }
}

VOID NoteDelete(PCWSTR pwz)
{
    FileInfo *pInfo = FileNames::FindPartial(pwz);
    if (pInfo->m_fWrite || pInfo->m_fRead) {
        pInfo->m_fCleanup = TRUE;
    }
    else {
        pInfo->m_fDelete = TRUE;
    }
    if (!pInfo->m_fRead) {
        pInfo->m_fCantRead = TRUE;
    }
}

VOID NoteCleanup(PCSTR psz)
{
    FileInfo *pInfo = FileNames::FindPartial(psz);
    pInfo->m_fCleanup = TRUE;
}

VOID NoteCleanup(PCWSTR pwz)
{
    FileInfo *pInfo = FileNames::FindPartial(pwz);
    pInfo->m_fCleanup = TRUE;
}

////////////////////////////////////////////////////////////// Logging System.
//
static BOOL s_bLog = 1;
static LONG s_nTlsIndent = -1;
static LONG s_nTlsThread = -1;
static LONG s_nThreadCnt = 0;

LONG EnterFunc()
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

    SetLastError(dwErr);

    return nIndent;
}

VOID ExitFunc()
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

    SetLastError(dwErr);
}

VOID Print(const CHAR *psz, ...)
{
    DWORD dwErr = GetLastError();

    if (s_bLog && psz) {
        va_list  args;
        va_start(args, psz);

        TblogV(psz, args);

        va_end(args);
    }

    SetLastError(dwErr);
}

VOID AssertFailed(CONST PCHAR pszMsg, CONST PCHAR pszFile, ULONG nLine)
{
    Tblog("ASSERT(%hs) failed in %s, line %d.\n", pszMsg, pszFile, nLine);
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
    InitializeCriticalSection(&s_csPipe);
    InitializeCriticalSection(&s_csChildPayload);

    Procs::Initialize();
    EnvVars::Initialize();
    FileNames::Initialize();
    OpenFiles::Initialize();

    s_bLog = FALSE;
    s_nTlsIndent = TlsAlloc();
    s_nTlsThread = TlsAlloc();

    s_hInst = hDll;
    s_hKernel32 = NULL;

    PBYTE xCreate = (PBYTE)DetourCodeFromPointer((PVOID)Real_CreateProcessW, NULL);
    PTBLOG_PAYLOAD pPayload = NULL;

    for (HMODULE hMod = NULL; (hMod = DetourEnumerateModules(hMod)) != NULL;) {
        ULONG cbData;
        PVOID pvData = DetourFindPayload(hMod, s_guidTrace, &cbData);

        if (pvData != NULL && pPayload == NULL) {
            pPayload = (PTBLOG_PAYLOAD)pvData;
        }

        ULONG cbMod = DetourGetModuleSize(hMod);

        if (((PBYTE)hMod) < xCreate && ((PBYTE)hMod + cbMod) > xCreate) {
            s_hKernel32 = hMod;
        }
    }

    ZeroMemory(&s_Payload, sizeof(s_Payload));

    if (pPayload == NULL) {
        return FALSE;
    }

    CopyMemory(&s_Payload, pPayload, sizeof(s_Payload));

    LoadStdHandleName(STD_INPUT_HANDLE, s_Payload.wzStdin, FALSE);
    LoadStdHandleName(STD_OUTPUT_HANDLE, s_Payload.wzStdout, s_Payload.fStdoutAppend);
    LoadStdHandleName(STD_ERROR_HANDLE, s_Payload.wzStderr, s_Payload.fStderrAppend);
    s_nTraceProcessId = s_Payload.nTraceProcessId;

    GetModuleFileNameA(s_hInst, s_szDllPath, ARRAYSIZE(s_szDllPath));

    // Find hidden functions.
    Real_PrivCopyFileExW =
        (BOOL (WINAPI *)(LPCWSTR, LPCWSTR, LPPROGRESS_ROUTINE, LPVOID, LPBOOL, DWORD))
        GetProcAddress(s_hKernel32, "PrivCopyFileExW");
    if (Real_PrivCopyFileExW == NULL) {
        DEBUG_BREAK();
    }

    LONG error = AttachDetours();
    if (error != NO_ERROR) {
        DEBUG_BREAK();
        Tblog("<!-- Error attaching detours: %d -->\n", error);
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
        Tblog("<!-- Error detaching detours: %d -->\n", error);
    }

    TblogClose();

    if (s_nTlsIndent >= 0) {
        TlsFree(s_nTlsIndent);
    }
    if (s_nTlsThread >= 0) {
        TlsFree(s_nTlsThread);
    }
    return TRUE;
}

inline VOID UpdateIfRoom(PWCHAR& pwzDst, PWCHAR pwzDstEnd, WCHAR c)
{
    if (pwzDst < pwzDstEnd) {
        *pwzDst++ = c;  // Write character if room in buffer.
    }
    else {
        pwzDst++;       // If no room, just advance pointer (to alloc calculation)
    }
}

static PCHAR RemoveReturns(PCHAR pszBuffer)
{
    PCHAR pszIn = pszBuffer;
    PCHAR pszOut = pszBuffer;

    while (*pszIn) {
        if (*pszIn == '\r') {
            pszIn++;
            continue;
        }
        *pszOut++ = *pszIn++;
    }
    *pszOut = '\0';

    return pszBuffer;
}

static PWCHAR RemoveReturns(PWCHAR pwzBuffer)
{
    PWCHAR pwzIn = pwzBuffer;
    PWCHAR pwzOut = pwzBuffer;

    while (*pwzIn) {
        if (*pwzIn == '\r') {
            pwzIn++;
            continue;
        }
        *pwzOut++ = *pwzIn++;
    }
    *pwzOut = '\0';

    return pwzBuffer;
}

PBYTE LoadFile(HANDLE hFile, DWORD cbFile)
{
    PBYTE pbFile = (PBYTE)GlobalAlloc(GPTR, cbFile + 3);
    if (pbFile == NULL) {
        return NULL;
    }

    DWORD cbRead = 0;
    Real_SetFilePointer(hFile, 0, NULL, FILE_BEGIN);
    Real_ReadFile(hFile, pbFile, cbFile, &cbRead, NULL);

    // Make sure the file is zero terminated.
    pbFile[cbRead + 0] = 0;
    pbFile[cbRead + 1] = 0;
    pbFile[cbRead + 2] = 0;

    return pbFile;
}

PWCHAR More(PCWSTR pwzPath, PWCHAR pwzDst, PWCHAR pwzDstEnd)
{
    HANDLE hFile = Real_CreateFileW(pwzPath, GENERIC_READ, 0, NULL, OPEN_EXISTING, 0, NULL);
    if (hFile == INVALID_HANDLE_VALUE) {
        return NULL;
    }

    FileInfo *pInfo = FileNames::FindPartial(pwzPath);
    pInfo->m_fAbsorbed = true;

    DWORD cbFile = Real_SetFilePointer(hFile, 0, NULL, FILE_END);
    DWORD cbRead = 0;

    PCHAR pszFile = (PCHAR)GlobalAlloc(GPTR, cbFile + 2);   // 2 bytes null for Unicode or Ascii.
    if (pszFile != NULL) {
        Real_SetFilePointer(hFile, 0, NULL, FILE_BEGIN);
        Real_ReadFile(hFile, pszFile, cbFile, &cbRead, NULL);

        if (((PUCHAR)pszFile)[0] == 0xff && ((PUCHAR)pszFile)[1] == 0xfe) {
            // Unicode
            PWCHAR pwzFile = ((PWCHAR)pszFile) + 1;
            PCWSTR pwzIn = pwzFile;
            while (*pwzIn) {
                if (*pwzIn == ' ' || *pwzIn == '\t' || *pwzIn == '\r' || *pwzIn == '\n') {
                    UpdateIfRoom(pwzDst, pwzDstEnd, ' ');
                    while (*pwzIn == ' ' || *pwzIn == '\t' || *pwzIn == '\r' || *pwzIn == '\n') {
                        pwzIn++;
                    }
                }
                else {
                    UpdateIfRoom(pwzDst, pwzDstEnd, *pwzIn++);
                }
            }
        }
        else {
            PCSTR pszIn = pszFile;
            while (*pszIn) {
                if (*pszIn == ' ' || *pszIn == '\t' || *pszIn == '\r' || *pszIn == '\n') {
                    UpdateIfRoom(pwzDst, pwzDstEnd, ' ');
                    while (*pszIn == ' ' || *pszIn == '\t' || *pszIn == '\r' || *pszIn == '\n') {
                        pszIn++;
                    }
                }
                else {
                    UpdateIfRoom(pwzDst, pwzDstEnd, *pszIn++);
                }
            }
        }

        GlobalFree(pszFile);
    }

    Real_CloseHandle(hFile);

    return pwzDst;
}

// This function is called twice.  On the first call, pwzDstEnd <= pwzDst and
// no data is copied, but pwzDst is advanced so we can see how big of a
// buffer is needed to hold the command line.
//
// On the second call, the command line is actually populated.
PWCHAR LoadCommandLine(PCWSTR pwz, PWCHAR pwzDst, PWCHAR pwzDstEnd)
{
    while (*pwz) {
        PCWSTR pwzArgBeg = NULL;
        PCWSTR pwzArgEnd = NULL;
        WCHAR cQuote = '\0';
        BOOL fMore = false;

        if (*pwz == '@') {
            fMore = true;
            pwz++;
        }

        if (*pwz == '\"' || *pwz == '\'') {
            cQuote = *pwz++;

            pwzArgBeg = pwz;
            while (*pwz != '\0' && *pwz != cQuote) {
                pwz++;
            }
            pwzArgEnd = pwz;

            if (*pwz == cQuote) {
                pwz++;
            }
        }
        else {
            pwzArgBeg = pwz;
            while (*pwz != '\0' && *pwz != ' ' && *pwz != '\t' && *pwz != '\n' && *pwz != '\r') {
                pwz++;
            }
            pwzArgEnd = pwz;
        }

        if (fMore) {
            // More arguments!
            WCHAR wzPath[MAX_PATH];
            PWCHAR pwzPath = wzPath;
            PCWSTR pwzTmp = pwzArgBeg + 1;
            while (pwzTmp < pwzArgEnd && pwzPath < wzPath + ARRAYSIZE(wzPath)-2) {
                *pwzPath++ = *pwzTmp++;
            }
            *pwzPath = '\0';

            PWCHAR pwzOut = More(wzPath, pwzDst, pwzDstEnd);
            if (pwzOut != NULL) {
                pwzDst = pwzOut;

                cQuote = 0;
                pwzArgBeg = pwzArgEnd;
            }
        }

        if (cQuote) {
            UpdateIfRoom(pwzDst, pwzDstEnd, cQuote);
        }
        for (; pwzArgBeg < pwzArgEnd; pwzArgBeg++) {
            UpdateIfRoom(pwzDst, pwzDstEnd, *pwzArgBeg);
        }
        if (cQuote) {
            UpdateIfRoom(pwzDst, pwzDstEnd, cQuote);
        }

        if (*pwz) {
            UpdateIfRoom(pwzDst, pwzDstEnd, ' ');
        }

        // skip over separating spaces.
        while (*pwz == ' ' || *pwz == '\t' || *pwz == '\n' || *pwz == '\r') {
            pwz++;
        }
    }
    return pwzDst;
}

void TestHandle(PCSTR pszName, HANDLE h)
{
    FileInfo *pInfo = OpenFiles::RecallFile(h);

    if (pInfo != NULL) {
#if 1 // Ignore PIPEs.
        if (FileNames::PrefixMatch(pInfo->m_pwzPath, L"\\\\.\\PIPE\\")) {
            // Ignore;
        }
        else
#endif
            if (FileNames::SuffixMatch(pInfo->m_pwzPath, L"\\conout$")) {
            // Ignore;
        }
        else if (FileNames::SuffixMatch(pInfo->m_pwzPath, L"\\conin$")) {
            // Ignore;
        }
        else if (FileNames::SuffixMatch(pInfo->m_pwzPath, L"\\nul")) {
            // Ignore;
        }
        else {
            Tblog("<%hs%hs>%le</%hs>\n",
                  pszName, pInfo->m_fAppend ? " append=\"true\"" : "", pInfo->m_pwzPath, pszName);
        }
    }
    else {
        Tblog("<!-- hand: %hs (%x) ***Unknown*** -->\n", pszName, h);
    }
}

LONG WINAPI DetourAttachIf(PVOID *ppPointer, PVOID pDetour)
{
    if (*ppPointer == NULL) {
        Tblog("<!-- DetourAttachIf failed: %p -->\n", pDetour);
        return NO_ERROR;
    }

    PDETOUR_TRAMPOLINE pRealTrampoline;
    PVOID pRealTarget;
    PVOID pRealDetour;

    LONG err = DetourAttachEx(ppPointer, pDetour, &pRealTrampoline, &pRealTarget, &pRealDetour);
    if (err == NO_ERROR) {
        // Tblog("<!-- DetourAttachIf %p at %p -->\n", pDetour, pRealTarget);
        return NO_ERROR;
    }
    return err;
}

int WINAPI Mine_EntryPoint(VOID)
{
    // This function is invoked instead of the process EntryPoint (Real_EntryPoint).

    TblogOpen();

    SaveEnvironment();

    {
        CHAR szExeName[MAX_PATH];
        CHAR szId[128];
        CHAR szParent[128];
        WCHAR wzPath[MAX_PATH];
        PCHAR pszExeName = szExeName;

        // Get the base command line (skipping over the executable name)
        PCWSTR pwzLine = GetCommandLineW();
        if (*pwzLine == '\"') {
            pwzLine++;
            while (*pwzLine && *pwzLine != '\"') {
                pwzLine++;
            }
            if (*pwzLine == '\"') {
                pwzLine++;
            }
        }
        else {
            while (*pwzLine && *pwzLine != ' ' && *pwzLine != '\t') {
                pwzLine++;
            }
        }
        while (*pwzLine && (*pwzLine == ' ' || *pwzLine == '\t')) {
            pwzLine++;
        }

        // Get the root executable name.
        if (GetModuleFileNameA(0, szExeName, ARRAYSIZE(szExeName))) {
            PCHAR psz = szExeName;

            while (*psz) {
                psz++;
            }

            while (psz > szExeName && psz[-1] != ':' && psz[-1] != '\\' && psz[-1] != '/') {
                psz--;
            }
            pszExeName = psz;
            while (*psz && *psz != '.') {
                psz++;
            }
            *psz = '\0';
        }
        else {
            szExeName[0] = '\0';
        }

        // Start the XML process node.
        Tblog("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        {
            PCHAR pszId = szId;
            PCHAR pszParent = szParent;
            for (DWORD i = 0; i < s_Payload.nGeneology; i++) {
                pszId = SafePrintf(pszId, 16, "%d.", s_Payload.rGeneology[i]);
                if (i < s_Payload.nGeneology - 1) {
                    pszParent = SafePrintf(pszParent, 16, "%d.", s_Payload.rGeneology[i]);
                }
            }
            *pszId = '\0';
            *pszParent = '\0';

            if (szParent[0] == '\0') {
                Tblog("<t:Process id=\"::%hs::\"", szId);
            }
            else {
                Tblog("<t:Process id=\"::%hs::\" parentId=\"::%hs::\"", szId, szParent);
            }

            Tblog(" par=\"%ls\" exe=\"%hs\"", s_Payload.wzParents, pszExeName);

            BOOL drop = false;
            PCWSTR pwzz = s_Payload.wzzDrop;
            while (*pwzz) {
                if (Compare(pwzz, pszExeName) == 0) {
                    // match
                    drop = true;
                    break;
                }
                pwzz += Size(pwzz) + 1;
            }
            if (drop) {
                Tblog(" drop=\"true\"");
            }
        }

        {
            PWCHAR pwz = s_Payload.wzParents;
            while (*pwz) {
                pwz++;
            }
            *pwz++ = '/';
            PCSTR psz = pszExeName;
            while (*psz) {
                *pwz++ = *psz++;
            }
            *pwz = '\0';
        }


        if (HasChar(pwzLine, '|')) {
            Tblog(" pipes=\"true\"");
        }
        if (HasChar(pwzLine, '>')) {
            Tblog(" redirects=\"true\"");
        }

        Tblog(" xmlns:t=\"http://schemas.microsoft.com/research/tracebld/2008\">\n");

        // Get the directory.
        DWORD dwSize = GetCurrentDirectoryA(ARRAYSIZE(szExeName), szExeName);
        if (dwSize > 0 && dwSize < ARRAYSIZE(szExeName)) {
            Tblog("<t:Directory>%hs</t:Directory>\n", szExeName);
        }

        // Get the real executable name.
        wzPath[0] = '\0';
        if (GetModuleFileNameA(0, szExeName, ARRAYSIZE(szExeName))) {
            FileInfo *pInfo = FileNames::FindPartial(szExeName);
            Tblog("<t:Executable>%ls</t:Executable>\n",
                  FileNames::ParameterizeName(wzPath, ARRAYSIZE(wzPath), pInfo));
        }

        // Construct the processed command line.
        PWCHAR pwzDstEnd = (PWCHAR)pwzLine;
        PWCHAR pwzDst = pwzDstEnd;
        pwzDst = LoadCommandLine(pwzLine, pwzDst, pwzDstEnd);
        DWORD wcNew = (DWORD)((pwzDst - pwzDstEnd) + 1);
        PWCHAR pwzFin = (PWCHAR)GlobalAlloc(GPTR, wcNew * sizeof(WCHAR));
        pwzDst = pwzFin;
        pwzDstEnd = pwzFin + wcNew;
        pwzDst = LoadCommandLine(pwzLine, pwzDst, pwzDstEnd);
        *pwzDst = '\0';

        FileNames::ParameterizeLine(pwzFin, pwzFin + wcNew);
        if (HasSpace(wzPath)) {
            Tblog("<t:Line>&quot;%le&quot; %le</t:Line>\n", wzPath, pwzFin);
        }
        else {
            Tblog("<t:Line>%le %le</t:Line>\n", wzPath, pwzFin);
        }

        TestHandle("t:StdIn", GetStdHandle(STD_INPUT_HANDLE));
        TestHandle("t:StdOut", GetStdHandle(STD_OUTPUT_HANDLE));
        TestHandle("t:StdErr", GetStdHandle(STD_ERROR_HANDLE));
    }

    if (FindMsvcr()) {
        FindProc(&(PVOID&)Real_getenv, "getenv");
        FindProc(&(PVOID&)Real_wgetenv, "_wgetenv");
        FindProc(&(PVOID&)Real_getenv_s, "getenv_s");
        FindProc(&(PVOID&)Real_wgetenv_s, "_wgetenv_s");
        FindProc(&(PVOID&)Real_dupenv_s, "_dupenv_s");
        FindProc(&(PVOID&)Real_wdupenv_s, "_wdupenv_s");

        DetourTransactionBegin();
        DetourUpdateThread(GetCurrentThread());

        DetourAttachIf(&(PVOID&)Real_getenv, Mine_getenv);
        DetourAttachIf(&(PVOID&)Real_getenv_s, Mine_getenv_s);
        DetourAttachIf(&(PVOID&)Real_wgetenv, Mine_wgetenv);
        DetourAttachIf(&(PVOID&)Real_wgetenv, Mine_wgetenv_s);
        DetourAttachIf(&(PVOID&)Real_dupenv_s, Mine_dupenv_s);
        DetourAttachIf(&(PVOID&)Real_wdupenv_s, Mine_wdupenv_s);

        DetourTransactionCommit();
    }

    return Real_EntryPoint();
}

VOID WINAPI Mine_ExitProcess(UINT a0)
{
    if (a0 & 0x80000000) {
        Tblog("<t:Return>%d</t:Return>\n", -(int)a0);
    }
    else {
        Tblog("<t:Return>%d</t:Return>\n", a0);
    }

    FileNames::Dump();
    EnvVars::Dump();

    TblogClose();

    Real_ExitProcess(a0);
}

BOOL APIENTRY DllMain(HINSTANCE hModule, DWORD dwReason, PVOID lpReserved)
{
    (void)hModule;
    (void)lpReserved;

    if (DetourIsHelperProcess()) {
        return TRUE;
    }

    if (dwReason == DLL_PROCESS_ATTACH) {
        DetourRestoreAfterWith();
        Real_EntryPoint = (int (WINAPI *)(VOID))DetourGetEntryPoint(NULL);
        return ProcessAttach(hModule);
    }
    else if (dwReason == DLL_PROCESS_DETACH) {
        return ProcessDetach(hModule);
    }
    else if (dwReason == DLL_THREAD_ATTACH) {
        return ThreadAttach(hModule);
    }
    else if (dwReason == DLL_THREAD_DETACH) {
        return ThreadDetach(hModule);
    }
    return TRUE;
}
//
///////////////////////////////////////////////////////////////// End of File.
