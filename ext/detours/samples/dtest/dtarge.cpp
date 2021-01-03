//////////////////////////////////////////////////////////////////////////////
//
//  Detours Test Program (dtarge.dll)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
#include <stdio.h>
#include <windows.h>
#include "dtarge.h"

DWORD_PTR WINAPI Target0()
{
    printf("    Target0 ()\n");
    return 1000;
}

DWORD_PTR WINAPI Target1(DWORD_PTR v1)
{
    printf("    Target1 (%ld)\n",
           (DWORD)v1);
    return 1001;
}

DWORD_PTR WINAPI Target2(DWORD_PTR v1, DWORD_PTR v2)
{
    printf("    Target2 (%ld,%ld)\n",
           (DWORD)v1, (DWORD)v2);
    return 1002;
}

DWORD_PTR WINAPI Target3(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3)
{
    printf("    Target3 (%ld,%ld,%ld)\n",
           (DWORD)v1, (DWORD)v2, (DWORD)v3);
    return 1003;
}

DWORD_PTR WINAPI Target4(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4)
{
    printf("    Target4 (%ld,%ld,%ld,%ld)\n",
           (DWORD)v1, (DWORD)v2, (DWORD)v3, (DWORD)v4);
    return 1004;
}

DWORD_PTR WINAPI Target5(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
                         DWORD_PTR v5)
{
    printf("    Target5 (%ld,%ld,%ld,%ld,%ld)\n",
           (DWORD)v1, (DWORD)v2, (DWORD)v3, (DWORD)v4,
           (DWORD)v5);
    return 1005;
}

DWORD_PTR WINAPI Target6(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
                         DWORD_PTR v5, DWORD_PTR v6)
{
    printf("    Target6 (%ld,%ld,%ld,%ld,%ld,%ld)\n",
           (DWORD)v1, (DWORD)v2, (DWORD)v3, (DWORD)v4,
           (DWORD)v5, (DWORD)v6);
    return 1006;
}

DWORD_PTR WINAPI Target7(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
                         DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7)
{
    printf("    Target7 (%ld,%ld,%ld,%ld,%ld,%ld,%ld)\n",
           (DWORD)v1, (DWORD)v2, (DWORD)v3, (DWORD)v4,
           (DWORD)v5, (DWORD)v6, (DWORD)v7);
    return 1007;
}

DWORD_PTR WINAPI Target8(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
                         DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7, DWORD_PTR v8)
{
    printf("    Target8 (%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld)\n",
           (DWORD)v1, (DWORD)v2, (DWORD)v3, (DWORD)v4,
           (DWORD)v5, (DWORD)v6, (DWORD)v7, (DWORD)v8);
    return 1008;
}

DWORD_PTR WINAPI Target9(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
                         DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7, DWORD_PTR v8,
                         DWORD_PTR v9)
{
    printf("    Target9 (%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld)\n",
           (DWORD)v1, (DWORD)v2, (DWORD)v3, (DWORD)v4,
           (DWORD)v5, (DWORD)v6, (DWORD)v7, (DWORD)v8,
           (DWORD)v9);
    return 1009;
}

DWORD_PTR WINAPI Target10(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
                          DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7, DWORD_PTR v8,
                          DWORD_PTR v9, DWORD_PTR v10)
{
    printf("    Target10(%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld)\n",
           (DWORD)v1, (DWORD)v2, (DWORD)v3, (DWORD)v4,
           (DWORD)v5, (DWORD)v6, (DWORD)v7, (DWORD)v8,
           (DWORD)v9, (DWORD)v10);
    return 1010;
}

DWORD_PTR WINAPI Target11(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
                          DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7, DWORD_PTR v8,
                          DWORD_PTR v9, DWORD_PTR v10, DWORD_PTR v11)
{
    printf("    Target11(%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld)\n",
           (DWORD)v1, (DWORD)v2, (DWORD)v3, (DWORD)v4,
           (DWORD)v5, (DWORD)v6, (DWORD)v7, (DWORD)v8,
           (DWORD)v9, (DWORD)v10, (DWORD)v11);
    return 1011;
}

