//////////////////////////////////////////////////////////////////////////////
//
//  Detours Test Program (talloc.cpp of talloc.exe)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
#define PSAPI_VERSION 2
#include <stdio.h>
#include <stdlib.h>
#include <windows.h>
#pragma warning(push)
#if _MSC_VER > 1400
#pragma warning(disable:6102 6103) // /analyze warnings
#endif
#include <strsafe.h>
#pragma warning(pop)
#include <psapi.h>
#include <detours.h>

//////////////////////////////////////////////////////////////////////////////

void TypeToString(DWORD Type, char *pszBuffer, size_t cBuffer)
{
    if (Type == MEM_IMAGE) {
        StringCchPrintfA(pszBuffer, cBuffer, "img");
    }
    else if (Type == MEM_MAPPED) {
        StringCchPrintfA(pszBuffer, cBuffer, "map");
    }
    else if (Type == MEM_PRIVATE) {
        StringCchPrintfA(pszBuffer, cBuffer, "pri");
    }
    else if (Type == 0) {
        StringCchPrintfA(pszBuffer, cBuffer, "   ");
    }
    else {
        StringCchPrintfA(pszBuffer, cBuffer, "%x", Type);
    }
}

void StateToString(DWORD State, char *pszBuffer, size_t cBuffer)
{
    if (State == MEM_COMMIT) {
        StringCchPrintfA(pszBuffer, cBuffer, "com");
    }
    else if (State == MEM_FREE) {
        StringCchPrintfA(pszBuffer, cBuffer, "fre");
    }
    else if (State == MEM_RESERVE) {
        StringCchPrintfA(pszBuffer, cBuffer, "res");
    }
    else {
        StringCchPrintfA(pszBuffer, cBuffer, "%x", State);
    }
}

void ProtectToString(DWORD Protect, char *pszBuffer, size_t cBuffer)
{
    if (Protect == 0) {
        StringCchPrintfA(pszBuffer, cBuffer, "");
    }
    else if (Protect == PAGE_EXECUTE) {
        StringCchPrintfA(pszBuffer, cBuffer, "--x");
    }
    else if (Protect == PAGE_EXECUTE_READ) {
        StringCchPrintfA(pszBuffer, cBuffer, "r-x");
    }
    else if (Protect == PAGE_EXECUTE_READWRITE) {
        StringCchPrintfA(pszBuffer, cBuffer, "rwx");
    }
    else if (Protect == PAGE_EXECUTE_WRITECOPY) {
        StringCchPrintfA(pszBuffer, cBuffer, "rcx");
    }
    else if (Protect == PAGE_NOACCESS) {
        StringCchPrintfA(pszBuffer, cBuffer, "---");
    }
    else if (Protect == PAGE_READONLY) {
        StringCchPrintfA(pszBuffer, cBuffer, "r--");
    }
    else if (Protect == PAGE_READWRITE) {
        StringCchPrintfA(pszBuffer, cBuffer, "rw-");
    }
    else if (Protect == PAGE_WRITECOPY) {
        StringCchPrintfA(pszBuffer, cBuffer, "rc-");
    }
    else if (Protect == (PAGE_GUARD | PAGE_EXECUTE)) {
        StringCchPrintfA(pszBuffer, cBuffer, "g--x");
    }
    else if (Protect == (PAGE_GUARD | PAGE_EXECUTE_READ)) {
        StringCchPrintfA(pszBuffer, cBuffer, "gr-x");
    }
    else if (Protect == (PAGE_GUARD | PAGE_EXECUTE_READWRITE)) {
        StringCchPrintfA(pszBuffer, cBuffer, "grwx");
    }
    else if (Protect == (PAGE_GUARD | PAGE_EXECUTE_WRITECOPY)) {
        StringCchPrintfA(pszBuffer, cBuffer, "grcx");
    }
    else if (Protect == (PAGE_GUARD | PAGE_NOACCESS)) {
        StringCchPrintfA(pszBuffer, cBuffer, "g---");
    }
    else if (Protect == (PAGE_GUARD | PAGE_READONLY)) {
        StringCchPrintfA(pszBuffer, cBuffer, "gr--");
    }
    else if (Protect == (PAGE_GUARD | PAGE_READWRITE)) {
        StringCchPrintfA(pszBuffer, cBuffer, "grw-");
    }
    else if (Protect == (PAGE_GUARD | PAGE_WRITECOPY)) {
        StringCchPrintfA(pszBuffer, cBuffer, "grc-");
    }
    else {
        StringCchPrintfA(pszBuffer, cBuffer, "%x", Protect);
    }
}

