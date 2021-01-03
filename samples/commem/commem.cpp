//////////////////////////////////////////////////////////////////////////////
//
//  Detour functions of a COM interface (commem.cpp of commem.exe)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
//
//
#include <stdio.h>

//////////////////////////////////////////////////////////////////////////////
//
//  WARNING:
//
//  CINTERFACE must be defined so that the lpVtbl pointer is visible
//  on COM interfaces.  However, once we've defined it, we must use
//  coding conventions when accessing interface members, for example:
//      i->lpVtbl->Write
//  instead of the C++ syntax:
//      i->Write.
//  We must also pass the implicit "this" parameter explicitly:
//      i->lpVtbl->Write(i, pb, 0, NULL)
//  instead of the C++ syntax:
//      i->Write(pb, 0, NULL)
//
#define CINTERFACE
#include <ole2.h>
#include <windows.h>
#include <detours.h>

//////////////////////////////////////////////////////////////////////////////
//
HRESULT (STDMETHODCALLTYPE *RealIStreamWrite)(IStream * This,
                                              const void *pv,
                                              ULONG cb,
                                              ULONG *pcbWritten) = NULL;

HRESULT STDMETHODCALLTYPE MineIStreamWrite(IStream * This,
                                           const void *pv,
                                           ULONG cb,
                                           ULONG *pcbWritten)
{
    HRESULT hr;
    ULONG cbWritten = 0;
    if (pcbWritten == NULL) {
        pcbWritten = &cbWritten;
    }

    printf("commem:   %p->IStreamWrite(pv=%p, cb=%ld)\n", This, pv, cb);
    hr = RealIStreamWrite(This, pv, cb, pcbWritten);
    printf("commem:   %p->IStreamWrite -> %08lx (pcbWritten=%ld)\n", This, hr, *pcbWritten);

    return hr;
}

//////////////////////////////////////////////////////////////////////////////
//
int main(int argc, char **argv)
{
    HRESULT hr;

    (void)argc;
    (void)argv;

    LPSTREAM pStream = NULL;
    ULARGE_INTEGER ul;
    LARGE_INTEGER li;

    CoInitialize(NULL);

    hr = CreateStreamOnHGlobal(NULL, TRUE, &pStream);

    RealIStreamWrite = pStream->lpVtbl->Write;

    ul.QuadPart = 512;
    hr = pStream->lpVtbl->SetSize(pStream, ul);
    li.QuadPart = 0;
    hr = pStream->lpVtbl->Seek(pStream, li, STREAM_SEEK_SET, NULL);

    printf("commem: Calling Write w/o before attach.\n");

    li.QuadPart = 0;
    hr = pStream->lpVtbl->Write(pStream, &ul, sizeof(ul), NULL);

    DetourTransactionBegin();
    DetourUpdateThread(GetCurrentThread());
    DetourAttach(&(PVOID&)RealIStreamWrite, MineIStreamWrite);
    DetourTransactionCommit();

    printf("commem: Calling Write w/o after attach.\n");

    li.QuadPart = 1;
    hr = pStream->lpVtbl->Write(pStream, &li, sizeof(li), NULL);

    DetourTransactionBegin();
    DetourUpdateThread(GetCurrentThread());
    DetourDetach(&(PVOID&)RealIStreamWrite, MineIStreamWrite);
    DetourTransactionCommit();

    printf("commem: Calling Write w/o after detach.\n");

    li.QuadPart = 2;
    hr = pStream->lpVtbl->Write(pStream, &li, sizeof(li), NULL);

    hr = pStream->lpVtbl->Release(pStream);
    pStream = NULL;

    CoUninitialize();

    return 0;
}

