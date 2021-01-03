//
//
//
#include <windows.h>

int WINAPI Echo(PCSTR pszMsg)
{
    int sum = 0;
    while (*pszMsg) {
        sum = sum + *pszMsg++;
    }
    return sum;
}

int main()
{
    return 0;
}
