//////////////////////////////////////////////////////////////////////////////
//
//  Detours Test Program (firstexc.cpp of firstexc.lib)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
//  For more information on exception handling, see "A Crash Course on the
//  Depths of Win32 Structured Exception Handling," by Matt Pietrek in the
//  January 1997 issue of Microsoft Systems Journal.
//
#include <stdio.h>
#include <windows.h>
#include "detours.h"
#include "firstexc.h"

#if _MSC_VER > 1000
#pragma warning(disable: 4740)
#endif

//////////////////////////////////////////////////////////////////////////////
//
static BOOL                         s_bExceptionDetourInstalled = FALSE;
static LPTOP_LEVEL_EXCEPTION_FILTER s_pFirstChanceFilter = NULL;

ULONG (NTAPI *Real_NtContinue)(IN PCONTEXT ContextRecord,
                             IN BOOLEAN TestAlerts) = NULL;

VOID (NTAPI *Real_KiUserExceptionDispatcher)(IN PEXCEPTION_RECORD ExceptionRecord,
                                        IN PCONTEXT ContextFrame) = NULL;

//////////////////////////////////////////////////////////////////////////////
//
//  This function effectively removes all try..catch frames for the current
//  stack.  It forces all exceptions to be treated as unhandled exceptions.
//
#pragma warning(push)
#pragma warning(disable: 4733)
static VOID WINAPI RemoveAllExceptionHandlers(VOID)
{
    // The basic, OS defined exception frame
    struct EXCEPTION_REGISTRATION
    {
        EXCEPTION_REGISTRATION* prev;
        FARPROC handler;
    };

    EXCEPTION_REGISTRATION * pVCExcRec = NULL;
    EXCEPTION_REGISTRATION * pLastGood = NULL;

    __asm mov eax, FS:[0];
    __asm mov [pVCExcRec], eax;

    for (pLastGood = pVCExcRec; (ULONG)pVCExcRec != ~0ul; ) {
        if ((ULONG)pVCExcRec >= 0x30000000)
            break;

        pLastGood = pVCExcRec;
        pVCExcRec = (EXCEPTION_REGISTRATION *)(pVCExcRec->prev);
    }

    __asm mov eax, [pLastGood];
    __asm mov FS:[0], eax;
}
#pragma warning(pop)

//////////////////////////////////////////////////////////////////////////////
// Routine Description:
//
//    This routine is entered on return from kernel mode to dispatch a user
//    mode exception. If a frame based handler handles the exception, then
//    the execution is continued. Else last chance processing is performed.
//
//    NOTE:  This procedure is not called, but rather dispatched to.
//           It depends on there not being a return address on the stack
//           (assumption w.r.t. argument offsets.)
//
// Arguments:
//    ExceptionRecord (esp+0) - Supplies a pointer to an exception record.
//    ContextRecord (esp+4) - Supplies a pointer to a context frame.
//
// Return Value:
//    None.
//
static VOID __declspec(naked) NTAPI
Detour_KiUserExceptionDispatcher(PEXCEPTION_RECORD pExceptRec,
                                 CONTEXT *pContext)
{
    __asm {
        xor     eax, eax                ; // Create fake return address on stack.
        push    eax                     ; // (Generally, we are called by the kernel.)

        push    ebp                     ; // Prolog
        mov     ebp, esp                ;
        sub     esp, __LOCAL_SIZE       ;
    }

    LPTOP_LEVEL_EXCEPTION_FILTER pFirstChanceFilter;
    EXCEPTION_POINTERS ep;
    DWORD dwReturn;
    DWORD dwError;

    ep.ExceptionRecord = pExceptRec;
    ep.ContextRecord = pContext;
    pFirstChanceFilter = s_pFirstChanceFilter;
    dwReturn = EXCEPTION_CONTINUE_SEARCH;
    dwError = 0;

    if (s_pFirstChanceFilter) {
        dwReturn = pFirstChanceFilter(&ep);
    }

    if (dwReturn == EXCEPTION_CONTINUE_EXECUTION) {
        dwError = Real_NtContinue(pContext, 0);
        // This call should *NEVER* return.  If it does, we want to fail to the debugger.
        RemoveAllExceptionHandlers();
    }

    if (dwReturn == EXCEPTION_EXECUTE_HANDLER) {        // Special: Call debugger.
        RemoveAllExceptionHandlers();
    }

    __asm {
        mov     ebx, pExceptRec         ;
        mov     ecx, pContext           ;
        push    ecx                     ;
        push    ebx                     ;
        mov     eax, [Real_KiUserExceptionDispatcher];
        jmp     eax                     ;
        ;
        ; The above code should never return.
        ;
        int     3                       ; // Break!
        ;
        mov     esp, ebp                ; // Epilog
        pop     ebp                     ;
        ret                             ;
    }
}

//////////////////////////////////////////////////////////////////////////////
//
//  Set the first-chance exception filter.
//
//  Returns the pointer to the last first-chance exception filter if there
//  was one.  If this is the first first-chance exception filter, installs
//  the necessary detour and acquires the appropriate function pointers.
//  If the parameter is NULL, first-chance exception filtering is disabled.
//
//  A first-chance exception filter should always return one of three
//  possible codes:
//
//    EXCEPTION_CONTINUE_SEARCH:
//        The exception was not handled by this filter; continue the
//        search for the appropriate exception handler.
//
//    EXCEPTION_CONTINUE_EXECUTION:
//        The exception was handled by this filter; continue execution
//        at the point were the exception was thrown.
//
//    EXCEPTION_EXECUTE_HANDLER:
//        Drastic failure in the exception filter.  Process the
//        exception as if no exception handlers were installed.
//        (i.e. Give the user a chance to invoke the debugger.)
//
LPTOP_LEVEL_EXCEPTION_FILTER WINAPI
DetourFirstChanceExceptionFilter(LPTOP_LEVEL_EXCEPTION_FILTER pNewFirstChanceFilter)
{
    if (!s_bExceptionDetourInstalled) {
        s_bExceptionDetourInstalled = TRUE;

        Real_NtContinue = (ULONG (NTAPI *)(IN PCONTEXT, IN BOOLEAN))
            DetourFindFunction("ntdll.dll", "NtContinue");
        Real_KiUserExceptionDispatcher =
            (VOID (NTAPI *)(IN PEXCEPTION_RECORD, IN PCONTEXT))
            DetourFindFunction("ntdll.dll", "KiUserExceptionDispatcher");

        DetourTransactionBegin();
        DetourUpdateThread(GetCurrentThread());
        DetourAttach(&(PVOID&)Real_KiUserExceptionDispatcher,
                       Detour_KiUserExceptionDispatcher);
        DetourTransactionCommit();
    }

    LPTOP_LEVEL_EXCEPTION_FILTER pOldFirstChanceFilter = s_pFirstChanceFilter;
    s_pFirstChanceFilter = pNewFirstChanceFilter;
    return pOldFirstChanceFilter;
}
//
///////////////////////////////////////////////////////////////// End of File.
