//////////////////////////////////////////////////////////////////////////////
//
//  Detours Test Program (dtest.cpp of dtest.exe)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
#include <stdio.h>
#include <stdarg.h>
#include <windows.h>
#pragma warning(push)
#if _MSC_VER > 1400
#pragma warning(disable:6102 6103) // /analyze warnings
#endif
#include <strsafe.h>
#pragma warning(pop)
#include <detours.h>
#include "dtarge.h"

DWORD_PTR WINAPI LocalTarget1(DWORD_PTR v1);

////////////////////////////////////////////////////// Multi-Argument Detours.
//
DWORD_PTR (WINAPI * Trampoline_LocalTarget1)(DWORD_PTR v1) = LocalTarget1;

DWORD_PTR (WINAPI * Trampoline_Target0)() = Target0;
DWORD_PTR (WINAPI * Trampoline_Target1)(DWORD_PTR v1) = Target1;
DWORD_PTR (WINAPI * Trampoline_Target2)(DWORD_PTR v1, DWORD_PTR v2) = Target2;
DWORD_PTR (WINAPI * Trampoline_Target3)
    (DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3) = Target3;
DWORD_PTR (WINAPI * Trampoline_Target4)
    (DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4) = Target4;
DWORD_PTR (WINAPI * Trampoline_Target5)
    (DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
     DWORD_PTR v5) = Target5;
DWORD_PTR (WINAPI * Trampoline_Target6)
    (DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
     DWORD_PTR v5, DWORD_PTR v6) = Target6;
DWORD_PTR (WINAPI * Trampoline_Target7)
    (DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
     DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7) = Target7;
DWORD_PTR (WINAPI * Trampoline_Target8)
    (DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
     DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7, DWORD_PTR v8) = Target8;
DWORD_PTR (WINAPI * Trampoline_Target9)
    (DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
     DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7, DWORD_PTR v8,
     DWORD_PTR v9) = Target9;
DWORD_PTR (WINAPI * Trampoline_Target10)
    (DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
     DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7, DWORD_PTR v8,
     DWORD_PTR v9, DWORD_PTR v10) = Target10;
DWORD_PTR (WINAPI * Trampoline_Target11)
    (DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
     DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7, DWORD_PTR v8,
     DWORD_PTR v9, DWORD_PTR v10, DWORD_PTR v11) = Target11;
DWORD_PTR (WINAPI * Trampoline_Target12)
    (DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
     DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7, DWORD_PTR v8,
     DWORD_PTR v9, DWORD_PTR v10, DWORD_PTR v11, DWORD_PTR v12) = Target12;
DWORD_PTR (WINAPI * Trampoline_Target13)
    (DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
     DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7, DWORD_PTR v8,
     DWORD_PTR v9, DWORD_PTR v10, DWORD_PTR v11, DWORD_PTR v12,
     DWORD_PTR v13) = Target13;
DWORD_PTR (WINAPI * Trampoline_Target14)
    (DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
     DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7, DWORD_PTR v8,
     DWORD_PTR v9, DWORD_PTR v10, DWORD_PTR v11, DWORD_PTR v12,
     DWORD_PTR v13, DWORD_PTR v14) = Target14;
DWORD_PTR (WINAPI * Trampoline_Target15)
    (DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
     DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7, DWORD_PTR v8,
     DWORD_PTR v9, DWORD_PTR v10, DWORD_PTR v11, DWORD_PTR v12,
     DWORD_PTR v13, DWORD_PTR v14, DWORD_PTR v15) = Target15;
DWORD_PTR (WINAPI * Trampoline_Target16)
    (DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
     DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7, DWORD_PTR v8,
     DWORD_PTR v9, DWORD_PTR v10, DWORD_PTR v11, DWORD_PTR v12,
     DWORD_PTR v13, DWORD_PTR v14, DWORD_PTR v15, DWORD_PTR v16) = Target16;
DWORD_PTR (WINAPI * Trampoline_TargetV)(DWORD_PTR v1, ...) = TargetV;
DWORD_PTR (WINAPI * Trampoline_TargetR)(DWORD_PTR v1, ...) = TargetR;

//////////////////////////////////////////////////////////////////////////////
//
VOID dprintf(const char * fmt, ...)
{
    CHAR szBuf[1024];

    va_list args;
    va_start(args, fmt);
    StringCchPrintfA(szBuf, sizeof(szBuf), fmt, args);
    va_end(args);

    OutputDebugStringA(szBuf);
}