ULONG PadToPage(ULONG Size)
{
    return (Size & 0xfff)
        ? Size + 0x1000 - (Size & 0xfff)
        : Size;
}

size_t NextAt(size_t start)
{
    size_t next = start;

    for (;;) {
        MEMORY_BASIC_INFORMATION mbi;

        ZeroMemory(&mbi, sizeof(mbi));
        if (VirtualQuery((PVOID)next, &mbi, sizeof(mbi)) == 0) {
            break;
        }
        if ((mbi.RegionSize & 0xfff) == 0xfff) {
            break;
        }

        if ((size_t)mbi.AllocationBase != start) {
            break;
        }

        next = (size_t)mbi.BaseAddress + mbi.RegionSize;
    }
    return next;
}

size_t RoundUpRegion(size_t value)
{
    size_t diff = value & 0xffff;
    return (diff != 0) ? value + 0x10000 - diff : value;
}

VOID DumpProcessHeaders()
{
    printf("  %12s %12s: %3s %3s %4s %3s : %8s\n",
           "Address", "Size", "Typ", "Sta", "Prot", "Ini", "Contents");
    printf("  %12s %12s: %3s %3s %4s %3s : %8s\n",
           "------------", "------------", "---", "---", "----", "---", "-----------------");
}

BOOL DumpProcess(UINT64 lo64, UINT64 hi64)
{
#ifdef _WIN64
    ULONG_PTR lo = lo64;
    ULONG_PTR hi = hi64;
#else
    ULONG_PTR lo = (size_t)(lo64 >> 4);
    ULONG_PTR hi = (size_t)(hi64 >> 4);
#endif

    size_t base;
    size_t next;

    MEMORY_BASIC_INFORMATION mbi;

    for (next = lo; next < hi;) {
        base = next;
        ZeroMemory(&mbi, sizeof(mbi));
        if (VirtualQuery((PVOID)base, &mbi, sizeof(mbi)) == 0) {
            break;
        }
        if ((mbi.RegionSize & 0xfff) == 0xfff) {
            break;
        }

        if ((size_t)mbi.BaseAddress < lo) {
            base = (size_t)mbi.BaseAddress;
        }

        size_t size = ((size_t)mbi.BaseAddress + mbi.RegionSize) - base;
        next = (size_t)mbi.BaseAddress + mbi.RegionSize;

        CHAR szType[16];
        TypeToString(mbi.Type, szType, ARRAYSIZE(szType));
        CHAR szState[16];
        StateToString(mbi.State, szState, ARRAYSIZE(szState));
        CHAR szProtect[16];
        ProtectToString(mbi.Protect, szProtect, ARRAYSIZE(szProtect));
        CHAR szAllocProtect[16];
        ProtectToString(mbi.AllocationProtect, szAllocProtect, ARRAYSIZE(szAllocProtect));

        CHAR szFile[MAX_PATH];
        szFile[0] = '\0';
        DWORD cb = 0;
        PCHAR pszFile = szFile;

        if (base == (size_t)mbi.AllocationBase) {
            next = NextAt(base);

            cb = GetMappedFileNameA(GetCurrentProcess(),
                                    mbi.AllocationBase, szFile, ARRAYSIZE(szFile));
            if (cb > 0) {
                for (DWORD c = 0; c < cb; c++) {
                    szFile[c] = (char)toupper(szFile[c]);
                }
                szFile[cb] = '\0';
            }
            else {
                szFile[0] = '\0';
            }
            if ((pszFile = strrchr(szFile, '\\')) == NULL) {
                pszFile = szFile;
            }
            else {
                pszFile++;
            }
        }

        printf("%c %12zx %12zx: %3s %3s %4s %3s : %s\n",
               " *"[base == (size_t)mbi.AllocationBase],
               base,
               size,
               szType,
               szState,
               szProtect,
               szAllocProtect,
               pszFile);
    }
    return TRUE;
}

