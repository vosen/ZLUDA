#ifndef OVERDRIVE6_H_
#define OVERDRIVE6_H_

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


ADL_EXTERNC int EXPOSED ADL2_Overdrive6_Capabilities_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, ADLOD6Capabilities *lpODCapabilities);


ADL_EXTERNC int EXPOSED ADL_Overdrive6_Capabilities_Get(int iAdapterIndex, ADLOD6Capabilities *lpODCapabilities);

ADL_EXTERNC int EXPOSED ADL2_Overdrive6_StateInfo_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iStateType, ADLOD6StateInfo *lpStateInfo);


ADL_EXTERNC int EXPOSED ADL_Overdrive6_StateInfo_Get(int iAdapterIndex, int iStateType, ADLOD6StateInfo *lpStateInfo);

ADL_EXTERNC int EXPOSED ADL2_Overdrive6_State_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iStateType, ADLOD6StateInfo *lpStateInfo);


ADL_EXTERNC int EXPOSED ADL_Overdrive6_State_Set(int iAdapterIndex, int iStateType, ADLOD6StateInfo *lpStateInfo);

ADL_EXTERNC int EXPOSED ADL2_Overdrive6_State_Reset(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iStateType);


ADL_EXTERNC int EXPOSED ADL_Overdrive6_State_Reset(int iAdapterIndex, int iStateType);

ADL_EXTERNC int EXPOSED ADL2_Overdrive6_CurrentStatus_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, ADLOD6CurrentStatus *lpCurrentStatus);


ADL_EXTERNC int EXPOSED ADL_Overdrive6_CurrentStatus_Get(int iAdapterIndex, ADLOD6CurrentStatus *lpCurrentStatus);

ADL_EXTERNC int EXPOSED ADL2_Overdrive6_ThermalController_Caps(ADL_CONTEXT_HANDLE context,int iAdapterIndex, ADLOD6ThermalControllerCaps *lpThermalControllerCaps);


ADL_EXTERNC int EXPOSED ADL_Overdrive6_ThermalController_Caps(int iAdapterIndex, ADLOD6ThermalControllerCaps *lpThermalControllerCaps);

ADL_EXTERNC int EXPOSED ADL2_Overdrive6_Temperature_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int *lpTemperature);


ADL_EXTERNC int EXPOSED ADL_Overdrive6_Temperature_Get(int iAdapterIndex, int *lpTemperature);

ADL_EXTERNC int EXPOSED ADL2_Overdrive6_FanSpeed_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, ADLOD6FanSpeedInfo *lpFanSpeedInfo);


ADL_EXTERNC int EXPOSED ADL_Overdrive6_FanSpeed_Get(int iAdapterIndex, ADLOD6FanSpeedInfo *lpFanSpeedInfo);

ADL_EXTERNC int EXPOSED ADL2_Overdrive6_FanSpeed_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex, ADLOD6FanSpeedValue *lpFanSpeedValue);


ADL_EXTERNC int EXPOSED ADL_Overdrive6_FanSpeed_Set(int iAdapterIndex, ADLOD6FanSpeedValue *lpFanSpeedValue);

ADL_EXTERNC int EXPOSED ADL2_Overdrive6_FanSpeed_Reset(ADL_CONTEXT_HANDLE context,int iAdapterIndex);


ADL_EXTERNC int EXPOSED ADL_Overdrive6_FanSpeed_Reset(int iAdapterIndex);

ADL_EXTERNC int EXPOSED ADL2_Overdrive6_PowerControl_Caps (ADL_CONTEXT_HANDLE context,int iAdapterIndex, int *lpSupported);


ADL_EXTERNC int EXPOSED ADL_Overdrive6_PowerControl_Caps (int iAdapterIndex, int *lpSupported);

ADL_EXTERNC int EXPOSED ADL2_Overdrive6_PowerControlInfo_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, ADLOD6PowerControlInfo *lpPowerControlInfo);


ADL_EXTERNC int EXPOSED ADL_Overdrive6_PowerControlInfo_Get(int iAdapterIndex, ADLOD6PowerControlInfo *lpPowerControlInfo);

ADL_EXTERNC int EXPOSED ADL2_Overdrive6_PowerControl_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int *lpCurrentValue, int *lpDefaultValue);


ADL_EXTERNC int EXPOSED ADL_Overdrive6_PowerControl_Get(int iAdapterIndex, int *lpCurrentValue, int *lpDefaultValue);

ADL_EXTERNC int EXPOSED ADL2_Overdrive6_PowerControl_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iValue);

ADL_EXTERNC int EXPOSED ADL_Overdrive6_PowerControl_Set(int iAdapterIndex, int iValue);


ADL_EXTERNC int EXPOSED ADL2_Overdrive6_VoltageControlInfo_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLOD6VoltageControlInfo *lpVoltageControlInfo);



ADL_EXTERNC int EXPOSED ADL_Overdrive6_VoltageControlInfo_Get(int iAdapterIndex, ADLOD6VoltageControlInfo *lpVoltageControlInfo);