//////////////////////////////////////////////////////////////////////////////
//
DWORD_PTR WINAPI LocalTarget1(DWORD_PTR v1)
{
    printf("  LocalTarget1 (%ld)\n", (DWORD)v1);
    // dprintf("LocalTarget1\n");
    // __debugbreak();
    return 9000;
}

//////////////////////////////////////////////////////////////////////////////
//
DWORD_PTR WINAPI MyLocalTarget1(DWORD_PTR v1)
{
    printf("  MyLocalTarget1 (%ld)\n",
           (DWORD)v1);
    // dprintf("LocalTarget1, Trampoline_LocalTarget1=%p\n", Trampoline_LocalTarget1);
    return Trampoline_LocalTarget1(v1);
}

DWORD_PTR WINAPI MyTarget0()
{
    printf("  MyTarget0 ()\n");
    return Trampoline_Target0();
}

DWORD_PTR WINAPI MyTarget1(DWORD_PTR v1)
{
    printf("  MyTarget1 (%ld)\n",
           (DWORD)v1);
    return Trampoline_Target1(v1);
}

DWORD_PTR WINAPI MyTarget2(DWORD_PTR v1, DWORD_PTR v2)
{
    printf("  MyTarget2 (%ld,%ld)\n",
           (DWORD)v1, (DWORD)v2);
    return Trampoline_Target2(v1,v2);
}

DWORD_PTR WINAPI MyTarget3(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3)
{
    printf("  MyTarget3 (%ld,%ld,%ld)\n",
           (DWORD)v1, (DWORD)v2, (DWORD)v3);
    return Trampoline_Target3(v1,v2,v3);
}

DWORD_PTR WINAPI MyTarget4(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4)
{
    printf("  MyTarget4 (%ld,%ld,%ld,%ld)\n",
           (DWORD)v1, (DWORD)v2, (DWORD)v3, (DWORD)v4);
    return Trampoline_Target4(v1,v2,v3,v4);
}

DWORD_PTR WINAPI MyTarget5(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
                           DWORD_PTR v5)
{
    printf("  MyTarget5 (%ld,%ld,%ld,%ld,%ld)\n",
           (DWORD)v1, (DWORD)v2, (DWORD)v3, (DWORD)v4,
           (DWORD)v5);
    return Trampoline_Target5(v1,v2,v3,v4,v5);
}

DWORD_PTR WINAPI MyTarget6(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
                           DWORD_PTR v5, DWORD_PTR v6)
{
    printf("  MyTarget6 (%ld,%ld,%ld,%ld,%ld,%ld)\n",
           (DWORD)v1, (DWORD)v2, (DWORD)v3, (DWORD)v4,
           (DWORD)v5, (DWORD)v6);
    return Trampoline_Target6(v1,v2,v3,v4,v5,v6);
}

DWORD_PTR WINAPI MyTarget7(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
                           DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7)
{
    printf("  MyTarget7 (%ld,%ld,%ld,%ld,%ld,%ld,%ld)\n",
           (DWORD)v1, (DWORD)v2, (DWORD)v3, (DWORD)v4,
           (DWORD)v5, (DWORD)v6, (DWORD)v7);
    return Trampoline_Target7(v1,v2,v3,v4,v5,v6,v7);
}

DWORD_PTR WINAPI MyTarget8(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
                           DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7, DWORD_PTR v8)
{
    printf("  MyTarget8 (%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld)\n",
           (DWORD)v1, (DWORD)v2, (DWORD)v3, (DWORD)v4,
           (DWORD)v5, (DWORD)v6, (DWORD)v7, (DWORD)v8);
    return Trampoline_Target8(v1,v2,v3,v4,v5,v6,v7,v8);
}

DWORD_PTR WINAPI MyTarget9(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
                         DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7, DWORD_PTR v8,
                         DWORD_PTR v9)
{
    printf("  MyTarget9 (%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld)\n",
           (DWORD)v1, (DWORD)v2, (DWORD)v3, (DWORD)v4,
           (DWORD)v5, (DWORD)v6, (DWORD)v7, (DWORD)v8,
           (DWORD)v9);
    return Trampoline_Target9(v1,v2,v3,v4,v5,v6,v7,v8,v9);
}

