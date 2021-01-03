//////////////////////////////////////////////////////////////////////////////
//
//  Detour Test Program (symtest.cpp of symtest.exe)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//

#include <windows.h>
#include <stdio.h>
#pragma warning(push)
#if _MSC_VER > 1400
#pragma warning(disable:6102 6103) // /analyze warnings
#endif
#include <strsafe.h>
#pragma warning(pop)
#include <detours.h>
#include "target.h"

#if (_MSC_VER < 1299)
#include <imagehlp.h>
typedef IMAGEHLP_MODULE IMAGEHLP_MODULE64;
typedef PIMAGEHLP_MODULE PIMAGEHLP_MODULE64;
typedef IMAGEHLP_SYMBOL SYMBOL_INFO;
typedef PIMAGEHLP_SYMBOL PSYMBOL_INFO;
#else
#pragma warning(push)
#pragma warning(disable:4091) // empty typedef
#include <dbghelp.h>
#pragma warning(pop)
#endif

//////////////////////////////////////////////////////////////////////////////
//
typedef LPAPI_VERSION (NTAPI *PF_ImagehlpApiVersionEx)(LPAPI_VERSION AppVersion);

typedef BOOL (NTAPI *PF_SymInitialize)(IN HANDLE hProcess,
                                       IN LPCSTR UserSearchPath,
                                       IN BOOL fInvadeProcess);
typedef DWORD (NTAPI *PF_SymSetOptions)(IN DWORD SymOptions);
typedef DWORD (NTAPI *PF_SymGetOptions)(VOID);
typedef DWORD64 (NTAPI *PF_SymLoadModule64)(IN HANDLE hProcess,
                                            IN HANDLE hFile,
                                            IN PSTR ImageName,
                                            IN PSTR ModuleName,
                                            IN DWORD64 BaseOfDll,
                                            IN DWORD SizeOfDll);
typedef BOOL (NTAPI *PF_SymGetModuleInfo64)(IN HANDLE hProcess,
                                            IN DWORD64 qwAddr,
                                            OUT PIMAGEHLP_MODULE64 ModuleInfo);
typedef BOOL (NTAPI *PF_SymFromName)(IN HANDLE hProcess,
                                     IN LPSTR Name,
                                     OUT PSYMBOL_INFO Symbol);
#if (_MSC_VER < 1299)
typedef BOOL (NTAPI *PF_SymRegisterCallback64)();
typedef BOOL (NTAPI *PF_SymEnumerateModules64)();
typedef BOOL (NTAPI *PF_SymEnumSymbols)();
#else
typedef BOOL (NTAPI *PF_SymRegisterCallback64)(IN HANDLE hProcess,
                                               IN PSYMBOL_REGISTERED_CALLBACK64
                                               CallbackFunction,
                                               IN ULONG64 UserContext);
typedef BOOL (NTAPI *PF_SymEnumerateModules64)(IN HANDLE hProcess,
                                               IN PSYM_ENUMMODULES_CALLBACK64
                                               EnumModulesCallback,
                                               IN PVOID UserContext);
typedef BOOL (NTAPI *PF_SymEnumSymbols)(IN HANDLE hProcess,
                                        IN ULONG64 BaseOfDll,
                                        IN PCSTR Mask,
                                        IN PSYM_ENUMERATESYMBOLS_CALLBACK
                                        EnumSymbolsCallback,
                                        IN PVOID UserContext);
#endif

PF_ImagehlpApiVersionEx     pfImagehlpApiVersionEx = NULL;
PF_SymInitialize            pfSymInitialize = NULL;
PF_SymSetOptions            pfSymSetOptions = NULL;
PF_SymGetOptions            pfSymGetOptions = NULL;
PF_SymLoadModule64          pfSymLoadModule64 = NULL;
PF_SymGetModuleInfo64       pfSymGetModuleInfo64 = NULL;
PF_SymFromName              pfSymFromName = NULL;
PF_SymRegisterCallback64    pfSymRegisterCallback64 = NULL;
PF_SymEnumerateModules64    pfSymEnumerateModules64 = NULL;
PF_SymEnumSymbols           pfSymEnumSymbols = NULL;

