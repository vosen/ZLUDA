//////////////////////////////////////////////////////////////////////////////
//
//  Detour Test Program (comeasy.cpp of comeasy.exe)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//

#include <ole2.h>
#include <windows.h>
#include <stdio.h>

//////////////////////////////////////////////////////////////////////////////
//
int __cdecl main(int argc, char **argv)
{
    HRESULT hr;

    (void)argc;
    (void)argv;

    LPSTREAM pStream = NULL;
    ULARGE_INTEGER ul;
    LARGE_INTEGER li;

    printf("comeasy.exe: Starting (at %p).\n", main);

    CoInitialize(NULL);

    hr = CreateStreamOnHGlobal(NULL, TRUE, &pStream);

    ul.QuadPart = 512;
    hr = pStream->SetSize(ul);

    li.QuadPart = 0;
    hr = pStream->Seek(li, STREAM_SEEK_SET, NULL);

    printf("comeasy.exe: First write.\n");
    fflush(stdout);

    li.QuadPart = 0;
    hr = pStream->Write(&ul, sizeof(ul), NULL);

    printf("comeasy.exe: Second write.\n");
    fflush(stdout);

    li.QuadPart = 1;
    hr = pStream->Write(&li, sizeof(li), NULL);

    printf("comeasy.exe: Third write.\n");
    fflush(stdout);

    li.QuadPart = 2;
    hr = pStream->Write(&li, sizeof(li), NULL);

    pStream->Release();
    pStream = NULL;

    CoUninitialize();

    printf("comeasy.exe: Exiting.\n\n");
    fflush(stdout);

    return 0;
}

//
///////////////////////////////////////////////////////////////// End of File.
