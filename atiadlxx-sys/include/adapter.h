#ifndef ADAPTER_H_
#define ADAPTER_H_

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


ADL_EXTERNC int EXPOSED ADL2_Adapter_ObservedClockInfo_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int* lpCoreClock, int* lpMemoryClock);

ADL_EXTERNC int EXPOSED ADL_Adapter_ObservedClockInfo_Get(int iAdapterIndex, int* lpCoreClock, int* lpMemoryClock);

ADL_EXTERNC int EXPOSED ADL_Adapter_ObservedGameClockInfo_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int* lpBaseClock, int* lpGameClock, int* lpBoostClock, int* lpMemoryClock);

ADL_EXTERNC int EXPOSED ADL2_Adapter_Active_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex,
      int iStatus,
      int* lpNewlyActivate);

ADL_EXTERNC int EXPOSED ADL_Adapter_Active_Set(int iAdapterIndex,
      int iStatus,
      int* lpNewlyActivate);

ADL_EXTERNC int EXPOSED ADL2_Adapter_Active_SetPrefer(ADL_CONTEXT_HANDLE context, int iAdapterIndex,
      int iStatus,
      int iNumPreferTarget,
      ADLDisplayTarget* lpPreferTarget,
      int* lpNewlyActivate);

ADL_EXTERNC int EXPOSED ADL_Adapter_Active_SetPrefer(int iAdapterIndex,
      int iStatus,
      int iNumPreferTarget,
      ADLDisplayTarget* lpPreferTarget,
      int* lpNewlyActivate);

ADL_EXTERNC int EXPOSED ADL2_Adapter_Primary_Get(ADL_CONTEXT_HANDLE context, int* lpPrimaryAdapterIndex);

ADL_EXTERNC int EXPOSED ADL_Adapter_Primary_Get(int* lpPrimaryAdapterIndex);

ADL_EXTERNC int EXPOSED ADL2_Adapter_Primary_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex);

ADL_EXTERNC int EXPOSED ADL_Adapter_Primary_Set(int iAdapterIndex);

ADL_EXTERNC int EXPOSED ADL2_Adapter_ModeSwitch(ADL_CONTEXT_HANDLE context, int iAdapterIndex);

ADL_EXTERNC int EXPOSED ADL_Adapter_ModeSwitch(int iAdapterIndex);

ADL_EXTERNC  int EXPOSED ADL2_Adapter_Active_Get (ADL_CONTEXT_HANDLE context, int iAdapterIndex, int* lpStatus);

ADL_EXTERNC  int EXPOSED ADL_Adapter_Active_Get ( int iAdapterIndex, int* lpStatus);


ADL_EXTERNC int EXPOSED ADL2_Adapter_Aspects_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, char* lpAspects, int iSize );

ADL_EXTERNC int EXPOSED ADL_Adapter_Aspects_Get( int iAdapterIndex, char* lpAspects, int iSize );

ADL_EXTERNC int EXPOSED ADL2_Adapter_NumberOfAdapters_Get (ADL_CONTEXT_HANDLE context,  int* lpNumAdapters );

ADL_EXTERNC int EXPOSED ADL_Adapter_NumberOfAdapters_Get (  int* lpNumAdapters );

ADL_EXTERNC int EXPOSED ADL2_Flush_Driver_Data(ADL_CONTEXT_HANDLE context,int iAdapterIndex);

ADL_EXTERNC int EXPOSED ADL_Flush_Driver_Data(int iAdapterIndex);

ADL_EXTERNC int EXPOSED ADL2_Adapter_AdapterInfo_Get (ADL_CONTEXT_HANDLE context,LPAdapterInfo lpInfo, int iInputSize);


ADL_EXTERNC int EXPOSED ADL2_Adapter_VerndorID_Int_get(ADL_CONTEXT_HANDLE context, int iAdapterIndex);

ADL_EXTERNC int EXPOSED ADL_Adapter_AdapterInfo_Get (LPAdapterInfo lpInfo, int iInputSize);

