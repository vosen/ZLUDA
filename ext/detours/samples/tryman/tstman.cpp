//////////////////////////////////////////////////////////////////////////////
//
//  Detours Test Program (tstman.cpp of tstman.dll)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
//  This DLL doesn't detour any APIs, but it does enumerate the modules
//  loaded in a process and look at their size and processor target.
//

#include <stdio.h>
#include <windows.h>
#pragma warning(push)
#if _MSC_VER > 1400
#pragma warning(disable:6102 6103) // /analyze warnings
#endif
#include <strsafe.h>
#pragma warning(pop)
#include "detours.h"

static HMODULE s_hInst = NULL;
static CHAR s_szDllPath[MAX_PATH];

static int (WINAPI * TrueEntryPoint)(VOID) = NULL;
static int (WINAPI * RawEntryPoint)(VOID) = NULL;

BOOL (WINAPI * Real_CreateProcessA)(LPCSTR a0,
                                    LPSTR a1,
                                    LPSECURITY_ATTRIBUTES a2,
                                    LPSECURITY_ATTRIBUTES a3,
                                    BOOL a4,
                                    DWORD a5,
                                    LPVOID a6,
                                    LPCSTR a7,
                                    struct _STARTUPINFOA* a8,
                                    LPPROCESS_INFORMATION a9)
    = CreateProcessA;

BOOL (WINAPI * Real_CreateProcessW)(LPCWSTR a0,
                                    LPWSTR a1,
                                    LPSECURITY_ATTRIBUTES a2,
                                    LPSECURITY_ATTRIBUTES a3,
                                    BOOL a4,
                                    DWORD a5,
                                    LPVOID a6,
                                    LPCWSTR a7,
                                    struct _STARTUPINFOW* a8,
                                    LPPROCESS_INFORMATION a9)
    = CreateProcessW;


BOOL WINAPI Mine_CreateProcessA(LPCSTR lpApplicationName,
                                LPSTR lpCommandLine,
                                LPSECURITY_ATTRIBUTES lpProcessAttributes,
                                LPSECURITY_ATTRIBUTES lpThreadAttributes,
                                BOOL bInheritHandles,
                                DWORD dwCreationFlags,
                                LPVOID lpEnvironment,
                                LPCSTR lpCurrentDirectory,
                                LPSTARTUPINFOA lpStartupInfo,
                                LPPROCESS_INFORMATION lpProcessInformation)
{
    BOOL rv = 0;
    __try {
        rv = DetourCreateProcessWithDllExA(lpApplicationName,
                                           lpCommandLine,
                                           lpProcessAttributes,
                                           lpThreadAttributes,
                                           bInheritHandles,
                                           dwCreationFlags,
                                           lpEnvironment,
                                           lpCurrentDirectory,
                                           lpStartupInfo,
                                           lpProcessInformation,
                                           s_szDllPath,
                                           Real_CreateProcessA);
    } __finally {
    };
    return rv;
}

BOOL WINAPI Mine_CreateProcessW(LPCWSTR lpApplicationName,
                                LPWSTR lpCommandLine,
                                LPSECURITY_ATTRIBUTES lpProcessAttributes,
                                LPSECURITY_ATTRIBUTES lpThreadAttributes,
                                BOOL bInheritHandles,
                                DWORD dwCreationFlags,
                                LPVOID lpEnvironment,
                                LPCWSTR lpCurrentDirectory,
                                LPSTARTUPINFOW lpStartupInfo,
                                LPPROCESS_INFORMATION lpProcessInformation)
{
    BOOL rv = 0;
    __try {
        rv = DetourCreateProcessWithDllExW(lpApplicationName,
                                           lpCommandLine,
                                           lpProcessAttributes,
                                           lpThreadAttributes,
                                           bInheritHandles,
                                           dwCreationFlags,
                                           lpEnvironment,
                                           lpCurrentDirectory,
                                           lpStartupInfo,
                                           lpProcessInformation,
                                           s_szDllPath,
                                           Real_CreateProcessW);
    } __finally {
    };
    return rv;
}

