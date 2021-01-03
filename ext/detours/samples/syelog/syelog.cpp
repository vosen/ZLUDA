//////////////////////////////////////////////////////////////////////////////
//
//  Detours Test Program (syelog.cpp of syelog.lib)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
#include <windows.h>
#include <stdarg.h>
#include <stddef.h>
#include <stdlib.h>
#include "detours.h"
#include "syelog.h"

#include <stdio.h>

//////////////////////////////////////////////////////////////////////////////
extern "C" {
    extern HANDLE ( WINAPI * Real_CreateFileW)(LPCWSTR a0,
                                               DWORD a1,
                                               DWORD a2,
                                               LPSECURITY_ATTRIBUTES a3,
                                               DWORD a4,
                                               DWORD a5,
                                               HANDLE a6);
    extern BOOL ( WINAPI * Real_WriteFile)(HANDLE hFile,
                                           LPCVOID lpBuffer,
                                           DWORD nNumberOfBytesToWrite,
                                           LPDWORD lpNumberOfBytesWritten,
                                           LPOVERLAPPED lpOverlapped);
    extern BOOL ( WINAPI * Real_FlushFileBuffers)(HANDLE hFile);
    extern BOOL ( WINAPI * Real_CloseHandle)(HANDLE hObject);

    extern BOOL ( WINAPI * Real_WaitNamedPipeW)(LPCWSTR lpNamedPipeName, DWORD nTimeOut);
    extern BOOL ( WINAPI * Real_SetNamedPipeHandleState)(HANDLE hNamedPipe,
                                                         LPDWORD lpMode,
                                                         LPDWORD lpMaxCollectionCount,
                                                         LPDWORD lpCollectDataTimeout);

    extern DWORD ( WINAPI * Real_GetCurrentProcessId)(VOID);
    extern VOID ( WINAPI * Real_GetSystemTimeAsFileTime)(LPFILETIME lpSystemTimeAsFileTime);

    extern VOID ( WINAPI * Real_InitializeCriticalSection)(LPCRITICAL_SECTION lpSection);
    extern VOID ( WINAPI * Real_EnterCriticalSection)(LPCRITICAL_SECTION lpSection);
    extern VOID ( WINAPI * Real_LeaveCriticalSection)(LPCRITICAL_SECTION lpSection);
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
                                pszOut = do_str(pszOut, pszEnd, "<NULL>");
                            }
                            else if (pvData < (PVOID)0x10000) {
                                pszOut = do_str(pszOut, pszEnd, "#");
                                pszOut = do_base(pszOut, (UINT64)pvData, 16,
                                             "0123456789ABCDEF");
                                pszOut = do_str(pszOut, pszEnd, "#");
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
                                pszOut = do_str(pszOut, pszEnd, "<NULL>");
                            }
                            else if (pvData < (PVOID)0x10000) {
                                pszOut = do_str(pszOut, pszEnd, ">");
                                pszOut = do_base(pszOut, (UINT64)pvData, 16,
                                             "0123456789ABCDEF");
                                pszOut = do_str(pszOut, pszEnd, ">");
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

                    if ((INT64)value == (INT64)-1 ||
                        (INT64)value == (INT64)-2) {
                        if (*pszMsg == 'p') {
                            pszMsg++;
                        }
                        szTemp[0] = '-';
                        szTemp[1] = ((INT64)value == (INT64)-1) ? '1' : '2';
                        szTemp[2] = '\0';
                        nLen = 2;
                    }
                    else {
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
static CRITICAL_SECTION s_csPipe;                       // Guards access to hPipe.
static HANDLE           s_hPipe = INVALID_HANDLE_VALUE;
static DWORD            s_nPipeError = 0;
static FILETIME         s_ftRetry = {0,0};
static BYTE             s_nFacility = SYELOG_FACILITY_APPLICATION;
static CHAR             s_szIdent[256] = "";
static DWORD            s_nProcessId = 0;

static inline INT syelogCompareTimes(CONST PFILETIME pft1, CONST PFILETIME pft2)
{
    INT64 ut1 = *(PINT64)pft1;
    INT64 ut2 = *(PINT64)pft2;

    if (ut1 < ut2) {
        return -1;
    }
    else if (ut1 > ut2) {
        return 1;
    }
    else {
        return 0;
    }
}

static inline VOID syelogAddMilliseconds(PFILETIME pft, DWORD nMilliseconds)
{
    *(PINT64&)pft += ((INT64)nMilliseconds * 10000);
}

//////////////////////////////////////////////////////////////////////////////
//
// Tries to insure that a named-pipe connection to the system log is open
// If the pipe closes, the next call will immediately try to re-open the pipe.
// If the pipe doesn't open again, we wait 5 minutes before trying again.
// We wait 5 minutes, because each attempt may take up to a full second to
// time out.
//
static BOOL syelogIsOpen(PFILETIME pftLog)
{
    if (s_hPipe != INVALID_HANDLE_VALUE) {
        return TRUE;
    }

    if (syelogCompareTimes(pftLog, &s_ftRetry) < 0) {
        return FALSE;
    }

    s_hPipe = Real_CreateFileW(SYELOG_PIPE_NAMEW,
                               GENERIC_WRITE, 0, NULL, OPEN_EXISTING,
                               SECURITY_ANONYMOUS, NULL);
    if (s_hPipe != INVALID_HANDLE_VALUE) {
        DWORD dwMode = PIPE_READMODE_MESSAGE;
        if (Real_SetNamedPipeHandleState(s_hPipe, &dwMode, NULL, NULL)) {
            return TRUE;
        }
    }

    if (Real_WaitNamedPipeW(SYELOG_PIPE_NAMEW, 2000)) { // Wait 2 seconds.
        // Pipe connected, change to message-read mode.
        //
        s_hPipe = Real_CreateFileW(SYELOG_PIPE_NAMEW,
                                   GENERIC_WRITE, 0, NULL, OPEN_EXISTING,
                                   SECURITY_ANONYMOUS, NULL);
        if (s_hPipe != INVALID_HANDLE_VALUE) {
            DWORD dwMode = PIPE_READMODE_MESSAGE;
            if (Real_SetNamedPipeHandleState(s_hPipe, &dwMode, NULL, NULL)) {
                return TRUE;
            }
        }
    }

    // Couldn't open pipe.
    s_ftRetry = *pftLog;
    syelogAddMilliseconds(&s_ftRetry, 300000);           // Wait 5 minute before retry.

    return FALSE;
}

VOID SyelogOpen(PCSTR pszIdentifier, BYTE nFacility)
{
    Real_InitializeCriticalSection(&s_csPipe);

    if (pszIdentifier) {
        PCHAR pszOut = s_szIdent;
        PCHAR pszEnd = s_szIdent + ARRAYSIZE(s_szIdent) - 1;
        pszOut = do_str(pszOut, pszEnd, pszIdentifier);
        pszOut = do_str(pszOut, pszEnd, ": ");
        *pszEnd = '\0';
    }
    else {
        s_szIdent[0] = '\0';
    }

    s_nFacility = nFacility;
    s_nProcessId = Real_GetCurrentProcessId();
}

VOID SyelogExV(BOOL fTerminate, BYTE nSeverity, PCSTR pszMsgf, va_list args)
{
    SYELOG_MESSAGE Message;
    DWORD cbWritten = 0;

    Real_GetSystemTimeAsFileTime(&Message.ftOccurance);
    Message.fTerminate = fTerminate;
    Message.nFacility = s_nFacility;
    Message.nSeverity = nSeverity;
    Message.nProcessId = s_nProcessId;
    PCHAR pszBuf = Message.szMessage;
    PCHAR pszEnd = Message.szMessage + ARRAYSIZE(Message.szMessage) - 1;
    if (s_szIdent[0]) {
        pszBuf = do_str(pszBuf, pszEnd, s_szIdent);
    }
    *pszEnd = '\0';
    VSafePrintf(pszMsgf, args,
                pszBuf, (int)(Message.szMessage + sizeof(Message.szMessage) - 1 - pszBuf));

    pszEnd = Message.szMessage;
    for (; *pszEnd; pszEnd++) {
        // no internal contents.
    }

    // Insure that the message always ends with a '\n'
    //
    if (pszEnd > Message.szMessage) {
        if (pszEnd[-1] != '\n') {
            *pszEnd++ = '\n';
            *pszEnd++ = '\0';
        }
        else {
            *pszEnd++ = '\0';
        }
    }
    else {
        *pszEnd++ = '\n';
        *pszEnd++ = '\0';
    }
    Message.nBytes = (USHORT)(pszEnd - ((PCSTR)&Message));

    Real_EnterCriticalSection(&s_csPipe);

    if (syelogIsOpen(&Message.ftOccurance)) {
        if (!Real_WriteFile(s_hPipe, &Message, Message.nBytes, &cbWritten, NULL)) {
            s_nPipeError = GetLastError();
            if (s_nPipeError == ERROR_BAD_IMPERSONATION_LEVEL) {
                // Don't close the file just for a temporary impersonation level.
            }
            else {
                if (s_hPipe != INVALID_HANDLE_VALUE) {
                    Real_CloseHandle(s_hPipe);
                    s_hPipe = INVALID_HANDLE_VALUE;
                }
                if (syelogIsOpen(&Message.ftOccurance)) {
                    Real_WriteFile(s_hPipe, &Message, Message.nBytes, &cbWritten, NULL);
                }
            }
        }
    }

    Real_LeaveCriticalSection(&s_csPipe);
}

VOID SyelogV(BYTE nSeverity, PCSTR pszMsgf, va_list args)
{
    SyelogExV(FALSE, nSeverity, pszMsgf, args);
}

VOID Syelog(BYTE nSeverity, PCSTR pszMsgf, ...)
{
    va_list args;
    va_start(args, pszMsgf);
    SyelogExV(FALSE, nSeverity, pszMsgf, args);
    va_end(args);
}

VOID SyelogEx(BOOL fTerminate, BYTE nSeverity, PCSTR pszMsgf, ...)
{
    va_list args;
    va_start(args, pszMsgf);
    SyelogExV(fTerminate, nSeverity, pszMsgf, args);
    va_end(args);
}

VOID SyelogClose(BOOL fTerminate)
{
    if (fTerminate) {
        SyelogEx(TRUE, SYELOG_SEVERITY_NOTICE, "Requesting exit on close.\n");
    }

    Real_EnterCriticalSection(&s_csPipe);

    if (s_hPipe != INVALID_HANDLE_VALUE) {
        Real_FlushFileBuffers(s_hPipe);
        Real_CloseHandle(s_hPipe);
        s_hPipe = INVALID_HANDLE_VALUE;
    }

    Real_LeaveCriticalSection(&s_csPipe);
}
//
///////////////////////////////////////////////////////////////// End of File.