ADL_EXTERNC int EXPOSED ADL2_Adapter_AdapterInfoX2_Get (ADL_CONTEXT_HANDLE context,AdapterInfo **lppAdapterInfo);

ADL_EXTERNC int EXPOSED ADL_Adapter_AdapterInfoX2_Get (AdapterInfo **lppAdapterInfo);

ADL_EXTERNC int EXPOSED ADL_Adapter_RegValueString_Get(int iAdapterIndex, int iDriverPathOption, char* szSubKey, char *szKeyName, int iSize, char *lpKeyValue);

ADL_EXTERNC int EXPOSED ADL_Adapter_RegValueString_Set(int iAdapterIndex, int iDriverPathOption, char* szSubKey, char *szKeyName, int iSize, char *lpKeyValue);

ADL_EXTERNC int EXPOSED ADL2_Adapter_RegValueString_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iDriverPathOption, char* szSubKey, char *szKeyName, int iSize, char *lpKeyValue);

ADL_EXTERNC int EXPOSED ADL2_Adapter_RegValueString_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iDriverPathOption, char* szSubKey, char *szKeyName, int iSize, char *lpKeyValue);

ADL_EXTERNC int EXPOSED ADL2_Adapter_RegValueInt_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iDriverPathOption, char* szSubKey, char *szKeyName, int *lpKeyValue);

ADL_EXTERNC int EXPOSED ADL_Adapter_RegValueInt_Get(int iAdapterIndex, int iDriverPathOption, char* szSubKey, char *szKeyName, int *lpKeyValue);

ADL_EXTERNC int EXPOSED ADL_Adapter_RegValueInt_Set(int iAdapterIndex, int iDriverPathOption, char *szSubKey, char *szKeyName, int iKeyValue);

ADL_EXTERNC int EXPOSED ADL2_Adapter_RegValueInt_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iDriverPathOption, char *szSubKey, char *szKeyName, int iKeyValue);


ADL_EXTERNC int EXPOSED ADL2_Adapter_ASICFamilyType_Get (ADL_CONTEXT_HANDLE context,int iAdapterIndex, int* lpAsicTypes, int* lpValids);

ADL_EXTERNC int EXPOSED ADL_Adapter_ASICFamilyType_Get (int iAdapterIndex, int* lpAsicTypes, int* lpValids);

ADL_EXTERNC int EXPOSED ADL2_Adapter_Speed_Caps (ADL_CONTEXT_HANDLE context,int iAdapterIndex, int* lpCaps, int* lpValid);

ADL_EXTERNC int EXPOSED ADL_Adapter_Speed_Caps (int iAdapterIndex, int* lpCaps, int* lpValid);

ADL_EXTERNC int EXPOSED ADL2_Adapter_Speed_Get (ADL_CONTEXT_HANDLE context,int iAdapterIndex, int *lpCurrent, int *lpDefault);

ADL_EXTERNC int EXPOSED ADL_Adapter_Speed_Get (int iAdapterIndex, int *lpCurrent, int *lpDefault);

ADL_EXTERNC int EXPOSED ADL2_Adapter_Speed_Set (ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iSpeed);

ADL_EXTERNC int EXPOSED ADL_Adapter_Speed_Set (int iAdapterIndex, int iSpeed);

ADL_EXTERNC int EXPOSED ADL2_Adapter_Accessibility_Get (ADL_CONTEXT_HANDLE context,int iAdapterIndex, int  *lpAccessibility); 

ADL_EXTERNC int EXPOSED ADL_Adapter_Accessibility_Get (int iAdapterIndex, int  *lpAccessibility); 

ADL_EXTERNC int EXPOSED ADL2_Adapter_VideoBiosInfo_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLBiosInfo* lpBiosInfo );

ADL_EXTERNC int EXPOSED ADL_Adapter_VideoBiosInfo_Get( int iAdapterIndex, ADLBiosInfo* lpBiosInfo );

