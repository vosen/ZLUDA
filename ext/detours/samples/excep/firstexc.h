//////////////////////////////////////////////////////////////////////////////
//
//  Detours Test Program (firstexc.h of firstexc.exe)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//

#pragma once
#ifndef _FIRSTEXC_H_
#define _FIRSTEXC_H_

/////////////////////////////////////////////// First Chance Exception Filter.
//
LPTOP_LEVEL_EXCEPTION_FILTER WINAPI
DetourFirstChanceExceptionFilter(LPTOP_LEVEL_EXCEPTION_FILTER lpTopLevelFilter);

#endif // _FIRSTEXC_H_
//
////////////////////////////////////////////////////////////////  End of File.