DWORD_PTR WINAPI MyTarget10(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
                            DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7, DWORD_PTR v8,
                            DWORD_PTR v9, DWORD_PTR v10)
{
    printf("  MyTarget10(%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld)\n",
           (DWORD)v1, (DWORD)v2, (DWORD)v3, (DWORD)v4,
           (DWORD)v5, (DWORD)v6, (DWORD)v7, (DWORD)v8,
           (DWORD)v9, (DWORD)v10);
    return Trampoline_Target10(v1,v2,v3,v4,v5,v6,v7,v8,v9,v10);
}

DWORD_PTR WINAPI MyTarget11(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
                            DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7, DWORD_PTR v8,
                            DWORD_PTR v9, DWORD_PTR v10, DWORD_PTR v11)
{
    printf("  MyTarget11(%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld)\n",
           (DWORD)v1, (DWORD)v2, (DWORD)v3, (DWORD)v4,
           (DWORD)v5, (DWORD)v6, (DWORD)v7, (DWORD)v8,
           (DWORD)v9, (DWORD)v10, (DWORD)v11);
    return Trampoline_Target11(v1,v2,v3,v4,v5,v6,v7,v8,v9,v10,v11);
}

DWORD_PTR WINAPI MyTarget12(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
                            DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7, DWORD_PTR v8,
                            DWORD_PTR v9, DWORD_PTR v10, DWORD_PTR v11, DWORD_PTR v12)
{
    printf("  MyTarget12(%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld)\n",
           (DWORD)v1, (DWORD)v2, (DWORD)v3, (DWORD)v4,
           (DWORD)v5, (DWORD)v6, (DWORD)v7, (DWORD)v8,
           (DWORD)v9, (DWORD)v10, (DWORD)v11, (DWORD)v12);
    return Trampoline_Target12(v1,v2,v3,v4,v5,v6,v7,v8,v9,v10,v11,v12);
}

DWORD_PTR WINAPI MyTarget13(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
                            DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7, DWORD_PTR v8,
                            DWORD_PTR v9, DWORD_PTR v10, DWORD_PTR v11, DWORD_PTR v12,
                            DWORD_PTR v13)
{
    printf("  MyTarget13(%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld)\n",
           (DWORD)v1, (DWORD)v2, (DWORD)v3, (DWORD)v4,
           (DWORD)v5, (DWORD)v6, (DWORD)v7, (DWORD)v8,
           (DWORD)v9, (DWORD)v10, (DWORD)v11, (DWORD)v12,
           (DWORD)v13);
    return Trampoline_Target13(v1,v2,v3,v4,v5,v6,v7,v8,v9,v10,v11,v12,v13);
}

DWORD_PTR WINAPI MyTarget14(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
                            DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7, DWORD_PTR v8,
                            DWORD_PTR v9, DWORD_PTR v10, DWORD_PTR v11, DWORD_PTR v12,
                            DWORD_PTR v13, DWORD_PTR v14)
{
    printf("  MyTarget14(%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld)\n",
           (DWORD)v1, (DWORD)v2, (DWORD)v3, (DWORD)v4,
           (DWORD)v5, (DWORD)v6, (DWORD)v7, (DWORD)v8,
           (DWORD)v9, (DWORD)v10, (DWORD)v11, (DWORD)v12,
           (DWORD)v13, (DWORD)v14);
    return Trampoline_Target14(v1,v2,v3,v4,v5,v6,v7,v8,v9,v10,v11,v12,v13,v14);
}

DWORD_PTR WINAPI MyTarget15(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
                            DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7, DWORD_PTR v8,
                            DWORD_PTR v9, DWORD_PTR v10, DWORD_PTR v11, DWORD_PTR v12,
                            DWORD_PTR v13, DWORD_PTR v14, DWORD_PTR v15)
{
    printf("  MyTarget15(%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld)\n",
           (DWORD)v1, (DWORD)v2, (DWORD)v3, (DWORD)v4,
           (DWORD)v5, (DWORD)v6, (DWORD)v7, (DWORD)v8,
           (DWORD)v9, (DWORD)v10, (DWORD)v11, (DWORD)v12,
           (DWORD)v13, (DWORD)v14, (DWORD)v15);
    return Trampoline_Target15(v1,v2,v3,v4,v5,v6,v7,v8,v9,v10,v11,v12,v13,v14,v15);
}