ADL_EXTERNC int EXPOSED ADL2_Adapter_ID_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int* lpAdapterID);

ADL_EXTERNC int EXPOSED ADL_Adapter_ID_Get(int iAdapterIndex, int* lpAdapterID);

ADL_EXTERNC int EXPOSED ADL2_AdapterX2_Caps(ADL_CONTEXT_HANDLE context,int iAdapterIndex,  ADLAdapterCapsX2 *adapterCaps);

ADL_EXTERNC int EXPOSED ADL_AdapterX2_Caps(int iAdapterIndex,  ADLAdapterCapsX2 *adapterCaps);

ADL_EXTERNC int EXPOSED ADL2_Stress_Test_Cap(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int * iSupported, int * iEnabled, int * iVersion);

ADL_EXTERNC int EXPOSED ADL2_Throttle_Notification_Cap(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int * iSupported, int * iEnabled, int * iVersion);

ADL_EXTERNC int EXPOSED ADL2_Adapter_AceDefaults_Restore(ADL_CONTEXT_HANDLE context, int iAdapterIndex);

ADL_EXTERNC int EXPOSED ADL2_Adapter_AdapterInfoX4_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int* numAdapters, AdapterInfoX2** lppAdapterInfoX2);

ADL_EXTERNC int EXPOSED ADL2_Adapter_AdapterInfoX3_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int* numAdapters, AdapterInfo** lppAdapterInfo);

ADL_EXTERNC int EXPOSED ADL2_Adapter_AdapterList_Disable(ADL_CONTEXT_HANDLE context, int iNumAdapters, int *lpAdapterIndexList, bool isSkipSaveDB = false);

ADL_EXTERNC int EXPOSED ADL_Adapter_AdapterList_Disable(int iNumAdapters, int *lpAdapterIndexList);

ADL_EXTERNC int EXPOSED ADL_Adapter_BigSw_Info_Get(int iAdapterIndex, int* lpBigSwSupportMajor, int* lpBigSwSupportMinor, int* lpRedStoneSupport);

ADL_EXTERNC int EXPOSED ADL2_Adapter_BigSw_Info_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int* lpBigSwSupportMajor, int* lpBigSwSupportMinor, int* lpRedStoneSupport);

ADL_EXTERNC int EXPOSED ADL2_Adapter_Caps(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLAdapterCaps *adapterCaps);

ADL_EXTERNC int EXPOSED ADL_Adapter_Caps(int iAdapterIndex, ADLAdapterCaps *adapterCaps);

ADL_EXTERNC int EXPOSED ADL2_Adapter_ChipSetInfo_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLChipSetInfo * lpChipSetInfo);

ADL_EXTERNC int EXPOSED ADL_Adapter_ChipSetInfo_Get(int iAdapterIndex, ADLChipSetInfo * lpChipSetInfo);

ADL_EXTERNC int EXPOSED ADL2_Adapter_Feature_Caps(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADL_UIFEATURES_GROUP iFeatureID, int *iIsFeatureSupported);

ADL_EXTERNC int EXPOSED ADL2_Adapter_HBC_Caps(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int *lpHbcCapable);

ADL_EXTERNC int EXPOSED ADL2_Adapter_Headless_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int *lpHeadless);

ADL_EXTERNC int EXPOSED ADL2_Adapter_IsGamingDriver_Info_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int* lpCwgSupport, int* lpIsGamingMode);

ADL_EXTERNC int EXPOSED ADL2_Adapter_MemoryInfo2_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLMemoryInfo2 *lpMemoryInfo2);

ADL_EXTERNC int EXPOSED ADL_Adapter_MemoryInfo2_Get(int iAdapterIndex, ADLMemoryInfo2 *lpMemoryInfo2);

ADL_EXTERNC int EXPOSED ADL2_Adapter_TRNG_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iTRNGSize, int iTRNGBufferSize, char *lpTRNGBuffer);


