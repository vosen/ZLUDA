//////////////////////////////////////////////////////////////////////////////
//
//  Detours Test Program (testapi.cpp of testapi.exe)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
#include "trcapi.cpp"

#if (_MSC_VER < 1299)
typedef ULONG * PULONG_PTR;
typedef ULONG ULONG_PTR;
typedef LONG * PLONG_PTR;
typedef LONG LONG_PTR;
#endif

VOID SyelogOpen(PCSTR pszIdentifier, BYTE nFacility)
{
    (void)pszIdentifier;
    (void)nFacility;
}

VOID SyelogExV(BOOL fTerminate, BYTE nSeverity, PCSTR pszMsgf, va_list args)
{
    (void)fTerminate;

    CHAR szBuffer[1024];
    PCHAR psz = szBuffer;
    BOOL fLf = FALSE;

    StringCchPrintfA(psz, szBuffer + sizeof(szBuffer) - psz, "--.%02x: ", nSeverity);
    while (*psz) {
        psz++;
    }

    StringCchVPrintfA(psz, szBuffer + sizeof(szBuffer) - psz, pszMsgf, args);
    for (psz = szBuffer; *psz; psz++) {
        if (*psz == '\n') {
            if (fLf) {
                *psz = '\0';
                break;
            }
            fLf = TRUE;
        }
    }
    if (!fLf) {
        *psz++ = '\n';
        *psz = '\0';
    }
    printf("%s", szBuffer);
    Real_OutputDebugStringA(szBuffer);
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
    (void)fTerminate;
}

DWORD main(int argc, char **argv)
{
    (void)argc;
    (void)argv;

    printf("testapi: Starting\n");
    ProcessAttach(NULL);
    Sleep(100);
    ProcessDetach(NULL);

    return 0;
}
//
//////////////////////////////////////////////////////////////////////////////
