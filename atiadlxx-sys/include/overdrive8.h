#pragma once
  
  
  
 #ifndef OVERDRIVE8_H_
 #define OVERDRIVE8_H_
  
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
  
  
 ADL_EXTERNC int EXPOSED ADL2_Overdrive8_Init_Setting_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLOD8InitSetting *lpInitSetting);
  
 ADL_EXTERNC int EXPOSED ADL2_Overdrive8_Current_Setting_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLOD8CurrentSetting *lpCurrentSetting);
  
 ADL_EXTERNC int EXPOSED ADL2_Overdrive8_Setting_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLOD8SetSetting *lpSetSetting, ADLOD8CurrentSetting *lpCurrentSetting);
  
  
 ADL_EXTERNC int EXPOSED ADL2_New_QueryPMLogData_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLPMLogDataOutput* lpDataOutput);
  
 ADL_EXTERNC int EXPOSED ADL2_Overdrive8_Init_SettingX2_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int* lpOverdrive8Capabilities, int *lpNumberOfFeatures, ADLOD8SingleInitSetting** lppInitSettingList);
  
 ADL_EXTERNC int EXPOSED ADL2_Overdrive8_Current_SettingX2_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int *lpNumberOfFeatures, int** lppCurrentSettingList);
  
 ADL_EXTERNC int EXPOSED ADL2_AutoTuningResult_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, bool* lpDataOutput);
  
 ADL_EXTERNC int EXPOSED ADL2_Overdrive8_PMLogSenorRange_Caps(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int *lpNumberOfSupportedSensorRange, ADLOD8SingleInitSetting** lppSenorRangeCapsList);
  
 ADL_EXTERNC int EXPOSED ADL2_Overdrive8_PMLogSenorType_Support_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int *lpNumOfSupportedSensorType, int** lppSenroTypesList);
  
 ADL_EXTERNC int EXPOSED ADL2_Overdrive8_PMLog_ShareMemory_Support(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int * lpSupported, int option);
  
 ADL_EXTERNC int EXPOSED ADL2_Overdrive8_PMLog_ShareMemory_Start(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iSampleRate, int iNumofPMLogSendorList, int* lpPMLogSendorList, ADL_D3DKMT_HANDLE* lpHDevice, void** lppSharedMemory, int iOption);
  
  
 ADL_EXTERNC int EXPOSED ADL2_Overdrive8_PMLog_ShareMemory_Read(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iNumSensor, int *lpSensorList, void** lppSharedMemory, ADLPMLogDataOutput* lpDataOutput);
  
 ADL_EXTERNC int EXPOSED ADL2_Overdrive8_PMLog_ShareMemory_Stop(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADL_D3DKMT_HANDLE *lpHDevice);
  
  
 ADL_EXTERNC int EXPOSED ADL2_Device_PMLog_Device_Create(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADL_D3DKMT_HANDLE *pDevice);
  
 ADL_EXTERNC int EXPOSED ADL2_Device_PMLog_Device_Destroy(ADL_CONTEXT_HANDLE context, ADL_D3DKMT_HANDLE hDevice);
  
 ADL_EXTERNC int EXPOSED ADL2_Overdrive8_Current_SettingX3_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int* lpFeatureNotAdjustableBits, int *lpNumberOfSettings, int** lppCurrentSettingList, int iOption);
  
 ADL_EXTERNC int EXPOSED ADL2_Adapter_PMLog_Support_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLPMLogSupportInfo* pPMLogSupportInfo);
  
 ADL_EXTERNC int EXPOSED ADL2_Adapter_PMLog_Start(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLPMLogStartInput* pPMLogStartInput, ADLPMLogStartOutput* pPMLogStartOutput, ADL_D3DKMT_HANDLE hDevice);
  
 ADL_EXTERNC int EXPOSED ADL2_Adapter_PMLog_Stop(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADL_D3DKMT_HANDLE hDevice);
  
  
 ADL_EXTERNC int EXPOSED ADL2_Adapter_PMLog_SensorLimits_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLPMLogSensorLimits* lpDataOutput);
  
  
 #endif /* OVERDRIVE8_H_ */