ADL_EXTERNC int EXPOSED ADL2_Overdrive6_VoltageControl_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int *lpCurrentValue, int *lpDefaultValue);


ADL_EXTERNC int EXPOSED ADL_Overdrive6_VoltageControl_Get(int iAdapterIndex, int *lpCurrentValue, int *lpDefaultValue);

ADL_EXTERNC int EXPOSED ADL2_Overdrive6_VoltageControl_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iValue);



ADL_EXTERNC int EXPOSED ADL_Overdrive6_VoltageControl_Set(int iAdapterIndex, int iValue);

ADL_EXTERNC int EXPOSED ADL2_Overdrive6_CapabilitiesEx_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLOD6CapabilitiesEx *lpODCapabilities);

ADL_EXTERNC int EXPOSED ADL_Overdrive6_CapabilitiesEx_Get(int iAdapterIndex, ADLOD6CapabilitiesEx *lpODCapabilities);

ADL_EXTERNC int EXPOSED ADL2_Overdrive6_StateEx_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iStateType, ADLOD6StateEx *lpODState);

ADL_EXTERNC int EXPOSED ADL_Overdrive6_StateEx_Get(int iAdapterIndex, int iStateType, ADLOD6StateEx *lpODState);

ADL_EXTERNC int EXPOSED ADL2_Overdrive6_StateEx_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iStateType, ADLOD6StateEx *lpODState);

ADL_EXTERNC int EXPOSED ADL_Overdrive6_StateEx_Set(int iAdapterIndex, int iStateType, ADLOD6StateEx *lpODState);

ADL_EXTERNC int EXPOSED ADL_Overdrive6_ThermalLimitUnlock_Set(int iAdapterIndex, int iStateType, int iEnable);

ADL_EXTERNC int EXPOSED ADL2_Overdrive6_ThermalLimitUnlock_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iStateType, int iEnable);

ADL_EXTERNC int EXPOSED ADL_Overdrive6_ThermalLimitUnlock_Get(int iAdapterIndex, int iStateType, int* pEnabled);

ADL_EXTERNC int EXPOSED ADL2_Overdrive6_ThermalLimitUnlock_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iStateType, int* pEnabled);

ADL_EXTERNC int EXPOSED ADL_Overdrive6_AdvancedFan_Caps (int iAdapterIndex, int *lpSupported);

ADL_EXTERNC int EXPOSED ADL2_Overdrive6_AdvancedFan_Caps (ADL_CONTEXT_HANDLE context, int iAdapterIndex, int *lpSupported);

ADL_EXTERNC int EXPOSED ADL_Overdrive6_TargetTemperatureRangeInfo_Get(int iAdapterIndex, ADLOD6ParameterRange *lpTargetTemperatureInfo);

ADL_EXTERNC int EXPOSED ADL2_Overdrive6_TargetTemperatureRangeInfo_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLOD6ParameterRange *lpTargetTemperatureInfo);

ADL_EXTERNC int EXPOSED ADL_Overdrive6_TargetTemperatureData_Get(int iAdapterIndex, int *lpCurrentValue, int *lpDefaultValue);

ADL_EXTERNC int EXPOSED ADL2_Overdrive6_TargetTemperatureData_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int *lpCurrentValue, int *lpDefaultValue);

ADL_EXTERNC int EXPOSED ADL_Overdrive6_TargetTemperatureData_Set(int iAdapterIndex, int iCurrentValue);

ADL_EXTERNC int EXPOSED ADL2_Overdrive6_TargetTemperatureData_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iCurrentValue);

ADL_EXTERNC int EXPOSED ADL_Overdrive6_FanPWMLimitRangeInfo_Get(int iAdapterIndex, ADLOD6ParameterRange *lpFanPWMLimitInfo);

ADL_EXTERNC int EXPOSED ADL2_Overdrive6_FanPWMLimitRangeInfo_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLOD6ParameterRange *lpFanPWMLimitInfo);

ADL_EXTERNC int EXPOSED ADL_Overdrive6_FanPWMLimitData_Get(int iAdapterIndex, int *lpCurrentValue, int *lpDefaultValue);

ADL_EXTERNC int EXPOSED ADL2_Overdrive6_FanPWMLimitData_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int *lpCurrentValue, int *lpDefaultValue);

ADL_EXTERNC int EXPOSED ADL_Overdrive6_FanPWMLimitData_Set(int iAdapterIndex, int iCurrentValue);

ADL_EXTERNC int EXPOSED ADL2_Overdrive6_FanPWMLimitData_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iCurrentValue);

ADL_EXTERNC int EXPOSED ADL2_Overdrive6_CurrentPower_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iPowerType, int *lpCurrentValue);

ADL_EXTERNC int EXPOSED ADL2_Overdrive6_ControlI2C(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iControl);

ADL_EXTERNC int EXPOSED ADL2_Overdrive6_FuzzyController_Caps(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int *lpSupported);

ADL_EXTERNC int EXPOSED ADL_Overdrive6_FuzzyController_Caps(int iAdapterIndex, int *lpSupported);


#endif /* OVERDRIVE6_H_ */

