//////////////////////////////////////////////////////////////////////////////
//
//  Detours Test Program (einst.cpp of einst.exe)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
#include <stdio.h>
#include <windows.h>
#include <detours.h>

struct CPrivateStuff
{
    DETOUR_SECTION_HEADER   header;
    DETOUR_SECTION_RECORD   record;
    CHAR                    szMessage[32];
};

#ifdef INCLUDE_THIS
#pragma data_seg(".detour")

static CPrivateStuff private_stuff = {
    DETOUR_SECTION_HEADER_DECLARE(sizeof(CPrivateStuff)),
    {
        (sizeof(CPrivateStuff) - sizeof(DETOUR_SECTION_HEADER)),
        0,
        { /* d9ab8a40-f4cc-11d1-b6d7-006097b010e3 */
            0xd9ab8a40,
            0xf4cc,
            0x11d1,
            {0xb6, 0xd7, 0x00, 0x60, 0x97, 0xb0, 0x10, 0xe3}
        }
    },
    "The Application!"
};
#pragma data_seg()
#endif

GUID my_guid =
{ /* d9ab8a40-f4cc-11d1-b6d7-006097b010e3 */
    0xd9ab8a40,
    0xf4cc,
    0x11d1,
    {0xb6, 0xd7, 0x00, 0x60, 0x97, 0xb0, 0x10, 0xe3}
};

__declspec(dllimport) VOID WINAPI EDll1Function(VOID);
__declspec(dllimport) VOID WINAPI EDll2Function(VOID);
__declspec(dllimport) VOID WINAPI EDll3Function(VOID);

void FindPayload(HINSTANCE hinst)
{
    CHAR szModuleName[256];
    GetModuleFileNameA(hinst, szModuleName, ARRAYSIZE(szModuleName));
    printf("  %p : %s\n", hinst, szModuleName);

    ULONG cbData = 0;
    PBYTE pbData = (PBYTE)DetourFindPayload(hinst, my_guid, &cbData);

    if (pbData) {
        printf("  %08p..%08p : %50.50s\n",
               pbData,
               pbData + cbData,
               pbData);
    }
}

int WINAPI WinMain(HINSTANCE hinst, HINSTANCE hprev, LPSTR lpszCmdLine, int nCmdShow)
{
    (void)hinst;
    (void)hprev;
    (void)lpszCmdLine;
    (void)nCmdShow;

    printf("Source .EXE:\n");
    FindPayload(NULL);
    printf("\n");

    printf("DLL and EXE binaries loaded:\n");

    EDll1Function();
    EDll2Function();
    EDll3Function();

    for (HINSTANCE hiter = NULL; (hiter = DetourEnumerateModules(hiter)) != NULL;) {
        FindPayload(hiter);
    }

    if ((PVOID)hinst == (PVOID)lpszCmdLine) {
        DispatchMessage(NULL);                          // Force load of gdi32.dll
    }

    return 0;
}

//
///////////////////////////////////////////////////////////////// End of File.