ADL_EXTERNC int EXPOSED ADL2_Adapter_Modes_ReEnumerate(ADL_CONTEXT_HANDLE context);

ADL_EXTERNC int EXPOSED ADL_Adapter_Modes_ReEnumerate();

ADL_EXTERNC int EXPOSED ADL2_Feature_Settings_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADL_UIFEATURES_GROUP iFeatureID, int *iCurrent);

ADL_EXTERNC int EXPOSED ADL2_Feature_Settings_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADL_UIFEATURES_GROUP iFeatureID, int iCurrent);

ADL_EXTERNC int EXPOSED ADL2_GcnAsicInfo_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLGcnInfo* gcnInfo);

ADL_EXTERNC int EXPOSED ADL2_GPUVMPageSize_Info_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int* iVMPageSizeSupport, int* iVMPageSizeType);

ADL_EXTERNC int EXPOSED ADL2_GPUVMPageSize_Info_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iVMPageSizeType);

ADL_EXTERNC int EXPOSED ADL2_Adapter_VRAMUsage_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int * iVRAMUsageInMB);

ADL_EXTERNC int EXPOSED ADL2_Adapter_DedicatedVRAMUsage_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int * iVRAMUsageInMB);





ADL_EXTERNC int EXPOSED ADL2_Adapter_VideoTheaterModeInfo_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int* lpOverlayDisplayMode, int* lpSavedSettings);

ADL_EXTERNC int EXPOSED ADL_Adapter_VideoTheaterModeInfo_Get(int iAdapterIndex, int* lpOverlayDisplayMode, int* lpSavedSettings);

ADL_EXTERNC int EXPOSED ADL2_Adapter_VideoTheaterModeInfo_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iOverlayDisplayMode, int iSavedSettings);

ADL_EXTERNC int EXPOSED ADL_Adapter_VideoTheaterModeInfo_Set(int iAdapterIndex, int iOverlayDisplayMode, int iSavedSettings);

ADL_EXTERNC int EXPOSED ADL2_MMD_Features_Caps(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLFeatureCaps ** lppFeatureCaps, int * lpFeatureCount);

ADL_EXTERNC int EXPOSED ADL_MMD_Features_Caps(int iAdapterIndex, ADLFeatureCaps ** lppFeatureCaps, int * lpFeatureCount);

ADL_EXTERNC int EXPOSED ADL2_MMD_FeatureValues_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLFeatureValues ** lppFeatureValues, int * lpFeatureCount);

ADL_EXTERNC int EXPOSED ADL_MMD_FeatureValues_Get(int iAdapterIndex, ADLFeatureValues ** lppFeatureValues, int * lpFeatureCount);

ADL_EXTERNC int EXPOSED ADL2_MMD_FeatureValues_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLFeatureValues * lpFeatureValues, int iFeatureCount, int ClientID);

ADL_EXTERNC int EXPOSED ADL_MMD_FeatureValues_Set(int iAdapterIndex, ADLFeatureValues * lpFeatureValues, int iFeatureCount, int ClientID);



#if defined (_WIN32) || defined(_WIN64)

ADL_EXTERNC int EXPOSED ADL2_PageMigration_Settings_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLVirtualSegmentSettingsOutput *lpVirtualSegSettings);

ADL_EXTERNC int EXPOSED ADL2_PageMigration_Settings_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iEnabled, int iNewSize);

#endif /*(_WIN32) || (_WIN64)*/




ADL_EXTERNC int EXPOSED ADL2_Adapter_Crossfire_Caps (ADL_CONTEXT_HANDLE context,int iAdapterIndex, int* lpPreferred, int* lpNumComb, ADLCrossfireComb **ppCrossfireComb);

ADL_EXTERNC int EXPOSED ADL_Adapter_Crossfire_Caps (int iAdapterIndex, int* lpPreferred, int* lpNumComb, ADLCrossfireComb **ppCrossfireComb);