//////////////////////////////////////////////////////////////////////////////
//

#if (_MSC_VER > 1299)
static BOOL WINAPI SymEnumerateCallback(
                                        PCSTR pszModule,
                                        DWORD64 base,
                                        PVOID pvUserContext)
{
    (void)pvUserContext;
    printf("  %p: %s\n", (PVOID)base, pszModule);
    return TRUE;
}

static int nSymbolCount = 0;
static BOOL WINAPI SymEnumerateSymbols(PSYMBOL_INFO pSym,
                                       ULONG size,
                                       PVOID pvUserContext)
{
    (void)size;
    (void)pvUserContext;
    if (strstr(pSym->Name, "Target") != NULL ||
        strstr(pSym->Name, "Hidden") != NULL) {
        printf("  %p: %s\n", (PVOID)pSym->Address, pSym->Name);
        nSymbolCount++;
    }
    else if (nSymbolCount < 5) {
        printf("  %p: %s\n", (PVOID)pSym->Address, pSym->Name);
        nSymbolCount++;
    }
    return TRUE;
}

static void truncate(PCHAR data)
{
    size_t len = strlen(data);
    if (len > 0 && data[len-1] == '\r') {
        data[--len] = '\0';
    }
    if (len > 0 && data[len-1] == '\n') {
        data[--len] = '\0';
    }
}

BOOL WINAPI CallbackFunction(HANDLE hProcess, ULONG action, ULONG64 data, ULONG64 context)
{
    (void)context;

    switch (action) {
      case CBA_DEBUG_INFO:
        truncate((PCHAR)data);
        printf("::> %s\n", (PCHAR)data);
        return TRUE;

      case CBA_DEFERRED_SYMBOL_LOAD_CANCEL:
        printf("::> proc=%p action=%08lx data=%p\n",
                      (PVOID)hProcess,
                      action,
                      (PVOID)data);
        {
            PIMAGEHLP_DEFERRED_SYMBOL_LOAD64 pi = (PIMAGEHLP_DEFERRED_SYMBOL_LOAD64)data;
            printf("pi->SizeOfStruct = %ld\n", pi->SizeOfStruct);
            printf("pi->BaseOfImage  = %p\n", (PVOID)(size_t)pi->BaseOfImage);
            printf("pi->CheckSum     = %8lx\n", pi->CheckSum);
            printf("pi->FileName     = %p [%s]\n", pi->FileName, pi->FileName);
            printf("pi->Reparse      = %d\n", pi->Reparse);
        }
        return FALSE;
      default:
        printf("::> proc=%p action=%08lx data=%p\n",
                      (PVOID)hProcess,
                      action,
                      (PVOID)data);
        return FALSE;
    }
}
#endif