void DumpModuleInfo(HMODULE hModule)
{
    PBYTE pbModule = (PBYTE)hModule;
    PIMAGE_DOS_HEADER pidh = (PIMAGE_DOS_HEADER)pbModule;
    PIMAGE_NT_HEADERS pinh = (PIMAGE_NT_HEADERS)(pbModule + pidh->e_lfanew);
    CHAR szFile[MAX_PATH] = "";

    GetModuleFileNameA(hModule, szFile, sizeof(szFile));

    CHAR szMagic[64];
    CHAR szMachine[64];
    CHAR szClr[64];

    PIMAGE_DATA_DIRECTORY pdir
        = (pinh->OptionalHeader.Magic == IMAGE_NT_OPTIONAL_HDR32_MAGIC)
        ? ((PIMAGE_NT_HEADERS32)pinh)->OptionalHeader.DataDirectory
        : ((PIMAGE_NT_HEADERS64)pinh)->OptionalHeader.DataDirectory;

    if (pdir[IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR].VirtualAddress != 0 &&
        pdir[IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR].Size != 0) {

        PDETOUR_CLR_HEADER pch
            = (PDETOUR_CLR_HEADER)
            (pbModule + pdir[IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR].VirtualAddress);

        if ((pch->Flags & 0x3) == 0x0) {
            StringCchPrintfA(szClr, ARRAYSIZE(szClr), "clr   ");   // 32- or 64-bit.
        }
        else if ((pch->Flags & 0x3) == 0x1) {
            StringCchPrintfA(szClr, ARRAYSIZE(szClr), "clri  ");   // IL-Only, 32- or 64-bit.
        }
        else if ((pch->Flags & 0x3) == 0x2) {
            StringCchPrintfA(szClr, ARRAYSIZE(szClr), "clr32 ");   // must be 32-bit.
        }
        else if ((pch->Flags & 0x3) == 0x3) {
            StringCchPrintfA(szClr, ARRAYSIZE(szClr), "clr32i");   // IL-Only, must be 32-bit.
        }
    }
    else {
        StringCchPrintfA(szClr, ARRAYSIZE(szClr), "      ");
    }

    if (pinh->OptionalHeader.Magic == 0x10b) {
        StringCchPrintfA(szMagic, ARRAYSIZE(szMagic), "32");
    }
    else if (pinh->OptionalHeader.Magic == 0x20b) {
        StringCchPrintfA(szMagic, ARRAYSIZE(szMagic), "64");
    }
    else {
        StringCchPrintfA(szMagic, ARRAYSIZE(szMagic), "??");
    }

    if (pinh->FileHeader.Machine == 0x8664) {
        StringCchPrintfA(szMachine, ARRAYSIZE(szMachine), "x64", pinh->FileHeader.Machine);
    }
    else if (pinh->FileHeader.Machine == 0x014c) {
        StringCchPrintfA(szMachine, ARRAYSIZE(szMachine), "x86", pinh->FileHeader.Machine);
    }
    else if (pinh->FileHeader.Machine == 0x0200) {
        StringCchPrintfA(szMachine, ARRAYSIZE(szMachine), "i64", pinh->FileHeader.Machine);
    }
    else if (pinh->FileHeader.Machine == 0x01c0) {
        StringCchPrintfA(szMachine, ARRAYSIZE(szMachine), "arm", pinh->FileHeader.Machine);
    }
    else {
        StringCchPrintfA(szMachine, ARRAYSIZE(szMachine), "%04x", pinh->FileHeader.Machine);
        DWORD dwSize = DetourGetSizeOfPayloads(hModule);
        if (dwSize > 0) {
            StringCchPrintfA(szMachine, ARRAYSIZE(szMachine), "     ");
            StringCchPrintfA(szFile, ARRAYSIZE(szFile), "-- %d byte payload.", dwSize);
        }
    }

    printf("%16I64x: %s %s %s %s\n", (ULONG64)hModule, szMagic, szMachine, szClr, szFile);
}