ADL_EXTERNC int EXPOSED ADL2_Adapter_Crossfire_Get (ADL_CONTEXT_HANDLE context,int iAdapterIndex, ADLCrossfireComb *lpCrossfireComb, ADLCrossfireInfo *lpCrossfireInfo);

ADL_EXTERNC int EXPOSED ADL_Adapter_Crossfire_Get (int iAdapterIndex, ADLCrossfireComb *lpCrossfireComb, ADLCrossfireInfo *lpCrossfireInfo);

ADL_EXTERNC int EXPOSED ADL2_Adapter_Crossfire_Set (ADL_CONTEXT_HANDLE context,int iAdapterIndex, ADLCrossfireComb *lpCrossfireComb);

ADL_EXTERNC int EXPOSED ADL_Adapter_Crossfire_Set (int iAdapterIndex, ADLCrossfireComb *lpCrossfireComb);

ADL_EXTERNC int EXPOSED ADL2_Adapter_MVPU_Set (ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iState);

ADL_EXTERNC int EXPOSED ADL2_Adapter_CrossfireX2_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLCrossfireComb *lpCrossfireComb, ADLCrossfireInfo *lpCrossfireInfo);

ADL_EXTERNC int EXPOSED ADL_Adapter_CrossfireX2_Get(int iAdapterIndex, ADLCrossfireComb *lpCrossfireComb, ADLCrossfireInfo *lpCrossfireInfo);




ADL_EXTERNC int EXPOSED ADL2_Adapter_BoardLayout_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int* lpValidFlags, int *lpNumberSlots, ADLBracketSlotInfo** lppBracketSlot, int* lpNumberConnector, ADLConnectorInfo** lppConnector);

ADL_EXTERNC int EXPOSED ADL_Adapter_BoardLayout_Get(int iAdapterIndex, int* lpValidFlags, int *lpNumberSlots, ADLBracketSlotInfo** lppBracketSlot, int* lpNumberConnector, ADLConnectorInfo** lppConnector);


ADL_EXTERNC int EXPOSED ADL2_Adapter_SupportedConnections_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, ADLDevicePort devicePort, ADLSupportedConnections* lpADLSupportedConnections);

ADL_EXTERNC int EXPOSED ADL_Adapter_SupportedConnections_Get(int iAdapterIndex, ADLDevicePort devicePort, ADLSupportedConnections* lpADLSupportedConnections);

ADL_EXTERNC int EXPOSED ADL2_Adapter_ConnectionState_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, ADLDevicePort devicePort, ADLConnectionState* lpADLConnectionState);

ADL_EXTERNC int EXPOSED ADL_Adapter_ConnectionState_Get(int iAdapterIndex, ADLDevicePort devicePort, ADLConnectionState* lpADLConnectionState);

ADL_EXTERNC int EXPOSED ADL2_Adapter_EmulationMode_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex, ADLDevicePort devicePort, int iEmulationMode);

ADL_EXTERNC int EXPOSED ADL_Adapter_EmulationMode_Set(int iAdapterIndex, ADLDevicePort devicePort, int iEmulationMode);

ADL_EXTERNC int EXPOSED ADL2_Adapter_ConnectionData_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex,  ADLDevicePort devicePort, ADLConnectionData ConnectionData);

ADL_EXTERNC int EXPOSED ADL_Adapter_ConnectionData_Set(int iAdapterIndex,  ADLDevicePort devicePort, ADLConnectionData ConnectionData);

ADL_EXTERNC int EXPOSED ADL2_Adapter_ConnectionData_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex,  ADLDevicePort devicePort, int iQueryType, ADLConnectionData* lpConnectionData);

ADL_EXTERNC int EXPOSED ADL_Adapter_ConnectionData_Get(int iAdapterIndex,  ADLDevicePort devicePort, int iQueryType, ADLConnectionData* lpConnectionData);

ADL_EXTERNC int EXPOSED ADL2_Adapter_ConnectionData_Remove(ADL_CONTEXT_HANDLE context,int iAdapterIndex,  ADLDevicePort devicePort);

