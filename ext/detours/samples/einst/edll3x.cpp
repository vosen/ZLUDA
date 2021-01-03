//////////////////////////////////////////////////////////////////////////////
//
//  Detours Test Program (edll3x.cpp of einst.exe/edll3x.dll)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
#include <stdio.h>
#include <windows.h>
#include <detours.h>

//////////////////////////////////////////////////////////////////// DLL Stuff
//
struct CPrivateStuffPart1
{
    DETOUR_SECTION_RECORD   header;
    CHAR                    szMessage[48];
};

struct CPrivateStuffPart2
{
    DETOUR_SECTION_RECORD   header;
    CHAR                    szMessage[64];
};

struct CPrivateStuff
{
    DETOUR_SECTION_HEADER   header;
    CPrivateStuffPart1      record1;
    CPrivateStuffPart2      record2;
};

#pragma data_seg(".detour")

static CPrivateStuff private_stuff = {
    DETOUR_SECTION_HEADER_DECLARE(sizeof(CPrivateStuff)),
    {
        {
            sizeof(CPrivateStuffPart1),
            0,
            { /* d9ab8a41-f4cc-11d1-b6d7-006097b010e3 */
                0xd9ab8a41,
                0xf4cc,
                0x11d1,
                {0xb6, 0xd7, 0x00, 0x60, 0x97, 0xb0, 0x10, 0xe3}
            }
        },
        "The Third DLL Part One!"
    },
    {
        {
            sizeof(CPrivateStuffPart2),
            0,
            { /* d9ab8a40-f4cc-11d1-b6d7-006097b010e3 */
                0xd9ab8a40,
                0xf4cc,
                0x11d1,
                {0xb6, 0xd7, 0x00, 0x60, 0x97, 0xb0, 0x10, 0xe3}
            }
        },
        "The Third DLL Part Two!"
    }
};
#pragma data_seg()

__declspec(dllexport) VOID WINAPI EDll3Function(VOID)
{
    return;
}

__declspec(dllexport) ULONG WINAPI
DllMain(HINSTANCE hInstance, DWORD dwReason, PVOID lpReserved)
{
    (void)hInstance;
    (void)dwReason;
    (void)lpReserved;

    return TRUE;
}

///////////////////////////////////////////////////////////////// End of File.
