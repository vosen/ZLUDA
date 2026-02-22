#include <cufftXt.h>
#include <cudalibxt.h>
#define undefined4 unsigned int
#define uint unsigned int
#define longlong unsigned long long
cufftResult CUFFTAPI cufftXtMakePlanGuru64(
    undefined4 param_1,
    int param_2,
    longlong param_3,
    int param_4,
    longlong param_5,
    undefined4 param_6,
    undefined4 param_7,
    undefined4 param_8,
    longlong param_9,
    uint param_10);
cufftResult CUFFTAPI cufftMakePlanGuru64(
    undefined4 param_1,
    int param_2,
    longlong param_3,
    int param_4,
    longlong param_5,
    undefined4 param_6,
    longlong param_7);
cufftResult CUFFTAPI cufftEnterCS(void);
cufftResult CUFFTAPI cufftLeaveCS(void);