//////////////////////////////////////////////////////////////////////////////

__declspec(dllimport) DWORD WINAPI Dll1Function(DWORD Value);
__declspec(dllimport) DWORD WINAPI Dll2Function(DWORD Value);
__declspec(dllimport) DWORD WINAPI Dll3Function(DWORD Value);
__declspec(dllimport) DWORD WINAPI Dll4Function(DWORD Value);
__declspec(dllimport) DWORD WINAPI Dll5Function(DWORD Value);
__declspec(dllimport) DWORD WINAPI Dll6Function(DWORD Value);
__declspec(dllimport) DWORD WINAPI Dll7Function(DWORD Value);
__declspec(dllimport) DWORD WINAPI Dll8Function(DWORD Value);
__declspec(dllimport) DWORD WINAPI Dll9Function(DWORD Value);

static LONG dwCountDll1 = 0;
static LONG dwCountDll2 = 0;
static LONG dwCountDll3 = 0;
static LONG dwCountDll4 = 0;
static LONG dwCountDll5 = 0;
static LONG dwCountDll6 = 0;
static LONG dwCountDll7 = 0;
static LONG dwCountDll8 = 0;
static LONG dwCountDll9 = 0;

static DWORD (WINAPI * TrueDll1Function)(DWORD Value) = Dll1Function;
static DWORD (WINAPI * TrueDll2Function)(DWORD Value) = Dll2Function;
static DWORD (WINAPI * TrueDll3Function)(DWORD Value) = Dll3Function;
static DWORD (WINAPI * TrueDll4Function)(DWORD Value) = Dll4Function;
static DWORD (WINAPI * TrueDll5Function)(DWORD Value) = Dll5Function;
static DWORD (WINAPI * TrueDll6Function)(DWORD Value) = Dll6Function;
static DWORD (WINAPI * TrueDll7Function)(DWORD Value) = Dll7Function;
static DWORD (WINAPI * TrueDll8Function)(DWORD Value) = Dll8Function;
static DWORD (WINAPI * TrueDll9Function)(DWORD Value) = Dll9Function;

DWORD WINAPI MineDll1Function(DWORD Value)
{
    Value = TrueDll1Function(Value);
    InterlockedIncrement(&dwCountDll1);

    return Value;
}

DWORD WINAPI MineDll2Function(DWORD Value)
{
    Value = TrueDll2Function(Value);
    InterlockedIncrement(&dwCountDll2);

    return Value;
}

DWORD WINAPI MineDll3Function(DWORD Value)
{
    Value = TrueDll3Function(Value);
    InterlockedIncrement(&dwCountDll3);

    return Value;
}

DWORD WINAPI MineDll4Function(DWORD Value)
{
    Value = TrueDll4Function(Value);
    InterlockedIncrement(&dwCountDll4);

    return Value;
}

DWORD WINAPI MineDll5Function(DWORD Value)
{
    Value = TrueDll5Function(Value);
    InterlockedIncrement(&dwCountDll5);

    return Value;
}

DWORD WINAPI MineDll6Function(DWORD Value)
{
    Value = TrueDll6Function(Value);
    InterlockedIncrement(&dwCountDll6);

    return Value;
}

DWORD WINAPI MineDll7Function(DWORD Value)
{
    Value = TrueDll7Function(Value);
    InterlockedIncrement(&dwCountDll7);

    return Value;
}

DWORD WINAPI MineDll8Function(DWORD Value)
{
    Value = TrueDll8Function(Value);
    InterlockedIncrement(&dwCountDll8);

    return Value;
}

DWORD WINAPI MineDll9Function(DWORD Value)
{
    Value = TrueDll9Function(Value);
    InterlockedIncrement(&dwCountDll9);

    return Value;
}

void Reserve(ULONG_PTR addr, ULONG_PTR size)
{
    PVOID mem = VirtualAlloc((PVOID)addr, size, MEM_RESERVE, PAGE_NOACCESS);
    if (mem != (PVOID)addr) {
        printf("*** Reservation failed: %p != %p\n", mem, (PVOID)addr);
    }
}

