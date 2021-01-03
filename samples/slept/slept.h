//////////////////////////////////////////////////////////////////////////////
//
//  Detour Test Program (slept.h of slept.dll)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
#pragma once

DWORD WINAPI UntimedSleepEx(DWORD dwMilliseconds, BOOL bAlertable);
DWORD WINAPI TimedSleepEx(DWORD dwMilliseconds, BOOL bAlertable);
DWORD WINAPI GetSleptTicks(VOID);
DWORD WINAPI TestTicks(VOID);
DWORD WINAPI TestTicksEx(DWORD Add);

//
///////////////////////////////////////////////////////////////// End of File.
