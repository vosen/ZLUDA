//
//
//
#include <windows.h>
#include <detours.h>
#include <stdio.h>

int WINAPI Echo(PCSTR pszMsg);

static int (WINAPI * Real_Echo)(PCSTR pszMsg) = Echo;

int WINAPI Mine_Echo(PCSTR pszMsg)
{
    printf("Echo(%s)\n", pszMsg);
    return Real_Echo(pszMsg);
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

        printf("echofx" DETOURS_STRINGIFY(DETOURS_BITS) ".dll:"
               " Starting.\n");
        fflush(stdout);

        DetourTransactionBegin();
        DetourUpdateThread(GetCurrentThread());
        DetourAttach(&(PVOID&)Real_Echo, Mine_Echo);
        error = DetourTransactionCommit();

        if (error == NO_ERROR) {
            printf("echofx" DETOURS_STRINGIFY(DETOURS_BITS) ".dll:"
                   " Detoured Echo().\n");
        }
        else {
            printf("echofx" DETOURS_STRINGIFY(DETOURS_BITS) ".dll:"
                   " Error detouring Echo(): %ld\n", error);
        }
    }
    else if (dwReason == DLL_PROCESS_DETACH) {
        DetourTransactionBegin();
        DetourUpdateThread(GetCurrentThread());
        DetourDetach(&(PVOID&)Real_Echo, Mine_Echo);
        error = DetourTransactionCommit();

        printf("echofx" DETOURS_STRINGIFY(DETOURS_BITS) ".dll:"
               " Removed Echo() (result=%ld)\n", error);
        fflush(stdout);
    }
    return TRUE;
}
