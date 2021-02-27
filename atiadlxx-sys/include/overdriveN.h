#ifndef OVERDRIVEN_H_
#define OVERDRIVEN_H_

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

ADL_EXTERNC int EXPOSED ADL2_OverdriveN_Capabilities_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLODNCapabilities *lpODCapabilities);

ADL_EXTERNC int EXPOSED ADL2_OverdriveN_CapabilitiesX2_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLODNCapabilitiesX2 *lpODCapabilities);

ADL_EXTERNC int EXPOSED ADL2_OverdriveN_SystemClocks_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLODNPerformanceLevels *lpODPerformanceLevels);

ADL_EXTERNC int EXPOSED ADL2_OverdriveN_SystemClocks_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLODNPerformanceLevels *lpODPerformanceLevels);


ADL_EXTERNC int EXPOSED ADL2_OverdriveN_SystemClocksX2_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLODNPerformanceLevelsX2 *lpODPerformanceLevels);

ADL_EXTERNC int EXPOSED ADL2_OverdriveN_SystemClocksX2_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLODNPerformanceLevelsX2 *lpODPerformanceLevels);

ADL_EXTERNC int EXPOSED ADL2_OverdriveN_MemoryClocksX2_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLODNPerformanceLevelsX2 *lpODPerformanceLevels);

ADL_EXTERNC int EXPOSED ADL2_OverdriveN_MemoryClocksX2_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLODNPerformanceLevelsX2 *lpODPerformanceLevels);

ADL_EXTERNC int EXPOSED ADL2_OverdriveN_MemoryClocks_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLODNPerformanceLevels *lpODPerformanceLevels);

ADL_EXTERNC int EXPOSED ADL2_OverdriveN_MemoryClocks_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLODNPerformanceLevels *lpODPerformanceLevels);

ADL_EXTERNC int EXPOSED ADL2_OverdriveN_FanControl_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLODNFanControl *lpODFanSpeed);

ADL_EXTERNC int EXPOSED ADL2_OverdriveN_FanControl_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLODNFanControl *lpODFanControl);

ADL_EXTERNC int EXPOSED ADL2_OverdriveN_PowerLimit_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLODNPowerLimitSetting *lpODPowerLimit);

ADL_EXTERNC int EXPOSED ADL2_OverdriveN_PowerLimit_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLODNPowerLimitSetting *lpODPowerLimit);

ADL_EXTERNC int EXPOSED ADL2_OverdriveN_Temperature_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iTemperatureType, int *iTemperature);

ADL_EXTERNC int EXPOSED ADL2_OverdriveN_PerformanceStatus_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLODNPerformanceStatus *lpODPerformanceStatus);

// Custom fan for WS ODN vega 10
ADL_EXTERNC int EXPOSED ADL2_CustomFan_Caps(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int *lpSupported);

ADL_EXTERNC int EXPOSED ADL2_CustomFan_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLODNFanControl *lpODFanControl);

ADL_EXTERNC int EXPOSED ADL2_CustomFan_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLODNFanControl *lpODFanControl);

ADL_EXTERNC int EXPOSED ADL2_OverdriveN_MemoryTimingLevel_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int *lpSupport, int *lpCurrentValue, int *lpDefaultValue, int *lpNumberLevels, int **lppLevelList);


ADL_EXTERNC int EXPOSED ADL2_OverdriveN_MemoryTimingLevel_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int currentValue);

ADL_EXTERNC int EXPOSED ADL2_OverdriveN_ZeroRPMFan_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int *lpSupport, int *lpCurrentValue, int *lpDefaultValue);

ADL_EXTERNC int EXPOSED ADL2_OverdriveN_ZeroRPMFan_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int currentValue);



ADL_EXTERNC int EXPOSED ADL2_OverdriveN_SettingsExt_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int* lpOverdriveNExtCapabilities, int *lpNumberOfODNExtFeatures, ADLODNExtSingleInitSetting** lppInitSettingList, int** lppCurrentSettingList);

ADL_EXTERNC int EXPOSED ADL2_OverdriveN_SettingsExt_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iNumberOfODNExtFeatures,int* itemValueValidList, int* lpItemValueList);

ADL_EXTERNC int EXPOSED ADL2_OverdriveN_AutoWattman_Caps(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int *lpSupported, int *lpDefault);

ADL_EXTERNC int EXPOSED ADL2_OverdriveN_AutoWattman_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int *lpCurrent);

ADL_EXTERNC int EXPOSED ADL2_OverdriveN_AutoWattman_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iCurrent);

ADL_EXTERNC int EXPOSED ADL2_OverdriveN_CountOfEvents_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int eventcounterType, int *eventCount);

ADL_EXTERNC int EXPOSED ADL2_OverdriveN_SCLKAutoOverClock_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int *lpStatus);

ADL_EXTERNC int EXPOSED ADL2_OverdriveN_SCLKAutoOverClock_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iStatus, int *iFlags);

ADL_EXTERNC int EXPOSED ADL2_OverdriveN_Test_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iEnabled);

ADL_EXTERNC int EXPOSED ADL2_OverdriveN_ThrottleNotification_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int *lpStatus, int *lpThrottleFlags);


#endif /* OVERDRIVEN_H_ */