DWORD_PTR WINAPI MyTarget16(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
                            DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7, DWORD_PTR v8,
                            DWORD_PTR v9, DWORD_PTR v10, DWORD_PTR v11, DWORD_PTR v12,
                            DWORD_PTR v13, DWORD_PTR v14, DWORD_PTR v15, DWORD_PTR v16)
{
    printf("  MyTarget16(%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld)\n",
           (DWORD)v1, (DWORD)v2, (DWORD)v3, (DWORD)v4,
           (DWORD)v5, (DWORD)v6, (DWORD)v7, (DWORD)v8,
           (DWORD)v9, (DWORD)v10, (DWORD)v11, (DWORD)v12,
           (DWORD)v13, (DWORD)v14, (DWORD)v15, (DWORD)v16);
    return Trampoline_Target16(v1,v2,v3,v4,v5,v6,v7,v8,v9,v10,v11,v12,v13,v14,v15,v16);
}

DWORD_PTR WINAPI MyTargetV(DWORD_PTR v1, ...)
{
    DWORD_PTR args[32];

    va_list va;
    va_start(va, v1);

    int argc = 0;
    for (args[argc++] = v1; args[argc-1] != 0;) {
        args[argc++] = va_arg(va, DWORD_PTR);
    }
    va_end(va);

    printf("  MyTargetV (");
    int i = argc - 1;
    for (; i > 0; i--) {
        printf("%ld,", (DWORD)args[i]);
    }
    printf("%ld)\n", (DWORD)args[0]);

    switch (argc) {
      default:
        return Trampoline_TargetV(0);
      case 1:
        return Trampoline_TargetV(args[0]);
      case 2:
        return Trampoline_TargetV(args[0], args[1]);
      case 3:
        return Trampoline_TargetV(args[0], args[1], args[2]);
      case 4:
        return Trampoline_TargetV(args[0], args[1], args[2], args[3]);
      case 5:
        return Trampoline_TargetV(args[0], args[1], args[2], args[3],
                                  args[4]);
      case 6:
        return Trampoline_TargetV(args[0], args[1], args[2], args[3],
                                  args[4], args[5]);
      case 7:
        return Trampoline_TargetV(args[0], args[1], args[2], args[3],
                                  args[4], args[5], args[6]);
      case 8:
        return Trampoline_TargetV(args[0], args[1], args[2], args[3],
                                  args[4], args[5], args[6], args[7]);
      case 9:
        return Trampoline_TargetV(args[0], args[1], args[2], args[3],
                                  args[4], args[5], args[6], args[7],
                                  args[8]);
      case 10:
        return Trampoline_TargetV(args[0], args[1], args[2], args[3],
                                  args[4], args[5], args[6], args[7],
                                  args[8], args[9]);
      case 11:
        return Trampoline_TargetV(args[0], args[1], args[2], args[3],
                                  args[4], args[5], args[6], args[7],
                                  args[8], args[9], args[10]);
      case 12:
        return Trampoline_TargetV(args[0], args[1], args[2], args[3],
                                  args[4], args[5], args[6], args[7],
                                  args[8], args[9], args[10], args[11]);
      case 13:
        return Trampoline_TargetV(args[0], args[1], args[2], args[3],
                                  args[4], args[5], args[6], args[7],
                                  args[8], args[9], args[10], args[11],
                                  args[12]);
      case 14:
        return Trampoline_TargetV(args[0], args[1], args[2], args[3],
                                  args[4], args[5], args[6], args[7],
                                  args[8], args[9], args[10], args[11],
                                  args[12], args[13]);
      case 15:
        return Trampoline_TargetV(args[0], args[1], args[2], args[3],
                                  args[4], args[5], args[6], args[7],
                                  args[8], args[9], args[10], args[11],
                                  args[12], args[13], args[14]);
      case 16:
        return Trampoline_TargetV(args[0], args[1], args[2], args[3],
                                  args[4], args[5], args[6], args[7],
                                  args[8], args[9], args[10], args[11],
                                  args[12], args[13], args[14], args[15]);
      case 17:
        return Trampoline_TargetV(args[0], args[1], args[2], args[3],
                                  args[4], args[5], args[6], args[7],
                                  args[8], args[9], args[10], args[11],
                                  args[12], args[13], args[14], args[15],
                                  args[16]);
    }
}

