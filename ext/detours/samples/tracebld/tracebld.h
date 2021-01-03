//////////////////////////////////////////////////////////////////////////////
//
//  Detours Test Program (tracebld.h of tracebld.exe)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
#pragma once
#ifndef _TRACEBLD_H_
#define _TRACEBLD_H_
#include <stdarg.h>

//////////////////////////////////////////////////////////////////////////////
//
//
#define TBLOG_PIPE_NAMEA       "\\\\.\\pipe\\tracebuild"
#define TBLOG_PIPE_NAMEW       L"\\\\.\\pipe\\tracebuild"
#ifdef UNICODE
#define TBLOG_PIPE_NAME        TBLOG_PIPE_NAMEW
#else
#define TBLOG_PIPE_NAME        TBLOG_PIPE_NAMEA
#endif

//////////////////////////////////////////////////////////////////////////////
//
typedef struct _TBLOG_MESSAGE
{
    DWORD       nBytes;
    CHAR        szMessage[32764]; // 32768 - sizeof(nBytes)
} TBLOG_MESSAGE, *PTBLOG_MESSAGE;

typedef struct _TBLOG_PAYLOAD
{
    DWORD       nParentProcessId;
    DWORD       nTraceProcessId;
    DWORD       nGeneology;
    DWORD       rGeneology[64];
    WCHAR       wzParents[256];
    WCHAR       wzStdin[256];
    WCHAR       wzStdout[256];
    WCHAR       wzStderr[256];
    BOOL        fStdoutAppend;
    BOOL        fStderrAppend;
    WCHAR       wzzDrop[1024];  // Like an environment: zero terminated strings with a last zero.
    WCHAR       wzzEnvironment[32768];
} TBLOG_PAYLOAD, *PTBLOG_PAYLOAD;

// Shared state payload guid.
//
const GUID s_guidTrace = {
    0xd8e2dc69, 0x3004, 0x453e,
    {0x94, 0x15, 0x19, 0x0e, 0x79, 0xe8, 0x93, 0x52}
};


#endif //  _TRACEBLD_H_
//
///////////////////////////////////////////////////////////////// End of File.
