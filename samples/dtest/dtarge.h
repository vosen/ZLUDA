//////////////////////////////////////////////////////////////////////////////
//
//  Detours Test Program (dtarge.h of dtarge.dll)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
#pragma once

#if (_MSC_VER < 1299)
typedef DWORD DWORD_PTR;
#endif

DWORD_PTR WINAPI Target0();
DWORD_PTR WINAPI Target1(DWORD_PTR v1);
DWORD_PTR WINAPI Target2(DWORD_PTR v1, DWORD_PTR v2);
DWORD_PTR WINAPI Target3(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3);
DWORD_PTR WINAPI Target4(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4);
DWORD_PTR WINAPI Target5(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
                         DWORD_PTR v5);
DWORD_PTR WINAPI Target6(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
                         DWORD_PTR v5, DWORD_PTR v6);
DWORD_PTR WINAPI Target7(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
                         DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7);
DWORD_PTR WINAPI Target8(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
                         DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7, DWORD_PTR v8);
DWORD_PTR WINAPI Target9(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
                         DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7, DWORD_PTR v8,
                         DWORD_PTR v9);
DWORD_PTR WINAPI Target10(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
                          DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7, DWORD_PTR v8,
                          DWORD_PTR v9, DWORD_PTR v10);
DWORD_PTR WINAPI Target11(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
                          DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7, DWORD_PTR v8,
                          DWORD_PTR v9, DWORD_PTR v10, DWORD_PTR v11);
DWORD_PTR WINAPI Target12(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
                          DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7, DWORD_PTR v8,
                          DWORD_PTR v9, DWORD_PTR v10, DWORD_PTR v11, DWORD_PTR v12);
DWORD_PTR WINAPI Target13(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
                          DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7, DWORD_PTR v8,
                          DWORD_PTR v9, DWORD_PTR v10, DWORD_PTR v11, DWORD_PTR v12,
                          DWORD_PTR v13);
DWORD_PTR WINAPI Target14(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
                          DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7, DWORD_PTR v8,
                          DWORD_PTR v9, DWORD_PTR v10, DWORD_PTR v11, DWORD_PTR v12,
                          DWORD_PTR v13, DWORD_PTR v14);
DWORD_PTR WINAPI Target15(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
                          DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7, DWORD_PTR v8,
                          DWORD_PTR v9, DWORD_PTR v10, DWORD_PTR v11, DWORD_PTR v12,
                          DWORD_PTR v13, DWORD_PTR v14, DWORD_PTR v15);
DWORD_PTR WINAPI Target16(DWORD_PTR v1, DWORD_PTR v2, DWORD_PTR v3, DWORD_PTR v4,
                          DWORD_PTR v5, DWORD_PTR v6, DWORD_PTR v7, DWORD_PTR v8,
                          DWORD_PTR v9, DWORD_PTR v10, DWORD_PTR v11, DWORD_PTR v12,
                          DWORD_PTR v13, DWORD_PTR v14, DWORD_PTR v15, DWORD_PTR v16);
DWORD_PTR WINAPI TargetV(DWORD_PTR v1, ...);
DWORD_PTR WINAPI TargetR(DWORD_PTR v1, ...);

//
///////////////////////////////////////////////////////////////// End of File.
