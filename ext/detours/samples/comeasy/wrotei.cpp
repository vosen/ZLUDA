//////////////////////////////////////////////////////////////////////////////
//
//  Detour Test Program (wrotei.cpp of wrotei.dll)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
//  An example dynamically detouring a function.
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
LONG dwWrote = 0;

static int (WINAPI * TrueEntryPoint)(VOID) = NULL;
static int (WINAPI * RawEntryPoint)(VOID) = NULL;

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

    hr = RealIStreamWrite(This, pv, cb, pcbWritten);

    for (;;) {
        LONG dwOld = dwWrote;
        LONG dwNew = dwOld + *pcbWritten;

        if (InterlockedCompareExchange(&dwWrote, dwNew, dwOld) == dwOld) {
            break;
        }
    }

    return hr;
}

//////////////////////////////////////////////////////////////////////////////
//
int WINAPI TimedEntryPoint(VOID)
{
    // We couldn't call CoInitializeEx in DllMain,
    // so we detour the vtable entries here...
    LONG error;
    LPSTREAM pStream = NULL;

    // Create a temporary object so we can get a vtable.
    CreateStreamOnHGlobal(NULL, TRUE, &pStream);

    // Apply the detour to the vtable.
    DetourTransactionBegin();
    DetourUpdateThread(GetCurrentThread());
    if (pStream != NULL) {
        RealIStreamWrite = pStream->lpVtbl->Write;
        DetourAttach(&(PVOID&)RealIStreamWrite, MineIStreamWrite);
    }
    error = DetourTransactionCommit();

    if (pStream != NULL) {
        pStream->lpVtbl->Release(pStream);
        pStream = NULL;
    }

    if (error == NO_ERROR) {
        printf("wrotei" DETOURS_STRINGIFY(DETOURS_BITS) ".dll:"
               " Detoured IStream::Wrote() from OnHGlobal.\n");
    }
    else {
        printf("wrotei" DETOURS_STRINGIFY(DETOURS_BITS) ".dll:"
               " Error detouring IStram::Wrote(): %ld\n", error);
    }

    printf("wrotei" DETOURS_STRINGIFY(DETOURS_BITS) ".dll:"
           " Calling EntryPoint\n\n");
    fflush(stdout);

    return TrueEntryPoint();
}

BOOL WINAPI DllMain(HINSTANCE hinst, DWORD dwReason, LPVOID reserved)
{
    LONG error;
    (void)hinst;
    (void)reserved;

    if (DetourIsHelperProcess()) {
        return TRUE;
    }

    if (dwReason == DLL_PROCESS_ATTACH) {
        DetourRestoreAfterWith();

        printf("wrotei" DETOURS_STRINGIFY(DETOURS_BITS) ".dll:"
               " Starting.\n");
        fflush(stdout);

        // NB: DllMain can't call LoadLibrary, so we hook the app entry point.
        TrueEntryPoint = (int (WINAPI *)(VOID))DetourGetEntryPoint(NULL);
        RawEntryPoint = TrueEntryPoint;

        DetourTransactionBegin();
        DetourUpdateThread(GetCurrentThread());
        DetourAttach(&(PVOID&)TrueEntryPoint, TimedEntryPoint);
        error = DetourTransactionCommit();

        if (error == NO_ERROR) {
            printf("wrotei" DETOURS_STRINGIFY(DETOURS_BITS) ".dll:"
                   " Detoured EntryPoint().\n");
        }
        else {
            printf("wrotei" DETOURS_STRINGIFY(DETOURS_BITS) ".dll:"
                   " Error detouring EntryPoint(): %ld\n", error);
        }
    }
    else if (dwReason == DLL_PROCESS_DETACH) {
        DetourTransactionBegin();
        DetourUpdateThread(GetCurrentThread());
        if (RealIStreamWrite != NULL) {
            DetourDetach(&(PVOID&)RealIStreamWrite, (PVOID)MineIStreamWrite);
        }
        DetourDetach(&(PVOID&)TrueEntryPoint, TimedEntryPoint);
        error = DetourTransactionCommit();

        printf("wrotei" DETOURS_STRINGIFY(DETOURS_BITS) ".dll:"
               " Removed IStream::Wrote() detours (%ld), wrote %ld bytes.\n",
               error, dwWrote);

        fflush(stdout);
    }
    return TRUE;
}
//
///////////////////////////////////////////////////////////////// End of File.
