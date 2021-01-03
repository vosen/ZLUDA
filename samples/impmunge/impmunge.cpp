//////////////////////////////////////////////////////////////////////////////
//
//  Detours Test Program (impmunge.cpp of impmunge.exe)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
#include <stdio.h>
#include <stdlib.h>
#include <windows.h>
#pragma warning(push)
#if _MSC_VER > 1400
#pragma warning(disable:6102 6103) // /analyze warnings
#endif
#include <strsafe.h>
#include <detours.h>
#pragma warning(disable:4091) // empty typedef
#include <imagehlp.h>
#pragma warning(pop)

////////////////////////////////////////////////////////////// Error Messages.
//
VOID AssertMessage(PCSTR szMsg, PCSTR szFile, DWORD nLine)
{
    printf("ASSERT(%s) failed in %s, line %ld.", szMsg, szFile, nLine);
}

#define ASSERT(x)   \
do { if (!(x)) { AssertMessage(#x, __FILE__, __LINE__); DebugBreak(); }} while (0)
    ;


//////////////////////////////////////////////////////////////////////////////
//
static BOOLEAN s_fRestore = FALSE;
static BOOLEAN s_fList = TRUE;
static BOOLEAN s_fMunge = FALSE;
static BOOLEAN s_fToSymbols = FALSE;

//////////////////////////////////////////////////////////////////////////////
//
static BOOL CALLBACK ListByway(_In_opt_ PVOID pContext,
                               _In_opt_ LPCSTR pszFile,
                               _Outptr_result_maybenull_ LPCSTR *ppszOutFile)
{
    (void)pContext;
    (void)ppszOutFile;

    printf("  byway   -------------------------------- %s\n", pszFile ? pszFile : "");
    return TRUE;
}

static BOOL CALLBACK ListFile(_In_opt_ PVOID pContext,
                              _In_ LPCSTR pszOrigFile,
                              _In_ LPCSTR pszFile,
                              _Outptr_result_maybenull_ LPCSTR *ppszOutFile)
{
    (void)pContext;
    (void)ppszOutFile;

    printf("  file    %-32.32s %-32.32s\n",
           pszOrigFile ? pszOrigFile : "",
           pszFile ? pszFile : "");
    return TRUE;
}

static BOOL CALLBACK ListSymbol(_In_opt_ PVOID pContext,
                                _In_ ULONG nOrigOrdinal,
                                _In_ ULONG nOrdinal,
                                _Out_ ULONG *pnOutOrdinal,
                                _In_opt_ LPCSTR pszOrigSymbol,
                                _In_opt_ LPCSTR pszSymbol,
                                _Outptr_result_maybenull_ LPCSTR *ppszOutSymbol)
{
    (void)pContext;
    (void)pnOutOrdinal;
    (void)ppszOutSymbol;

    char szOrig[80];
    char szLast[80];

    if (pszOrigSymbol == NULL) {
        StringCchPrintfA(szOrig, sizeof(szOrig), "#%d", nOrigOrdinal);
        pszOrigSymbol = szOrig;
    }
    if (pszSymbol == NULL) {
        StringCchPrintfA(szLast, sizeof(szLast), "#%d", nOrdinal);
        pszSymbol = szLast;
    }

    printf("  symbol  %-32.32s %-32.32s\n", pszOrigSymbol, pszSymbol);
    return TRUE;
}

static BOOL CALLBACK ListCommit(PVOID pContext)
{
    (void)pContext;

    printf("  commit\n");
    return TRUE;
}

//////////////////////////////////////////////////////////////////////////////
//
struct MUNGE_STATE
{
    BOOL fLastWasByway;
    LONG nBywayCount;
    CHAR szBuffer[512];
};

static BOOL CALLBACK MungeByway(_In_opt_ PVOID pContext,
                                _In_opt_ LPCSTR pszFile,
                                _Outptr_result_maybenull_ LPCSTR *ppszOutFile)
{
    MUNGE_STATE *pState = (MUNGE_STATE *)pContext;

    printf("|");

    if (pState->fLastWasByway) {
        return TRUE;
    }

    pState->fLastWasByway = TRUE;

    if (pszFile == NULL) {
        StringCchPrintfA(pState->szBuffer, sizeof(pState->szBuffer), "mb_munge_%d.dll", pState->nBywayCount++);
        *ppszOutFile = pState->szBuffer;
    }
    return TRUE;
}

static BOOL CALLBACK MungeFile(_In_opt_ PVOID pContext,
                               _In_ LPCSTR pszOrigFile,
                               _In_ LPCSTR pszFile,
                               _Outptr_result_maybenull_ LPCSTR *ppszOutFile)
{
    (void)pszOrigFile;
    MUNGE_STATE *pState = (MUNGE_STATE *)pContext;

    pState->fLastWasByway = FALSE;

    printf("*");
    StringCchPrintfA(pState->szBuffer, sizeof(pState->szBuffer), "mf_%s", pszFile);
    *ppszOutFile = pState->szBuffer;
    return TRUE;
}

static BOOL CALLBACK MungeSymbol(_In_opt_ PVOID pContext,
                                 _In_ ULONG nOrigOrdinal,
                                 _In_ ULONG nOrdinal,
                                 _Out_ ULONG *pnOutOrdinal,
                                 _In_opt_ LPCSTR pszOrigSymbol,
                                 _In_opt_ LPCSTR pszSymbol,
                                 _Outptr_result_maybenull_ LPCSTR *ppszOutSymbol)
{
    (void)nOrigOrdinal;
    (void)pszOrigSymbol;
    MUNGE_STATE *pState = (MUNGE_STATE *)pContext;

    pState->fLastWasByway = FALSE;

    printf(".");
    if (nOrdinal != 0) {
        if (s_fToSymbols) {
            StringCchPrintfA(pState->szBuffer, sizeof(pState->szBuffer), "mo_%d", (int)nOrdinal);
            *pnOutOrdinal = 0;
            *ppszOutSymbol = pState->szBuffer;
        }
        else {
            *pnOutOrdinal = 10000 + nOrdinal;
            *ppszOutSymbol = NULL;
        }
    }
    else {
        StringCchPrintfA(pState->szBuffer, sizeof(pState->szBuffer), "ms_%s", pszSymbol);
        *pnOutOrdinal = 0;
        *ppszOutSymbol = pState->szBuffer;
    }
    return TRUE;
}

static BOOL CALLBACK MungeCommit(PVOID pContext)
{
    MUNGE_STATE *pState = (MUNGE_STATE *)pContext;

    pState->fLastWasByway = FALSE;

    printf("\n");
    (void)pContext;
    return TRUE;
}

//////////////////////////////////////////////////////////////////////////////
//
static BOOL CALLBACK RestoreByway(_In_opt_ PVOID pContext,
                                  _In_opt_ LPCSTR pszFile,
                                  _Outptr_result_maybenull_ LPCSTR *ppszOutFile)
{
    (void)pContext;
    (void)pszFile;

    *ppszOutFile = NULL;
    return TRUE;
}

static BOOL CALLBACK RestoreFile(_In_opt_ PVOID pContext,
                                 _In_ LPCSTR pszOrigFile,
                                 _In_ LPCSTR pszFile,
                                 _Outptr_result_maybenull_ LPCSTR *ppszOutFile)
{
    (void)pContext;
    (void)pszFile;

    *ppszOutFile = pszOrigFile;
    return TRUE;
}

static BOOL CALLBACK RestoreSymbol(_In_opt_ PVOID pContext,
                                   _In_ ULONG nOrigOrdinal,
                                   _In_ ULONG nOrdinal,
                                   _Out_ ULONG *pnOutOrdinal,
                                   _In_opt_ LPCSTR pszOrigSymbol,
                                   _In_opt_ LPCSTR pszSymbol,
                                   _Outptr_result_maybenull_ LPCSTR *ppszOutSymbol)
{
    (void)pContext;
    (void)nOrdinal;
    (void)pszSymbol;

    *pnOutOrdinal = nOrigOrdinal;
    *ppszOutSymbol = pszOrigSymbol;
    return TRUE;
}

static BOOL CALLBACK RestoreCommit(PVOID pContext)
{
    (void)pContext;
    return TRUE;
}

//////////////////////////////////////////////////////////////////////////////
//

BOOL EditFile(PCHAR pszInput, PCHAR pszOutput)
{
    BOOL fGood = TRUE;

    HANDLE hOld = INVALID_HANDLE_VALUE;
    HANDLE hNew = INVALID_HANDLE_VALUE;
    PDETOUR_BINARY pBinary = NULL;

    if (pszOutput != NULL) {
        printf("%s -> %s:\n", pszInput, pszOutput);
    }
    else {
        printf("%s:\n", pszInput);
    }

    hOld = CreateFileA(pszInput,
                       GENERIC_READ,
                       FILE_SHARE_READ,
                       NULL,
                       OPEN_EXISTING,
                       FILE_ATTRIBUTE_NORMAL,
                       NULL);

    if (hOld == INVALID_HANDLE_VALUE) {
        printf("Couldn't open input file: %s, error: %ld\n",
               pszInput, GetLastError());
        fGood = FALSE;
        goto end;
    }

    if ((pBinary = DetourBinaryOpen(hOld)) == NULL) {
        printf("DetourBinaryOpen failed: %ld\n", GetLastError());
        goto end;
    }

    if (hOld != INVALID_HANDLE_VALUE) {
        CloseHandle(hOld);
        hOld = INVALID_HANDLE_VALUE;
    }

    if (s_fRestore) {
        if (!DetourBinaryEditImports(pBinary,
                                     NULL,
                                     RestoreByway,
                                     RestoreFile,
                                     RestoreSymbol,
                                     RestoreCommit)) {

            printf("DetourBinaryEditImports for munge failed: %ld\n", GetLastError());
        }
    }

    if (s_fMunge) {
        MUNGE_STATE state;
        state.fLastWasByway = FALSE;
        state.nBywayCount = 1;

        if (!DetourBinaryEditImports(pBinary,
                                     &state,
                                     MungeByway,
                                     MungeFile,
                                     MungeSymbol,
                                     MungeCommit)) {

            printf("DetourBinaryEditImports for munge failed: %ld\n", GetLastError());
        }
    }

    if (s_fList) {
        if (!DetourBinaryEditImports(pBinary,
                                     NULL,
                                     ListByway,
                                     ListFile,
                                     ListSymbol,
                                     ListCommit)) {

            printf("DetourBinaryEditImports for list failed: %ld\n", GetLastError());
        }
    }

    if (pszOutput != NULL) {
        hNew = CreateFileA(pszOutput,
                           GENERIC_WRITE | GENERIC_READ, 0, NULL, CREATE_ALWAYS,
                           FILE_ATTRIBUTE_NORMAL | FILE_FLAG_SEQUENTIAL_SCAN, NULL);
        if (hNew == INVALID_HANDLE_VALUE) {
            printf("Couldn't open output file: %s, error: %ld\n",
                   pszOutput, GetLastError());
            fGood = FALSE;
            goto end;
        }

        if (!DetourBinaryWrite(pBinary, hNew)) {
            printf("DetourBinaryWrite failed: %ld\n", GetLastError());
            fGood = FALSE;
        }

        CloseHandle(hNew);
        hNew = INVALID_HANDLE_VALUE;
    }

    DetourBinaryClose(pBinary);
    pBinary = NULL;


    if (fGood && pszOutput != NULL) {
        if (!BindImageEx(BIND_NO_BOUND_IMPORTS, pszOutput, ".", ".", NULL)) {
            printf("Warning: Couldn't bind binary %s: %ld\n", pszOutput, GetLastError());
        }
    }

  end:
    if (pBinary) {
        DetourBinaryClose(pBinary);
        pBinary = NULL;
    }
    if (hNew != INVALID_HANDLE_VALUE) {
        CloseHandle(hNew);
        hNew = INVALID_HANDLE_VALUE;
    }
    if (hOld != INVALID_HANDLE_VALUE) {
        CloseHandle(hOld);
        hOld = INVALID_HANDLE_VALUE;
    }
    return fGood;
}

//////////////////////////////////////////////////////////////////////////////
//
void PrintUsage(void)
{
    printf("Usage:\n"
           "    impmunge [options] binary_files\n"
           "Options:\n"
           "    /l           : List imports.\n"
           "    /l-          : Don't list imports.\n"
           "    /m           : Munge imports.\n"
           "    /r           : Remove import munges.\n"
           "    /o:file      : Set name of output file; must be include with /m or /r.\n"
           "    /?           : This help screen.\n");
}

//////////////////////////////////////////////////////////////////////// main.
//
int CDECL main(int argc, char **argv)
{
    BOOL fNeedHelp = FALSE;
    PCHAR pszOutput = NULL;

    int arg = 1;
    for (; arg < argc && !fNeedHelp; arg++) {
        if (argv[arg][0] == '-' || argv[arg][0] == '/') {
            CHAR *argn = argv[arg] + 1;
            CHAR *argp = argn;
            while (*argp && *argp != ':')
                argp++;
            if (*argp == ':')
                *argp++ = '\0';

            switch (argn[0]) {

              case 'l':                                 // List contents of import table.
              case 'L':
                s_fList = (argn[1] != '-');
                break;

              case 'm':                                 // Munge import table.
              case 'M':
                s_fMunge = (argn[1] != '-');
                break;

              case 'o':                                 // Set output file name.
              case 'O':
                pszOutput = argp;
                break;
              case 'r':                                 // Restore file to unmunged state.
              case 'R':
                s_fRestore = (argn[1] != '-');
                break;

              case 's':                                 // Munge ordinals to symbols
              case 'S':
                s_fToSymbols = true;
                break;

              case '?':                                 // Help
                fNeedHelp = TRUE;
                break;

              default:
                fNeedHelp = TRUE;
                printf("Bad argument: %s:%s\n", argn, argp);
                break;
            }
        }
        else {
            if (!s_fList && !s_fMunge && !s_fRestore) {
                fNeedHelp = TRUE;
                break;
            }
            if (pszOutput == NULL && (s_fMunge || s_fRestore)) {
                fNeedHelp = TRUE;
                break;
            }

            EditFile(argv[arg], pszOutput);
            pszOutput = NULL;
        }
    }
    if (argc == 1) {
        fNeedHelp = TRUE;
    }
    if (fNeedHelp) {
        PrintUsage();
        return 1;
    }
    return 0;
}

// End of File
