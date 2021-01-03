//////////////////////////////////////////////////////////////////////////////
//
//  Detours Test Program (dumpi.cpp of dumpi.exe)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
#include <stdio.h>
#include <stdlib.h>
#include <windows.h>
#include <shellapi.h>
#pragma warning(push)
#if _MSC_VER > 1400
#pragma warning(disable:6102 6103) // /analyze warnings
#endif
#include <strsafe.h>
#pragma warning(pop)
#include <detours.h>

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
static CHAR s_szFile[MAX_PATH] = "\0";

static BOOL CALLBACK ListFileCallback(_In_opt_ PVOID pContext,
                                      _In_z_ LPCSTR pszOrigFile,
                                      _In_z_ LPCSTR pszFile,
                                      _Outptr_result_maybenull_ LPCSTR *ppszOutFile)
{
    (void)pContext;
    (void)pszFile;

    *ppszOutFile = NULL;

    StringCchCopyA(s_szFile, sizeof(s_szFile), pszOrigFile);

    PCHAR psz;
    if ((psz = strchr(s_szFile, '.')) != NULL) {
        *psz = '\0';
    }
    return TRUE;
}

BOOL CALLBACK ListSymbolCallback(_In_opt_ PVOID pContext,
                                 _In_ ULONG nOrigOrdinal,
                                 _In_ ULONG nOrdinal,
                                 _Out_ ULONG *pnOutOrdinal,
                                 _In_opt_z_ LPCSTR pszOrigSymbol,
                                 _In_opt_z_ LPCSTR pszSymbol,
                                 _Outptr_result_maybenull_ LPCSTR *ppszOutSymbol)
{
    (void)pContext;
    (void)nOrdinal;
    (void)pszSymbol;

    *ppszOutSymbol = NULL;
    *pnOutOrdinal = 0;

    if (nOrigOrdinal != 0) {
        printf("  %s::#%ld\n",
               s_szFile, nOrigOrdinal);
    }
    else {
        printf("  %s::%s\n",
               s_szFile, pszOrigSymbol);
    }

    return TRUE;
}

BOOL DimpFile(PCHAR pszPath)
{
    BOOL bGood = TRUE;
    HANDLE hOld = INVALID_HANDLE_VALUE;
    PDETOUR_BINARY pBinary = NULL;


    hOld = CreateFileA(pszPath,
                       GENERIC_READ,
                       FILE_SHARE_READ,
                       NULL,
                       OPEN_EXISTING,
                       FILE_ATTRIBUTE_NORMAL,
                       NULL);

    if (hOld == INVALID_HANDLE_VALUE) {
        printf("%s: Failed to open input file with error: %ld\n",
               pszPath, GetLastError());
        bGood = FALSE;
        goto end;
    }

    if ((pBinary = DetourBinaryOpen(hOld)) == NULL) {
        printf("%s: DetourBinaryOpen failed: %ld\n", pszPath, GetLastError());
        goto end;
    }

    if (hOld != INVALID_HANDLE_VALUE) {
        CloseHandle(hOld);
        hOld = INVALID_HANDLE_VALUE;
    }

    printf("%s:\n", pszPath);
    if (!DetourBinaryEditImports(pBinary,
                                 NULL,
                                 NULL,
                                 ListFileCallback,
                                 ListSymbolCallback,
                                 NULL)) {

        printf("%s: DetourBinaryEditImports failed: %ld\n", pszPath, GetLastError());
    }

    DetourBinaryClose(pBinary);
    pBinary = NULL;

  end:
    if (pBinary) {
        DetourBinaryClose(pBinary);
        pBinary = NULL;
    }
    if (hOld != INVALID_HANDLE_VALUE) {
        CloseHandle(hOld);
        hOld = INVALID_HANDLE_VALUE;
    }
    return bGood;
}

//////////////////////////////////////////////////////////////////////////////
int DimpArgument(char *dir, char *argp, int fDoSubs)
{
    //////////////////////////////////////////////////////////////////////////

    WIN32_FIND_DATAA wfd;
    HANDLE  hFind = NULL;
    char    name[1024];
    int     nFound = 0;

    StringCchCopyA(name, sizeof(name), dir ? dir : "");
    StringCchCatA(name, sizeof(name), argp);

    hFind = FindFirstFileA(name, &wfd);
    if (hFind != INVALID_HANDLE_VALUE) {
        do {
            if (!(wfd.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY)) {
                StringCchCopyA(name, sizeof(name), dir ? dir : "");
                StringCchCatA(name, sizeof(name), wfd.cFileName);

                nFound += DimpFile(name);
            }
        } while (FindNextFileA(hFind, &wfd));
        FindClose(hFind);
    }

    if (fDoSubs) {
        StringCchCopyA(name, sizeof(name), dir ? dir : "");
        StringCchCatA(name, sizeof(name), "*");

        hFind = FindFirstFileA(name, &wfd);
        if (hFind == INVALID_HANDLE_VALUE)
            return nFound;

        do {
            if ((wfd.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY) &&
                wfd.cFileName[0] != '.') {

                StringCchCopyA(name, sizeof(name), dir ? dir : "");
                StringCchCatA(name, sizeof(name), wfd.cFileName);
                StringCchCatA(name, sizeof(name), "\\");

                nFound += DimpArgument(name, argp, fDoSubs);
            }
        } while (FindNextFileA(hFind, &wfd));
        FindClose(hFind);
    }
    return nFound;
}


//////////////////////////////////////////////////////////////////////////////
//
void PrintUsage(void)
{
    printf("Usage:\n"
           "    dimp [options] binary_files\n"
           "Options:\n"
           "    /s           : Recurse through subdirectories.\n"
           "    /?           : This help screen.\n"
           "Examples:\n"
           "    dimp /s *.exe\n"
           "");
}

//////////////////////////////////////////////////////////////////////// main.
//
int CDECL main(int argc, char **argv)
{
    BOOL fNeedHelp = FALSE;
    BOOL fSubdirs = FALSE;

    int arg = 1;
    for (; arg < argc; arg++) {
        if (argv[arg][0] == '-' || argv[arg][0] == '/') {
            CHAR *argn = argv[arg] + 1;
            CHAR *argp = argn;
            while (*argp && *argp != ':')
                argp++;
            if (*argp == ':')
                *argp++ = '\0';

            switch (argn[0]) {

              case 's':                                 // Do Subdirectories.
              case 'S':
                fSubdirs = TRUE;
                break;

              case '?':                                 // Help.
                fNeedHelp = TRUE;
                break;

              default:
                fNeedHelp = TRUE;
                printf("Bad argument: %s:%s\n", argn, argp);
                break;
            }
        }
        else {
            CHAR szDir[MAX_PATH] = "";
            CHAR szArg[MAX_PATH] = "";
            PCHAR pszDir;

            if ((pszDir = strrchr(argv[arg], '\\')) != NULL) {
                *pszDir++ = '\0';
                StringCchCopyA(szArg, sizeof(szArg), pszDir);
                StringCchCopyA(szDir, sizeof(szDir), argv[arg]);
                StringCchCatA(szDir, sizeof(szDir), "\\");
            }
            else {
                if (GetCurrentDirectoryA(sizeof(szDir), szDir) > 3) {
                    StringCchCatA(szDir, sizeof(szDir), "\\");
                }
                StringCchCopyA(szArg, sizeof(szArg), argv[arg]);
            }

            DimpArgument(szDir, szArg, fSubdirs);
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