ADL_EXTERNC int EXPOSED ADL_Adapter_ConnectionData_Remove(int iAdapterIndex,  ADLDevicePort devicePort);

ADL_EXTERNC int EXPOSED ADL2_Adapter_EDIDManagement_Caps(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int *lpSupported);

ADL_EXTERNC int EXPOSED ADL_Adapter_EDIDManagement_Caps(int iAdapterIndex, int *lpSupported);

ADL_EXTERNC int EXPOSED ADL2_Workstation_GlobalEDIDPersistence_Get(ADL_CONTEXT_HANDLE context,int *lpCurResultValue, int *lpDefResultValue);

ADL_EXTERNC int EXPOSED ADL_Workstation_GlobalEDIDPersistence_Get(int *lpCurResultValue, int *lpDefResultValue);

ADL_EXTERNC int EXPOSED ADL2_Workstation_GlobalEDIDPersistence_Set(ADL_CONTEXT_HANDLE context,int iCurState);

ADL_EXTERNC int EXPOSED ADL_Workstation_GlobalEDIDPersistence_Set(int iCurState);



ADL_EXTERNC int EXPOSED ADL2_ElmCompatibilityMode_Caps(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int *lpSupported, int *lpDefault);

ADL_EXTERNC int EXPOSED ADL2_ElmCompatibilityMode_Status_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int *lpValue);

ADL_EXTERNC int EXPOSED ADL2_ElmCompatibilityMode_Status_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iValue);


#if defined (_WIN32) || defined(_WIN64)
ADL_EXTERNC int EXPOSED ADL2_FPS_Caps (ADL_CONTEXT_HANDLE context,int iAdapterIndex, int *lpSupported, int *lpVersion);

ADL_EXTERNC int EXPOSED ADL2_FPS_Settings_Get (ADL_CONTEXT_HANDLE context,int iAdapterIndex, ADLFPSSettingsOutput *lpFPSSettings);

ADL_EXTERNC int EXPOSED ADL2_FPS_Settings_Set (ADL_CONTEXT_HANDLE context,int iAdapterIndex, ADLFPSSettingsInput lpFPSSettings);

ADL_EXTERNC int EXPOSED ADL2_FPS_Settings_Reset (ADL_CONTEXT_HANDLE context,int iAdapterIndex);


ADL_EXTERNC int EXPOSED ADL2_RIS_Settings_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADL_RIS_SETTINGS settings, ADL_RIS_NOTFICATION_REASON changeReason);

ADL_EXTERNC int EXPOSED ADL2_RIS_Settings_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADL_RIS_SETTINGS* settings);

ADL_EXTERNC int EXPOSED ADL2_CHILL_SettingsX2_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADL_CHILL_SETTINGS settings, ADL_CHILL_NOTFICATION_REASON changeReason, ADL_ERROR_REASON* errorReason);

ADL_EXTERNC int EXPOSED ADL2_CHILL_SettingsX2_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADL_CHILL_SETTINGS* settings);


ADL_EXTERNC int EXPOSED ADL2_DELAG_Settings_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADL_DELAG_SETTINGS settings, ADL_DELAG_NOTFICATION_REASON changeReason, ADL_ERROR_REASON* errorReason);

ADL_EXTERNC int EXPOSED ADL2_DELAG_Settings_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADL_DELAG_SETTINGS* settings);

ADL_EXTERNC int EXPOSED ADL2_BOOST_Settings_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADL_BOOST_SETTINGS settings, ADL_BOOST_NOTFICATION_REASON changeReason, ADL_ERROR_REASON* errorReason);

ADL_EXTERNC int EXPOSED ADL2_BOOST_Settings_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADL_BOOST_SETTINGS* settings);

ADL_EXTERNC int EXPOSED ADL2_PROVSR_Settings_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADL_PROVSR_SETTINGS settings, ADL_PROVSR_NOTFICATION_REASON changeReason, ADL_ERROR_REASON* errorReason);