DWORD_PTR WINAPI Target12(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
                          DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7, DWORD_PTR v8,
                          DWORD_PTR v9, DWORD_PTR v10, DWORD_PTR v11, DWORD_PTR v12)
{
    printf("    Target12(%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld)\n",
           (DWORD)v1, (DWORD)v2, (DWORD)v3, (DWORD)v4,
           (DWORD)v5, (DWORD)v6, (DWORD)v7, (DWORD)v8,
           (DWORD)v9, (DWORD)v10, (DWORD)v11, (DWORD)v12);
    return 1012;
}

DWORD_PTR WINAPI Target13(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
                          DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7, DWORD_PTR v8,
                          DWORD_PTR v9, DWORD_PTR v10, DWORD_PTR v11, DWORD_PTR v12,
                          DWORD_PTR v13)
{
    printf("    Target13(%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld)\n",
           (DWORD)v1, (DWORD)v2, (DWORD)v3, (DWORD)v4,
           (DWORD)v5, (DWORD)v6, (DWORD)v7, (DWORD)v8,
           (DWORD)v9, (DWORD)v10, (DWORD)v11, (DWORD)v12,
           (DWORD)v13);
    return 1013;
}

DWORD_PTR WINAPI Target14(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
                          DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7, DWORD_PTR v8,
                          DWORD_PTR v9, DWORD_PTR v10, DWORD_PTR v11, DWORD_PTR v12,
                          DWORD_PTR v13, DWORD_PTR v14)
{
    printf("    Target14(%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld)\n",
           (DWORD)v1, (DWORD)v2, (DWORD)v3, (DWORD)v4,
           (DWORD)v5, (DWORD)v6, (DWORD)v7, (DWORD)v8,
           (DWORD)v9, (DWORD)v10, (DWORD)v11, (DWORD)v12,
           (DWORD)v13, (DWORD)v14);
    return 1014;
}

DWORD_PTR WINAPI Target15(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
                          DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7, DWORD_PTR v8,
                          DWORD_PTR v9, DWORD_PTR v10, DWORD_PTR v11, DWORD_PTR v12,
                          DWORD_PTR v13, DWORD_PTR v14, DWORD_PTR v15)
{
    printf("    Target15(%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld)\n",
           (DWORD)v1, (DWORD)v2, (DWORD)v3, (DWORD)v4,
           (DWORD)v5, (DWORD)v6, (DWORD)v7, (DWORD)v8,
           (DWORD)v9, (DWORD)v10, (DWORD)v11, (DWORD)v12,
           (DWORD)v13, (DWORD)v14, (DWORD)v15);
    return 1015;
}

DWORD_PTR WINAPI Target16(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
                          DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7, DWORD_PTR v8,
                          DWORD_PTR v9, DWORD_PTR v10, DWORD_PTR v11, DWORD_PTR v12,
                          DWORD_PTR v13, DWORD_PTR v14, DWORD_PTR v15, DWORD_PTR v16)
{
    printf("    Target16(%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld,%ld)\n",
           (DWORD)v1, (DWORD)v2, (DWORD)v3, (DWORD)v4,
           (DWORD)v5, (DWORD)v6, (DWORD)v7, (DWORD)v8,
           (DWORD)v9, (DWORD)v10, (DWORD)v11, (DWORD)v12,
           (DWORD)v13, (DWORD)v14, (DWORD)v15, (DWORD)v16);
    return 1016;
}

DWORD_PTR WINAPI TargetV(DWORD_PTR v1, ...)
{
    DWORD_PTR args[32];

    va_list va;
    va_start(va, v1);

    int argc = 0;
    for (args[argc++] = v1; args[argc-1] != 0;) {
        args[argc++] = va_arg(va, DWORD_PTR);
    }
    va_end(va);

    printf("    TargetV (");
    int i = argc - 1;
    for (; i > 0; i--) {
        printf("%ld,", (DWORD)args[i]);
    }
    printf("%ld)\n", (DWORD)args[0]);

    return 1000 + argc;
}