int __cdecl main(void)
{
    printf("symtest.exe: Starting.\n");
    fflush(stdout);

    //////////////////////////////////////////////////////// Get the functions.
    //
    HMODULE hDbgHelp = LoadLibraryA("dbghelp.dll");
    if (hDbgHelp == NULL) {
        printf("Couldn't load dbghelp.dll");
        return 1;
    }

    pfImagehlpApiVersionEx
        = (PF_ImagehlpApiVersionEx)GetProcAddress(hDbgHelp,
                                                  "ImagehlpApiVersionEx");
    pfSymInitialize
        = (PF_SymInitialize)GetProcAddress(hDbgHelp, "SymInitialize");
    pfSymSetOptions
        = (PF_SymSetOptions)GetProcAddress(hDbgHelp, "SymSetOptions");
    pfSymGetOptions
        = (PF_SymGetOptions)GetProcAddress(hDbgHelp, "SymGetOptions");
    pfSymLoadModule64
        = (PF_SymLoadModule64)GetProcAddress(hDbgHelp, "SymLoadModule64");
    pfSymGetModuleInfo64
        = (PF_SymGetModuleInfo64)GetProcAddress(hDbgHelp, "SymGetModuleInfo64");
    pfSymFromName
        = (PF_SymFromName)GetProcAddress(hDbgHelp, "SymFromName");
    pfSymRegisterCallback64
        = (PF_SymRegisterCallback64)GetProcAddress(hDbgHelp, "SymRegisterCallback64");
    pfSymEnumerateModules64
        = (PF_SymEnumerateModules64)GetProcAddress(hDbgHelp, "SymEnumerateModules64");
    pfSymEnumSymbols
        = (PF_SymEnumSymbols)GetProcAddress(hDbgHelp, "SymEnumSymbols");

    //////////////////////////////////////////////////////////////////////////////
    //
    HANDLE hProcess = GetCurrentProcess();

    API_VERSION av;
    ZeroMemory(&av, sizeof(av));
    av.MajorVersion = API_VERSION_NUMBER;

    pfImagehlpApiVersionEx(&av);
    printf("  Version: %d.%d (%d)\n",
                  av.MajorVersion,
                  av.MinorVersion,
                  API_VERSION_NUMBER);

    if (!pfSymInitialize(hProcess, NULL, FALSE)) {
        printf("SymInitialize failed: %ld\n", GetLastError());
        return 1;
    }

#if (_MSC_VER > 1299)
    pfSymRegisterCallback64(hProcess, CallbackFunction, NULL);
#endif

    DWORD dw = pfSymGetOptions();
    printf("GetOptions = %08lx\n", dw);
    dw &= ~(SYMOPT_CASE_INSENSITIVE |
            SYMOPT_UNDNAME |
            SYMOPT_DEFERRED_LOADS |
            0);
    dw |= (
#if defined(SYMOPT_EXACT_SYMBOLS)
           SYMOPT_EXACT_SYMBOLS |
#endif
#if defined(SYMOPT_DEBUG)
           SYMOPT_DEBUG |
#endif
#if defined(SYMOPT_NO_UNQUALIFIED_LOADS)
           SYMOPT_NO_UNQUALIFIED_LOADS |
#endif
#if defined(SYMOPT_FAIL_CRITICAL_ERRORS)
           SYMOPT_FAIL_CRITICAL_ERRORS |
#endif
#if defined(SYMOPT_INCLUDE_32BIT_MODULES)
           SYMOPT_INCLUDE_32BIT_MODULES |
#endif
           0);
    printf("SetOptions = %08lx\n", dw);
    pfSymSetOptions(dw);

    /////////////////////////////////////////////// First, try GetProcAddress.
    //
    PCHAR pszFile = "target" DETOURS_STRINGIFY(DETOURS_BITS) ".dll";
    HMODULE hModule = LoadLibraryA(pszFile);
    if (hModule == NULL) {
        printf("LoadLibraryA(%s) failed: %ld\n", pszFile, GetLastError());
        return 2;
    }

    ////////////////////////////////////////////////////// Then try ImageHelp.
    //
#if (_MSC_VER > 1299)

    //CHAR szFull[MAX_PATH];
    //GetModuleFileNameA(hModule, szFull, sizeof(szFull));
    printf("SymLoadModule64(%s) will be called.\n", pszFile /*szFull*/);
    DWORD64 loaded = pfSymLoadModule64(hProcess, NULL,
                                       (PCHAR)pszFile/*szFull*/, NULL,
                                       (DWORD64)hModule, 0);
    if (loaded == 0) {
        printf("SymLoadModule64(%p) failed: %ld\n", hProcess, GetLastError());
        printf("\n");
    }
    else {
        printf("SymLoadModule64(%p) succeeded: 0x%p\n", hProcess, (PVOID)loaded);
    }

    CHAR szModName[512];

    printf("Modules:\n");
// The first parameter of PSYM_ENUMMODULES_CALLBACK64 changed from PSTR to PCSTR
// between Windows 2003 and Windows 7. Cast here to work with either.
    pfSymEnumerateModules64(hProcess, (PSYM_ENUMMODULES_CALLBACK64)SymEnumerateCallback, NULL);
    printf("\n");

    IMAGEHLP_MODULE64 modinfo;
    ZeroMemory(&modinfo, sizeof(modinfo));
    modinfo.SizeOfStruct = 512/*sizeof(modinfo)*/;
    if (!pfSymGetModuleInfo64(hProcess, (DWORD64)hModule, &modinfo)) {
        printf("SymGetModuleInfo64(%p, %p) [64] failed: %ld\n",
                      hProcess, hModule, GetLastError());
    }
    else {
        printf("SymGetModuleInfo64(%p, %p) [64] succeeded: %ld\n",
                      hProcess, hModule, GetLastError());
        StringCchCopyA(szModName, ARRAYSIZE(szModName), modinfo.ModuleName);
        StringCchCatA(szModName, ARRAYSIZE(szModName), "!");

        printf("NumSyms:         %ld\n", modinfo.NumSyms);
        printf("SymType:         %d\n", modinfo.SymType);
        printf("ModuleName:      %s\n", modinfo.ModuleName);
        printf("ImageName:       %s\n", modinfo.ImageName);
        printf("LoadedImageName: %s\n", modinfo.LoadedImageName);
    }

    printf("\n");
    fflush(stdout);

    printf("DLLs:\n");
    for (hModule = NULL; (hModule = DetourEnumerateModules(hModule)) != NULL;) {
        CHAR szName[MAX_PATH];
        GetModuleFileNameA(hModule, szName, sizeof(szName));
        printf("  %p: %s\n", hModule, szName);
    }

    if (pfSymEnumSymbols == NULL) {
        printf("Couldn't find SymEnumSymbols.\n");
    }
    else {
        printf("===Enum===\n");
        SetLastError(0);
        nSymbolCount = 0;
        if (!pfSymEnumSymbols(hProcess, (DWORD64)hModule, NULL, SymEnumerateSymbols, NULL)) {
            printf("SymEnumSymbols() failed: %ld\n",
                          GetLastError());
        }
    }

    // Look for specific symbols.
    struct CFullSymbol : SYMBOL_INFO {
        CHAR szRestOfName[MAX_SYM_NAME];
    } symbol;
    CHAR szFullName[512];

    // Look for Target
    StringCchCopyA(szFullName, ARRAYSIZE(szFullName), szModName);
    StringCchCatA(szFullName, ARRAYSIZE(szFullName), "Target");
    printf("Symbol: [%s]\n", szFullName);

    ZeroMemory(&symbol, sizeof(symbol));
    symbol.SizeOfStruct = sizeof(SYMBOL_INFO);
#ifdef DBHLPAPI
    symbol.MaxNameLen = MAX_SYM_NAME;
#else
    symbol.MaxNameLength = MAX_SYM_NAME;
#endif

    SetLastError(0);
    if (!pfSymFromName(hProcess, szFullName, &symbol)) {
        printf("--SymFromName(%s) failed: %ld\n", szFullName, GetLastError());
    }
    if (symbol.Address != 0) {
        printf("--SymFromName(%s) succeeded\n", szFullName);
    }

    printf("%s => %p\n\n", szFullName, (PBYTE)symbol.Address);

    // Look for Hidden
    StringCchCopyA(szFullName, ARRAYSIZE(szFullName), szModName);
    StringCchCatA(szFullName, ARRAYSIZE(szFullName), "Hidden");
    printf("Symbol: [%s]\n", szFullName);

    ZeroMemory(&symbol, sizeof(symbol));
    symbol.SizeOfStruct = sizeof(SYMBOL_INFO);
#ifdef DBHLPAPI
    symbol.MaxNameLen = MAX_SYM_NAME;
#else
    symbol.MaxNameLength = MAX_SYM_NAME;
#endif

    SetLastError(0);
    if (!pfSymFromName(hProcess, szFullName, &symbol)) {
        printf("--SymFromName(%s) failed: %ld\n", szFullName, GetLastError());
    }
    if (symbol.Address != 0) {
        printf("--SymFromName(%s) succeeded\n", szFullName);
    }

    printf("%s => %p\n\n", szFullName, (PBYTE)symbol.Address);
#endif

    // We call Target once to insure it is loaded.
    Target(0);
    return 0;
}
//
///////////////////////////////////////////////////////////////// End of File.