ADL_EXTERNC int EXPOSED ADL2_PROVSR_Settings_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADL_PROVSR_SETTINGS* settings);


ADL_EXTERNC int EXPOSED ADL2_Chill_Settings_Notify(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iChanged);

ADL_EXTERNC int EXPOSED ADL2_Chill_Settings_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iEnabled);

ADL_EXTERNC int EXPOSED ADL2_Chill_Settings_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int* lpEnabled);

ADL_EXTERNC int EXPOSED ADL2_Chill_Caps_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int* iSupported, int* iCheckCaps);

ADL_EXTERNC int EXPOSED ADL2_PerformanceTuning_Caps(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int *lpSupported, int *lpDefault);

ADL_EXTERNC int EXPOSED ADL2_PerfTuning_Status_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int *lpPTuningValue);

ADL_EXTERNC int EXPOSED ADL2_PerfTuning_Status_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int lpPTuningValue);

ADL_EXTERNC int EXPOSED ADL2_PPW_Caps(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int *lpSupported, int *lpDefault);

ADL_EXTERNC int EXPOSED ADL2_PPW_Status_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int *lpFPWValue);

ADL_EXTERNC int EXPOSED ADL2_PPW_Status_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iFPWValue);

#endif /*(_WIN32) || (_WIN64)*/

ADL_EXTERNC int EXPOSED ADL2_Adapter_FrameMetrics_Caps(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int *iIsFrameMonitorSupported);

ADL_EXTERNC int EXPOSED ADL2_Adapter_FrameMetrics_Start(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int VidPnSourceId);

ADL_EXTERNC int EXPOSED ADL2_Adapter_FrameMetrics_Stop(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int VidPnSourceId);

ADL_EXTERNC int EXPOSED ADL2_Adapter_FrameMetrics_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int VidPnSourceId, float *iFramesPerSecond);

ADL_EXTERNC int EXPOSED ADL2_Adapter_FrameMetrics_FrameDuration_Enable(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADL_FRAME_DURATION_HANDLE* frameDurationHandle);

ADL_EXTERNC int EXPOSED ADL2_Adapter_FrameMetrics_FrameDuration_Disable(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADL_FRAME_DURATION_HANDLE* frameDurationHandle);

ADL_EXTERNC int EXPOSED ADL2_Adapter_FrameMetrics_FrameDuration_Start(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int VidPnSourceId);

ADL_EXTERNC int EXPOSED ADL2_Adapter_FrameMetrics_FrameDuration_Stop(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int VidPnSourceId);

ADL_EXTERNC int EXPOSED ADL2_Adapter_FrameMetrics_FrameDuration_Get(ADL_CONTEXT_HANDLE context, ADL_FRAME_DURATION_HANDLE frameDurationHandle, unsigned long long * pFrameDurationsArr, unsigned int frameDurationsArrSize, unsigned int *elementsCopied);

// Deprecated APIs

ADL_EXTERNC int EXPOSED ADL2_Adapter_ClockInfo_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, ADLClockInfo* lpClockInfo);

ADL_EXTERNC int EXPOSED ADL_Adapter_ClockInfo_Get(int iAdapterIndex, ADLClockInfo* lpClockInfo);

ADL_EXTERNC int EXPOSED ADL2_Display_AdapterID_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int* lpAdapterID);

ADL_EXTERNC int EXPOSED ADL_Display_AdapterID_Get(int iAdapterIndex, int* lpAdapterID);

ADL_EXTERNC int EXPOSED ADL2_Adapter_EDC_ErrorRecords_Get (ADL_CONTEXT_HANDLE context,int iAdapterIndex, int* pErrorrecordCount, ADLErrorRecord* errorRecords); 

ADL_EXTERNC int EXPOSED ADL2_Adapter_EDC_ErrorInjection_Set (ADL_CONTEXT_HANDLE context,int iAdapterIndex, ADLErrorInjection* errorInjection);


#endif /* ADAPTER_H_ */