void DumpMemory(PBYTE pbData, DWORD cbData)
{
    for (DWORD i = 0; i < cbData; i += 16) {
        printf("  %p:", pbData + i);
        for (DWORD j = 0; j < 16; j++) {
            if (i + j < cbData) {
                printf("%02x", pbData[i+j]);
            }
            else {
                printf("  ");
            }
        }
        printf(" ");
        for (DWORD j = 0; j < 16; j++) {
            if (i + j < cbData) {
                if ( pbData[i+j] >= ' ' && pbData[i+j] < 127) {
                    printf("%c", pbData[i+j]);
                }
                else {
                    printf(".");
                }
            }
            else {
                printf(" ");
            }
        }
        printf("\n");
    }
}

int WINAPI Test3264(int arg)
{
    return arg + 1;
}

int WINAPI TestEntryPoint(VOID)
{
#if DETOURS_64BIT
    printf("----------------: ");
#else
    printf("--------: ");
#endif

    printf("Calling EntryPoint() from detour.\n");
    fflush(stdout);

    return TrueEntryPoint();
}

BOOL WINAPI DllMain(HINSTANCE hinst, DWORD dwReason, LPVOID reserved)
{
    (void)hinst;
    (void)reserved;

    if (DetourIsHelperProcess()) {
        return TRUE;
    }

    if (dwReason == DLL_PROCESS_ATTACH) {
        DetourRestoreAfterWith();

        s_hInst = hinst;
        GetModuleFileNameA(s_hInst, s_szDllPath, ARRAYSIZE(s_szDllPath));

#if DETOURS_64BIT
        printf("----------------: ");
#else
        printf("--------: ");
#endif

        SYSTEM_INFO si;
        GetSystemInfo(&si);

        if (si.wProcessorArchitecture == 9) {
            printf("x64 Processor\n");
        }
        else if (si.wProcessorArchitecture == 0) {
            printf("x86 Processor\n");
        }
        else if (si.wProcessorArchitecture == 6) {
            printf("ia64 Processor\n");
        }
        else {
            printf("%04x Processor\n", si.wProcessorArchitecture);
        }

        HMODULE hSelf = GetModuleHandle(NULL);
        HMODULE hTest = (HMODULE)DetourGetContainingModule(DetourCodeFromPointer(Test3264, NULL));
        HMODULE hKern = (HMODULE)DetourGetContainingModule(DetourCodeFromPointer(CreateProcessW, NULL));

        DumpModuleInfo(hSelf);
        DumpModuleInfo(hTest);
        DumpModuleInfo(hKern);
        for (HINSTANCE hInst = NULL; (hInst = DetourEnumerateModules(hInst)) != NULL;) {
            if (hInst == hSelf || hInst == hTest || hInst == hKern) {
                continue;
            }

            DumpModuleInfo(hInst);
        }
        fflush(stdout);

        TrueEntryPoint = (int (WINAPI *)(VOID))DetourGetEntryPoint(NULL);
        RawEntryPoint = TrueEntryPoint;

        DetourTransactionBegin();
        DetourUpdateThread(GetCurrentThread());
        DetourAttach(&(PVOID&)TrueEntryPoint, TestEntryPoint);
        DetourAttach(&(PVOID&)Real_CreateProcessA, Mine_CreateProcessA);
        DetourAttach(&(PVOID&)Real_CreateProcessW, Mine_CreateProcessW);
        LONG error = DetourTransactionCommit();

#if DETOURS_64BIT
        printf("----------------: ");
#else
        printf("--------: ");
#endif

        if (error == NO_ERROR) {
            printf("Detoured EntryPoint().\n");
        }
        else {
            printf("Error detouring EntryPoint(): %ld (@ %p)\n", error, RawEntryPoint);
            __debugbreak();
        }
    }
    else if (dwReason == DLL_PROCESS_DETACH) {

        DetourTransactionBegin();
        DetourUpdateThread(GetCurrentThread());
        DetourDetach(&(PVOID&)TrueEntryPoint, TestEntryPoint);
        DetourDetach(&(PVOID&)Real_CreateProcessA, Mine_CreateProcessA);
        DetourDetach(&(PVOID&)Real_CreateProcessW, Mine_CreateProcessW);
        LONG error = DetourTransactionCommit();

        if (error != NO_ERROR) {
            printf("Error detach detours failed: %ld\n", error);
        }
    }

    return TRUE;
}

//
///////////////////////////////////////////////////////////////// End of File.
