//
//
//
#include <windows.h>

int WINAPI Echo(PCSTR pszMsg);

extern "C" int __stdcall mainCRTStartup(HINSTANCE hInstance,
                             HINSTANCE hPrevInstance,
                             LPSTR lpCmdLine,
                             int nCmdShow
                            )
{
    (void)hInstance;
    (void)hPrevInstance;
    (void)lpCmdLine;
    (void)nCmdShow;

    Echo("Hello World");
    Echo("Goodbye World");

    return 0x99;
}