int WINAPI WinMain(HINSTANCE hinst, HINSTANCE hprev, LPSTR lpszCmdLine, int nCmdShow)
{
    (void)hinst;
    (void)hprev;
    (void)lpszCmdLine;
    (void)nCmdShow;
    DWORD error = NO_ERROR;

    size_t Dll1 = (size_t)LoadLibraryA("tdll1x" DETOURS_STRINGIFY(DETOURS_BITS) ".dll");
    size_t Dll2 = (size_t)LoadLibraryA("tdll2x" DETOURS_STRINGIFY(DETOURS_BITS) ".dll");
    size_t Dll3 = (size_t)LoadLibraryA("tdll3x" DETOURS_STRINGIFY(DETOURS_BITS) ".dll");
    size_t Dll4 = (size_t)LoadLibraryA("tdll4x" DETOURS_STRINGIFY(DETOURS_BITS) ".dll");
    size_t Dll5 = (size_t)LoadLibraryA("tdll5x" DETOURS_STRINGIFY(DETOURS_BITS) ".dll");
    size_t Dll6 = (size_t)LoadLibraryA("tdll6x" DETOURS_STRINGIFY(DETOURS_BITS) ".dll");
    size_t Dll7 = (size_t)LoadLibraryA("tdll7x" DETOURS_STRINGIFY(DETOURS_BITS) ".dll");
    size_t Dll8 = (size_t)LoadLibraryA("tdll8x" DETOURS_STRINGIFY(DETOURS_BITS) ".dll");
    size_t Dll9 = (size_t)LoadLibraryA("tdll9x" DETOURS_STRINGIFY(DETOURS_BITS) ".dll");

    size_t DllEnd = RoundUpRegion(NextAt(Dll1));
    ULONG_PTR DllSize = (DllEnd - Dll1);

    (void)Dll6;
    (void)Dll7;
    (void)Dll8;

    // Force allocation below moving lower.
    Reserve(Dll1 - 0x40000000, 0x40000000);
    Reserve(Dll1 - 0x40100000, 0x00100000);
    Reserve(Dll1 - 0x40110000, 0x00010000);
    Reserve(Dll1 - 0x40120000, 0x00001000);
    Reserve(Dll1 + DllSize, 0x80000000 - DllSize);

    // Force allocation above moving higher.
    Reserve(Dll2 - 0x80000000, 0x80000000);
    Reserve(Dll2 + DllSize, 0x40000000);
    Reserve(Dll2 + 0x40000000 + DllSize, 0x00100000);
    Reserve(Dll2 + 0x40100000 + DllSize, 0x00010000);
    Reserve(Dll2 + 0x40110000 + DllSize, 0x00001000);

    // Force allocation below moving higher.
    Reserve(Dll3 - 0x80000000, 0x40000000);
    Reserve(Dll3 - 0x40000000, 0x00100000);
    Reserve(Dll3 - 0x3ff00000, 0x00010000);
    Reserve(Dll3 - 0x3fef0000, 0x00001000);
    Reserve(Dll3 + DllSize, 0x80000000 - DllSize);

    // Force allocation above moving lower.
    Reserve(Dll4 - 0x80000000, 0x80000000);
    Reserve(Dll4 + 0x40000000, 0x40000000);
    Reserve(Dll4 + 0x3ff00000, 0x00100000);
    Reserve(Dll4 + 0x3fef0000, 0x00010000);
    Reserve(Dll4 + 0x3fee0000, 0x00001000);

    // Force allocation above and below.
    Reserve(Dll5 - 0x7ff00000, 0x7ff00000);
    Reserve(Dll9 + DllSize, 0x7fe00000);

    DetourTransactionBegin();
    DetourUpdateThread(GetCurrentThread());
    DetourAttach(&(PVOID&)TrueDll1Function, MineDll1Function);
    error = DetourTransactionCommit();
    if (error != NO_ERROR) {
      failed:
        printf("talloc.exe: Error detouring functions: %ld\n", error);
        exit(1);
    }

    DetourTransactionBegin();
    DetourUpdateThread(GetCurrentThread());
    DetourAttach(&(PVOID&)TrueDll2Function, MineDll2Function);
    error = DetourTransactionCommit();
    if (error != NO_ERROR) {
        goto failed;
    }

    DetourTransactionBegin();
    DetourUpdateThread(GetCurrentThread());
    DetourAttach(&(PVOID&)TrueDll3Function, MineDll3Function);
    error = DetourTransactionCommit();
    if (error != NO_ERROR) {
        goto failed;
    }

    DetourTransactionBegin();
    DetourUpdateThread(GetCurrentThread());
    DetourAttach(&(PVOID&)TrueDll4Function, MineDll4Function);
    error = DetourTransactionCommit();
    if (error != NO_ERROR) {
        goto failed;
    }

    DetourTransactionBegin();
    DetourUpdateThread(GetCurrentThread());
    DetourAttach(&(PVOID&)TrueDll5Function, MineDll5Function);
    error = DetourTransactionCommit();
    if (error != NO_ERROR) {
        goto failed;
    }

    DetourTransactionBegin();
    DetourUpdateThread(GetCurrentThread());
    DetourAttach(&(PVOID&)TrueDll6Function, MineDll6Function);
    error = DetourTransactionCommit();
    if (error != NO_ERROR) {
        goto failed;
    }

    DetourTransactionBegin();
    DetourUpdateThread(GetCurrentThread());
    DetourAttach(&(PVOID&)TrueDll7Function, MineDll7Function);
    error = DetourTransactionCommit();
    if (error != NO_ERROR) {
        goto failed;
    }

    DetourTransactionBegin();
    DetourUpdateThread(GetCurrentThread());
    DetourAttach(&(PVOID&)TrueDll8Function, MineDll8Function);
    error = DetourTransactionCommit();
    if (error != NO_ERROR) {
        goto failed;
    }

    DetourTransactionBegin();
    DetourUpdateThread(GetCurrentThread());
    DetourAttach(&(PVOID&)TrueDll9Function, MineDll9Function);
    error = DetourTransactionCommit();
    if (error != NO_ERROR) {
        goto failed;
    }

    printf("talloc.exe: Detoured functions.\n");
    printf("\n");

    DumpProcessHeaders();
    printf("%-47s %17zx\n", "Exe:", (size_t)GetModuleHandleW(NULL));
    DumpProcess(0x100000000, 0x200000000);
    printf("%-47s %17zx\n", "Dll1:", Dll1);
    DumpProcess(0x200000000, 0x300000000);
    printf("%-47s %17zx\n", "Dll2:", Dll2);
    DumpProcess(0x300000000, 0x400000000);
    printf("%-47s %17zx\n", "Dll3:", Dll3);
    DumpProcess(0x400000000, 0x500000000);
    printf("%-47s %17zx\n", "Dll4:", Dll4);
    DumpProcess(0x500000000, 0x600000000);
    printf("%-47s %17zx\n", "Dll5:", Dll5);
    DumpProcess(0x600000000, 0x700000000);
    fflush(stdout);

    Dll1Function(1);
    Dll2Function(2);
    Dll2Function(3);
    Dll3Function(4);
    Dll3Function(5);
    Dll3Function(6);
    Dll4Function(7);
    Dll5Function(8);
    Dll6Function(9);
    Dll7Function(10);
    Dll8Function(10);
    Dll9Function(10);

    DetourTransactionBegin();
    DetourUpdateThread(GetCurrentThread());
    DetourDetach(&(PVOID&)TrueDll1Function, MineDll1Function);
    DetourDetach(&(PVOID&)TrueDll2Function, MineDll2Function);
    DetourDetach(&(PVOID&)TrueDll3Function, MineDll3Function);
    DetourDetach(&(PVOID&)TrueDll4Function, MineDll4Function);
    DetourDetach(&(PVOID&)TrueDll5Function, MineDll5Function);
    DetourDetach(&(PVOID&)TrueDll6Function, MineDll6Function);
    DetourDetach(&(PVOID&)TrueDll7Function, MineDll7Function);
    DetourDetach(&(PVOID&)TrueDll8Function, MineDll8Function);
    DetourDetach(&(PVOID&)TrueDll9Function, MineDll9Function);
    error = DetourTransactionCommit();
    if (error != NO_ERROR) {
        goto failed;
    }

    printf("\n");
    printf("talloc.exe: %ld calls to Dll1Function\n", dwCountDll1);
    fflush(stdout);

    return 0;
}

//
///////////////////////////////////////////////////////////////// End of File.
