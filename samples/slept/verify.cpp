//////////////////////////////////////////////////////////////////////////////
//
//  Detour Test Program (verify.cpp)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//

#include <detours.h>

static VOID Dump(PBYTE pbBytes, LONG nBytes, PBYTE pbTarget)
{
    for (LONG n = 0; n < nBytes; n += 16) {
        printf("    %p: ", pbBytes + n);
        for (LONG m = n; m < n + 16; m++) {
            if (m >= nBytes) {
                printf("  ");
            }
            else {
                printf("%02x", pbBytes[m]);
            }
            if (m % 4 == 3) {
                printf(" ");
            }
        }
        if (n == 0 && pbTarget != DETOUR_INSTRUCTION_TARGET_NONE) {
            printf(" [%p]", pbTarget);
        }
        printf("\n");
    }
}

static VOID Decode(PCSTR pszDesc, PBYTE pbCode, PBYTE pbOther, PBYTE pbPointer, LONG nInst)
{
    if (pbCode != pbPointer) {
        printf("  %s = %p [%p]\n", pszDesc, pbCode, pbPointer);
    }
    else {
        printf("  %s = %p\n", pszDesc, pbCode);
    }

    if (pbCode == pbOther) {
        printf("    ... unchanged ...\n");
        return;
    }

    PBYTE pbSrc = pbCode;
    PBYTE pbEnd;
    PVOID pbTarget;
    for (LONG n = 0; n < nInst; n++) {
        pbEnd = (PBYTE)DetourCopyInstruction(NULL, NULL, pbSrc, &pbTarget, NULL);
        Dump(pbSrc, (int)(pbEnd - pbSrc), (PBYTE)pbTarget);
        pbSrc = pbEnd;
    }
}


VOID WINAPI Verify(PCHAR pszFunc, PVOID pvPointer)
{
    PVOID pvCode = DetourCodeFromPointer(pvPointer, NULL);

    Decode(pszFunc, (PBYTE)pvCode, NULL, (PBYTE)pvPointer, 3);
}

VOID WINAPI VerifyEx(PCHAR pszFunc, PVOID pvPointer, LONG nInst)
{
    PVOID pvCode = DetourCodeFromPointer(pvPointer, NULL);

    Decode(pszFunc, (PBYTE)pvCode, NULL, (PBYTE)pvPointer, nInst);
}

//
///////////////////////////////////////////////////////////////// End of File.