DWORD_PTR WINAPI MyTargetR(DWORD_PTR v1, ...)
{
    DWORD_PTR args[32];

    va_list va;
    va_start(va, v1);

    int argc = 0;
    for (args[argc++] = v1; args[argc-1] != 0;) {
        args[argc++] = va_arg(va, DWORD_PTR);
    }
    va_end(va);

    if (v1 < 5) {
        printf("  MyTargetR (");
        int i = argc - 1;
        for (; i > 0; i--) {
            printf("%ld,", (DWORD)args[i]);
        }
        printf("%ld)\n", (DWORD)args[0]);
    }
    else {
        printf(".");
    }

    switch (argc) {
      default:
        return Trampoline_TargetR(0);
      case 1:
        return Trampoline_TargetR(args[0]);
      case 2:
        return Trampoline_TargetR(args[0], args[1]);
      case 3:
        return Trampoline_TargetR(args[0], args[1], args[2]);
      case 4:
        return Trampoline_TargetR(args[0], args[1], args[2], args[3]);
      case 5:
        return Trampoline_TargetR(args[0], args[1], args[2], args[3],
                                  args[4]);
      case 6:
        return Trampoline_TargetR(args[0], args[1], args[2], args[3],
                                  args[4], args[5]);
      case 7:
        return Trampoline_TargetR(args[0], args[1], args[2], args[3],
                                  args[4], args[5], args[6]);
      case 8:
        return Trampoline_TargetR(args[0], args[1], args[2], args[3],
                                  args[4], args[5], args[6], args[7]);
      case 9:
        return Trampoline_TargetR(args[0], args[1], args[2], args[3],
                                  args[4], args[5], args[6], args[7],
                                  args[8]);
      case 10:
        return Trampoline_TargetR(args[0], args[1], args[2], args[3],
                                  args[4], args[5], args[6], args[7],
                                  args[8], args[9]);
      case 11:
        return Trampoline_TargetR(args[0], args[1], args[2], args[3],
                                  args[4], args[5], args[6], args[7],
                                  args[8], args[9], args[10]);
      case 12:
        return Trampoline_TargetR(args[0], args[1], args[2], args[3],
                                  args[4], args[5], args[6], args[7],
                                  args[8], args[9], args[10], args[11]);
      case 13:
        return Trampoline_TargetR(args[0], args[1], args[2], args[3],
                                  args[4], args[5], args[6], args[7],
                                  args[8], args[9], args[10], args[11],
                                  args[12]);
      case 14:
        return Trampoline_TargetR(args[0], args[1], args[2], args[3],
                                  args[4], args[5], args[6], args[7],
                                  args[8], args[9], args[10], args[11],
                                  args[12], args[13]);
      case 15:
        return Trampoline_TargetR(args[0], args[1], args[2], args[3],
                                  args[4], args[5], args[6], args[7],
                                  args[8], args[9], args[10], args[11],
                                  args[12], args[13], args[14]);
      case 16:
        return Trampoline_TargetR(args[0], args[1], args[2], args[3],
                                  args[4], args[5], args[6], args[7],
                                  args[8], args[9], args[10], args[11],
                                  args[12], args[13], args[14], args[15]);
      case 17:
        return Trampoline_TargetR(args[0], args[1], args[2], args[3],
                                  args[4], args[5], args[6], args[7],
                                  args[8], args[9], args[10], args[11],
                                  args[12], args[13], args[14], args[15],
                                  args[16]);
    }
}

/////////////////////////////////////////////////////////// Recursive Detours.
//
DWORD_PTR (WINAPI * Trampoline_Target0_1)() = NULL;
DWORD_PTR (WINAPI * Trampoline_Target0_2)() = NULL;
DWORD_PTR (WINAPI * Trampoline_Target0_3)() = NULL;

static DWORD_PTR WINAPI MyTarget0_1()
{
    printf("  Starting Target0_1.\n");
    DWORD_PTR rv = Trampoline_Target0_1();
    printf("  End Target0_1.\n");
    return rv;
}

static DWORD_PTR WINAPI MyTarget0_2()
{
    printf("  Starting Target0_2.\n");
    DWORD_PTR rv = Trampoline_Target0_2();
    printf("  End Target0_2.\n");
    return rv;
}

