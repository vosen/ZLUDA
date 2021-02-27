#ifndef OVERDRIVE5_H_
#define OVERDRIVE5_H_

#include "adl_structures.h"

#ifndef ADL_EXTERNC
#ifdef __cplusplus
#define ADL_EXTERNC extern "C"
#else
#define ADL_EXTERNC
#endif
#endif

#ifndef EXPOSED
#define EXPOSED
#endif /* EXPOSED */


ADL_EXTERNC int EXPOSED ADL2_Overdrive5_CurrentActivity_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, ADLPMActivity *lpActivity);

ADL_EXTERNC int EXPOSED ADL_Overdrive5_CurrentActivity_Get(int iAdapterIndex, ADLPMActivity *lpActivity);

ADL_EXTERNC int EXPOSED ADL2_Overdrive5_ThermalDevices_Enum(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iThermalControllerIndex, ADLThermalControllerInfo *lpThermalControllerInfo);

ADL_EXTERNC int EXPOSED ADL_Overdrive5_ThermalDevices_Enum(int iAdapterIndex, int iThermalControllerIndex, ADLThermalControllerInfo *lpThermalControllerInfo);

ADL_EXTERNC int EXPOSED ADL2_Overdrive5_Temperature_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iThermalControllerIndex, ADLTemperature *lpTemperature);

ADL_EXTERNC int EXPOSED ADL_Overdrive5_Temperature_Get(int iAdapterIndex, int iThermalControllerIndex, ADLTemperature *lpTemperature);

ADL_EXTERNC int EXPOSED ADL2_Overdrive5_FanSpeedInfo_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iThermalControllerIndex, ADLFanSpeedInfo *lpFanSpeedInfo);

ADL_EXTERNC int EXPOSED ADL_Overdrive5_FanSpeedInfo_Get(int iAdapterIndex, int iThermalControllerIndex, ADLFanSpeedInfo *lpFanSpeedInfo);

ADL_EXTERNC int EXPOSED ADL2_Overdrive5_FanSpeed_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iThermalControllerIndex, ADLFanSpeedValue *lpFanSpeedValue);

ADL_EXTERNC int EXPOSED ADL_Overdrive5_FanSpeed_Get(int iAdapterIndex, int iThermalControllerIndex, ADLFanSpeedValue *lpFanSpeedValue);

ADL_EXTERNC int EXPOSED ADL2_Overdrive5_FanSpeed_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iThermalControllerIndex, ADLFanSpeedValue *lpFanSpeedValue);

ADL_EXTERNC int EXPOSED ADL_Overdrive5_FanSpeed_Set(int iAdapterIndex, int iThermalControllerIndex, ADLFanSpeedValue *lpFanSpeedValue);

ADL_EXTERNC int EXPOSED ADL2_Overdrive5_FanSpeedToDefault_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iThermalControllerIndex);

ADL_EXTERNC int EXPOSED ADL_Overdrive5_FanSpeedToDefault_Set(int iAdapterIndex, int iThermalControllerIndex);

ADL_EXTERNC int EXPOSED ADL2_Overdrive5_ODParameters_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, ADLODParameters *lpOdParameters);

ADL_EXTERNC int EXPOSED ADL_Overdrive5_ODParameters_Get(int iAdapterIndex, ADLODParameters *lpOdParameters);

ADL_EXTERNC int EXPOSED ADL2_Overdrive5_ODPerformanceLevels_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDefault, ADLODPerformanceLevels *lpOdPerformanceLevels);

ADL_EXTERNC int EXPOSED ADL_Overdrive5_ODPerformanceLevels_Get(int iAdapterIndex, int iDefault, ADLODPerformanceLevels *lpOdPerformanceLevels);

ADL_EXTERNC int EXPOSED ADL2_Overdrive5_ODPerformanceLevels_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex, ADLODPerformanceLevels *lpOdPerformanceLevels);

ADL_EXTERNC int EXPOSED ADL_Overdrive5_ODPerformanceLevels_Set(int iAdapterIndex, ADLODPerformanceLevels *lpOdPerformanceLevels);


ADL_EXTERNC int EXPOSED ADL2_Overdrive5_PowerControl_Caps(ADL_CONTEXT_HANDLE context,int iAdapterIndex,  int *lpSupported);


ADL_EXTERNC int EXPOSED ADL_Overdrive5_PowerControl_Caps(int iAdapterIndex,  int *lpSupported);

ADL_EXTERNC int EXPOSED ADL2_Overdrive5_PowerControlInfo_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, ADLPowerControlInfo *lpPowerControlInfo);


ADL_EXTERNC int EXPOSED ADL_Overdrive5_PowerControlInfo_Get(int iAdapterIndex, ADLPowerControlInfo *lpPowerControlInfo);

ADL_EXTERNC int EXPOSED ADL2_Overdrive5_PowerControl_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int *lpCurrentValue, int *lpDefaultValue);


ADL_EXTERNC int EXPOSED ADL_Overdrive5_PowerControl_Get(int iAdapterIndex, int *lpCurrentValue, int *lpDefaultValue);

ADL_EXTERNC int EXPOSED ADL2_Overdrive5_PowerControl_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iValue);


ADL_EXTERNC int EXPOSED ADL_Overdrive5_PowerControl_Set(int iAdapterIndex, int iValue);

ADL_EXTERNC int EXPOSED ADL2_Overdrive_Caps (ADL_CONTEXT_HANDLE context,int iAdapterIndex, int * iSupported, int * iEnabled, int * iVersion );

ADL_EXTERNC int EXPOSED ADL_Overdrive_Caps (int iAdapterIndex, int * iSupported, int * iEnabled, int * iVersion );


#endif /* OVERDRIVE5_H_ */