DWORD_PTR WINAPI TargetR(DWORD_PTR v1, ...)
{
    DWORD_PTR args[32];

    va_list va;
    va_start(va, v1);

    int argc = 0;
    for (args[argc++] = v1; args[argc-1] != 0;) {
        args[argc++] = va_arg(va, DWORD_PTR);
    }
    va_end(va);

    if (v1 > 1) {
        printf(":");
        switch (argc) {
          default:
            return TargetR(0) + 1;
          case 1:
            return TargetR(args[0] - 1) + 1;
          case 2:
            return TargetR(args[0] - 1, args[1]) + 1;
          case 3:
            return TargetR(args[0] - 1, args[1], args[2]) + 1;
          case 4:
            return TargetR(args[0] - 1, args[1], args[2], args[3]) + 1;
          case 5:
            return TargetR(args[0] - 1, args[1], args[2], args[3],
                           args[4]) + 1;
          case 6:
            return TargetR(args[0] - 1, args[1], args[2], args[3],
                           args[4], args[5]) + 1;
          case 7:
            return TargetR(args[0] - 1, args[1], args[2], args[3],
                           args[4], args[5], args[6]) + 1;
          case 8:
            return TargetR(args[0] - 1, args[1], args[2], args[3],
                           args[4], args[5], args[6], args[7]) + 1;
          case 9:
            return TargetR(args[0] - 1, args[1], args[2], args[3],
                           args[4], args[5], args[6], args[7],
                           args[8]) + 1;
          case 10:
            return TargetR(args[0] - 1, args[1], args[2], args[3],
                           args[4], args[5], args[6], args[7],
                           args[8], args[9]) + 1;
          case 11:
            return TargetR(args[0] - 1, args[1], args[2], args[3],
                           args[4], args[5], args[6], args[7],
                           args[8], args[9], args[10]) + 1;
          case 12:
            return TargetR(args[0] - 1, args[1], args[2], args[3],
                           args[4], args[5], args[6], args[7],
                           args[8], args[9], args[10], args[11]) + 1;
          case 13:
            return TargetR(args[0] - 1, args[1], args[2], args[3],
                           args[4], args[5], args[6], args[7],
                           args[8], args[9], args[10], args[11],
                           args[12]) + 1;
          case 14:
            return TargetR(args[0] - 1, args[1], args[2], args[3],
                           args[4], args[5], args[6], args[7],
                           args[8], args[9], args[10], args[11],
                           args[12], args[13]) + 1;
          case 15:
            return TargetR(args[0] - 1, args[1], args[2], args[3],
                           args[4], args[5], args[6], args[7],
                           args[8], args[9], args[10], args[11],
                           args[12], args[13], args[14]) + 1;
          case 16:
            return TargetR(args[0] - 1, args[1], args[2], args[3],
                           args[4], args[5], args[6], args[7],
                           args[8], args[9], args[10], args[11],
                           args[12], args[13], args[14], args[15]) + 1;
          case 17:
            return TargetR(args[0] - 1, args[1], args[2], args[3],
                           args[4], args[5], args[6], args[7],
                           args[8], args[9], args[10], args[11],
                           args[12], args[13], args[14], args[15],
                           args[16]) + 1;
        }
    }

    printf("    TargetR (");
    int i = argc - 1;
    for (; i > 0; i--) {
        printf("%ld,", (DWORD)args[i]);
    }
    printf("%ld)\n", (DWORD)args[0]);

    return 2000 + argc;
}


BOOL WINAPI DllMain(HINSTANCE hinst, DWORD dwReason, LPVOID reserved)
{
    (void)hinst;
    (void)dwReason;
    (void)reserved;

    return TRUE;
}

//
///////////////////////////////////////////////////////////////// End of File.