static DWORD_PTR WINAPI MyTarget0_3()
{
    printf("  Starting Target0_3.\n");
    DWORD_PTR rv = Trampoline_Target0_3();
    printf("  End Target0_3.\n");
    return rv;
}

//////////////////////////////////////////////////////////////////////////////
//
int WINAPI WinMain(HINSTANCE hinst, HINSTANCE hprev, LPSTR lpszCmdLine, int nCmdShow)
{
    (void)hinst;
    (void)hprev;
    (void)lpszCmdLine;
    (void)nCmdShow;

    printf("Calling LocalTarget1 w/o detour\n");
    LocalTarget1(1);

    DetourTransactionBegin();
    DetourUpdateThread(GetCurrentThread());
    DetourAttach(&(PVOID&)Trampoline_LocalTarget1, MyLocalTarget1);
    DetourTransactionCommit();

    printf("Calling LocalTarget1 w/ detour\n");
    LocalTarget1(2);

    DetourTransactionBegin();
    DetourUpdateThread(GetCurrentThread());
    DetourAttach(&(PVOID&)Trampoline_Target0, MyTarget0);
    DetourTransactionCommit();

    printf("Calling Target0 function.\n");
    //dprintf("- Trampoline_Target0:: %p\n", Trampoline_Target0);
    //dprintf("- Target0           :: %p\n", Target0);
    Target0();

    DetourTransactionBegin();
    DetourUpdateThread(GetCurrentThread());
    DetourAttach(&(PVOID&)Trampoline_Target1, MyTarget1);
    DetourAttach(&(PVOID&)Trampoline_Target2, MyTarget2);
    DetourAttach(&(PVOID&)Trampoline_Target3, MyTarget3);
    DetourAttach(&(PVOID&)Trampoline_Target4, MyTarget4);
    DetourAttach(&(PVOID&)Trampoline_Target5, MyTarget5);
    DetourAttach(&(PVOID&)Trampoline_Target6, MyTarget6);
    DetourAttach(&(PVOID&)Trampoline_Target7, MyTarget7);
    DetourAttach(&(PVOID&)Trampoline_Target8, MyTarget8);
    DetourAttach(&(PVOID&)Trampoline_Target9, MyTarget9);
    DetourAttach(&(PVOID&)Trampoline_Target10, MyTarget10);
    DetourAttach(&(PVOID&)Trampoline_Target11, MyTarget11);
    DetourAttach(&(PVOID&)Trampoline_Target12, MyTarget12);
    DetourAttach(&(PVOID&)Trampoline_Target13, MyTarget13);
    DetourAttach(&(PVOID&)Trampoline_Target14, MyTarget14);
    DetourAttach(&(PVOID&)Trampoline_Target15, MyTarget15);
    DetourAttach(&(PVOID&)Trampoline_Target16, MyTarget16);
    DetourAttach(&(PVOID&)Trampoline_TargetV, MyTargetV);
    DetourAttach(&(PVOID&)Trampoline_TargetR, MyTargetR);
    DetourTransactionCommit();

    printf("Calling TargetN functions.\n");
    LocalTarget1(1);
    Target0();
    Target1(1);
    Target2(1,2);
    Target3(1,2,3);
    Target4(1,2,3,4);
    Target5(1,2,3,4,5);
    Target6(1,2,3,4,5,6);
    Target7(1,2,3,4,5,6,7);
    Target8(1,2,3,4,5,6,7,8);
    Target9(1,2,3,4,5,6,7,8,9);
    Target10(1,2,3,4,5,6,7,8,9,10);
    Target11(1,2,3,4,5,6,7,8,9,10,11);
    Target12(1,2,3,4,5,6,7,8,9,10,11,12);
    Target13(1,2,3,4,5,6,7,8,9,10,11,12,13);
    Target14(1,2,3,4,5,6,7,8,9,10,11,12,13,14);
    Target15(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15);
    Target16(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16);
    TargetV(0);
    TargetV(1,0);
    TargetV(2,1,0);
    TargetV(3,2,1,0);
    TargetV(4,3,2,1,0);
    TargetV(5,4,3,2,1,0);
    TargetV(6,5,4,3,2,1,0);
    TargetV(7,6,5,4,3,2,1,0);
    TargetV(8,7,6,5,4,3,2,1,0);
    TargetV(9,8,7,6,5,4,3,2,1,0);
    TargetV(10,9,8,7,6,5,4,3,2,1,0);
    TargetV(11,10,9,8,7,6,5,4,3,2,1,0);
    TargetV(12,11,10,9,8,7,6,5,4,3,2,1,0);
    TargetV(13,12,11,10,9,8,7,6,5,4,3,2,1,0);
    TargetV(14,13,12,11,10,9,8,7,6,5,4,3,2,1,0);
    TargetV(15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0);
    TargetV(16,15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0);
    TargetR(4,3,2,1,0);
    DWORD_PTR rv = TargetR(100,10,9,8,7,6,5,4,3,2,1,0);
    printf(" => %ld\n", (DWORD)rv);

    Trampoline_Target0_1 = Target0;
    Trampoline_Target0_2 = Target0;
    Trampoline_Target0_3 = Target0;

    //dprintf("Trampoline_Target0_1 = %p\n", DetourCodeFromPointer(Trampoline_Target0_1, NULL));
    //__debugbreak();

    printf("Calling Target0 again with 1 detour.\n");
    Target0();

    DetourTransactionBegin();
    DetourUpdateThread(GetCurrentThread());
    DetourAttach(&(PVOID&)Trampoline_Target0_1, MyTarget0_1);
    DetourTransactionCommit();

    //dprintf("Trampoline_Target0_2 = %p\n", DetourCodeFromPointer(Trampoline_Target0_2, NULL));
    //__debugbreak();
    printf("Calling Target0 again with 2 detours.\n");
    Target0();

    DetourTransactionBegin();
    DetourUpdateThread(GetCurrentThread());
    DetourAttach(&(PVOID&)Trampoline_Target0_2, MyTarget0_2);
    DetourTransactionCommit();

    //dprintf("Trampoline_Target0_3 = %p\n", DetourCodeFromPointer(Trampoline_Target0_3, NULL));
    //__debugbreak();
    printf("Calling Target0 again with 3 detours.\n");
    Target0();


    DetourTransactionBegin();
    DetourUpdateThread(GetCurrentThread());
    DetourAttach(&(PVOID&)Trampoline_Target0_3, MyTarget0_3);
    DetourTransactionCommit();

    //dprintf("Trampoline_Target0_3 = %p\n", DetourCodeFromPointer(Trampoline_Target0_3, NULL));
    //__debugbreak();
    printf("Calling Target0 again with 4 detours.\n");
    Target0();

    DetourTransactionBegin();
    DetourUpdateThread(GetCurrentThread());
    DetourDetach(&(PVOID&)Trampoline_Target0, MyTarget0);
    DetourDetach(&(PVOID&)Trampoline_Target1, MyTarget1);
    DetourDetach(&(PVOID&)Trampoline_Target2, MyTarget2);
    DetourDetach(&(PVOID&)Trampoline_Target3, MyTarget3);
    DetourDetach(&(PVOID&)Trampoline_Target4, MyTarget4);
    DetourDetach(&(PVOID&)Trampoline_Target5, MyTarget5);
    DetourDetach(&(PVOID&)Trampoline_Target6, MyTarget6);
    DetourDetach(&(PVOID&)Trampoline_Target7, MyTarget7);
    DetourDetach(&(PVOID&)Trampoline_Target8, MyTarget8);
    DetourDetach(&(PVOID&)Trampoline_Target9, MyTarget9);
    DetourDetach(&(PVOID&)Trampoline_Target10, MyTarget10);
    DetourDetach(&(PVOID&)Trampoline_Target11, MyTarget11);
    DetourDetach(&(PVOID&)Trampoline_Target12, MyTarget12);
    DetourDetach(&(PVOID&)Trampoline_Target13, MyTarget13);
    DetourDetach(&(PVOID&)Trampoline_Target14, MyTarget14);
    DetourDetach(&(PVOID&)Trampoline_Target15, MyTarget15);
    DetourDetach(&(PVOID&)Trampoline_Target16, MyTarget16);
    DetourDetach(&(PVOID&)Trampoline_TargetV, MyTargetV);
    DetourDetach(&(PVOID&)Trampoline_TargetR, MyTargetR);
    DetourTransactionCommit();

    printf("Done.\n");

    return 0;
}

//
///////////////////////////////////////////////////////////////// End of File.
