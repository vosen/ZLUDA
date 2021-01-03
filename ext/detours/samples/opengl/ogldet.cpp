//////////////////////////////////////////////////////////////////////////////
//
//  Module:     ogldet.dll
//
//  This DLL is based on the sample simple.dll. A detour is inserted for
//  the OpenGL glFinish function.
//
#include <stdio.h>
#include <windows.h>
#include <GL/gl.h>
#include "detours.h"

static void (WINAPI * trueGlFinish)(void) = glFinish;

void WINAPI hookedGlFinish(void)
{
    printf("ogldet" DETOURS_STRINGIFY(DETOURS_BITS) ".dll:"
           " hookedGlFinish Starting.\n");
    fflush(stdout);

    trueGlFinish();

    printf("ogldet" DETOURS_STRINGIFY(DETOURS_BITS) ".dll:"
           " hookedGlFinish done.\n");
    fflush(stdout);
}

BOOL WINAPI DllMain(HINSTANCE hinst, DWORD dwReason, LPVOID reserved)
{
    LONG error;
    (void)hinst;
    (void)reserved;

    if (DetourIsHelperProcess()) {
        return TRUE;
    }

    if (dwReason == DLL_PROCESS_ATTACH) {
        DetourRestoreAfterWith();

        printf("ogldet" DETOURS_STRINGIFY(DETOURS_BITS) ".dll:"
               " Starting.\n");
        fflush(stdout);

        DetourTransactionBegin();
        DetourUpdateThread(GetCurrentThread());
        DetourAttach(&(PVOID&)trueGlFinish, hookedGlFinish);
        error = DetourTransactionCommit();

        if (error == NO_ERROR) {
            printf("ogldet" DETOURS_STRINGIFY(DETOURS_BITS) ".dll:"
                   " Detoured glFinish().\n");
        }
        else {
            printf("ogldet" DETOURS_STRINGIFY(DETOURS_BITS) ".dll:"
                   " Error detouring glFinish(): %d\n", error);
        }
    }
    else if (dwReason == DLL_PROCESS_DETACH) {
        DetourTransactionBegin();
        DetourUpdateThread(GetCurrentThread());
        DetourDetach(&(PVOID&)trueGlFinish, hookedGlFinish);
        error = DetourTransactionCommit();

        printf("ogldet" DETOURS_STRINGIFY(DETOURS_BITS) ".dll:"
               " Removed detour glFinish() (result=%d)\n", error);
        fflush(stdout);
    }

    return TRUE;
}

//
///////////////////////////////////////////////////////////////// End of File.
