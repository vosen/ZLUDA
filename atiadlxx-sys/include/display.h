#ifndef DISPLAY_H_
#define DISPLAY_H_


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



ADL_EXTERNC int EXPOSED ADL2_Display_VirtualType_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iDisplayIndex, ADL_VIRTUALDISPLAY_TYPE *iVirtualType);

ADL_EXTERNC int EXPOSED ADL2_Display_WriteAndReadI2CLargePayload(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLI2CLargePayload *plI2C);

ADL_EXTERNC int EXPOSED ADL_Display_WriteAndReadI2CLargePayload(int iAdapterIndex, ADLI2CLargePayload *plI2C);

ADL_EXTERNC int EXPOSED ADL2_Display_HDCP_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iDisplayIndex, ADLHDCPSettings *lpHDCPSettings);

ADL_EXTERNC int EXPOSED ADL2_Display_HDCP_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iDisplayIndex, int iSetToDefault, int iEnable);

ADL_EXTERNC int EXPOSED ADL2_Display_HDRState_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLDisplayID displayID, int *iSupport, int *iEnable);

ADL_EXTERNC int EXPOSED ADL2_Display_HDRState_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLDisplayID displayID, int iEnable);

ADL_EXTERNC int EXPOSED ADL2_Display_Limits_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iDisplayIndex, int* lpMaxHRes, int* lpMaxVRes, int* lpMaxRefresh);

ADL_EXTERNC int EXPOSED ADL2_Display_PreferredMode_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iDisplayIndex, int* lpNumModes, ADLMode** lppModes);

ADL_EXTERNC int EXPOSED ADL_Display_Limits_Get(int iAdapterIndex, int iDisplayIndex, int* lpMaxHRes, int* lpMaxVRes, int* lpMaxRefresh);

ADL_EXTERNC int EXPOSED ADL2_Display_DisplayInfo_Get (ADL_CONTEXT_HANDLE context,int iAdapterIndex, 
                                                                                 int* lpNumDisplays, 
                                                                                 ADLDisplayInfo ** lppInfo, 
                                                                                 int iForceDetect);

ADL_EXTERNC int EXPOSED ADL_Display_DisplayInfo_Get (int iAdapterIndex, 
                                                                                 int* lpNumDisplays, 
                                                                                 ADLDisplayInfo ** lppInfo, 
                                                                                 int iForceDetect);

ADL_EXTERNC int EXPOSED ADL2_Display_DpMstInfo_Get (ADL_CONTEXT_HANDLE context,int iAdapterIndex, 
                                         int* lpNumDisplays, 
                                         ADLDisplayDPMSTInfo ** lppDisplayDPMstInfo,
                                                                                 int iForceDetect);

ADL_EXTERNC int EXPOSED ADL_Display_DpMstInfo_Get (int iAdapterIndex, 
                                         int* lpNumDisplays, 
                                         ADLDisplayDPMSTInfo ** lppDisplayDPMstInfo,
                                                                                 int iForceDetect);


ADL_EXTERNC int EXPOSED ADL2_Display_NumberOfDisplays_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int *lpNumDisplays);

ADL_EXTERNC int EXPOSED ADL_Display_NumberOfDisplays_Get(int iAdapterIndex, int *lpNumDisplays);

ADL_EXTERNC int EXPOSED ADL2_Display_PreservedAspectRatio_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int* lpSupport, int * lpCurrent, int * lpDefault);

ADL_EXTERNC int EXPOSED ADL_Display_PreservedAspectRatio_Get(int iAdapterIndex, int iDisplayIndex, int* lpSupport, int * lpCurrent, int * lpDefault);


ADL_EXTERNC int EXPOSED ADL2_Display_PreservedAspectRatio_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex,  int iCurrent);

ADL_EXTERNC int EXPOSED ADL_Display_PreservedAspectRatio_Set(int iAdapterIndex, int iDisplayIndex,  int iCurrent);

ADL_EXTERNC int EXPOSED ADL2_Display_ImageExpansion_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int* lpSupport, int * lpCurrent, int * lpDefault);

ADL_EXTERNC int EXPOSED ADL_Display_ImageExpansion_Get(int iAdapterIndex, int iDisplayIndex, int* lpSupport, int * lpCurrent, int * lpDefault);

ADL_EXTERNC int EXPOSED ADL2_Display_ImageExpansion_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int iCurrent);

ADL_EXTERNC int EXPOSED ADL_Display_ImageExpansion_Set(int iAdapterIndex, int iDisplayIndex, int iCurrent);

ADL_EXTERNC int EXPOSED ADL2_Display_Position_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int* lpX, int* lpY, int* lpXDefault, int* lpYDefault, 
                                                                        int* lpMinX, int* lpMinY, int* lpMaxX, int*lpMaxY, int* lpStepX, int* lpStepY);

ADL_EXTERNC int EXPOSED ADL_Display_Position_Get(int iAdapterIndex, int iDisplayIndex, int* lpX, int* lpY, int* lpXDefault, int* lpYDefault, 
                                                                        int* lpMinX, int* lpMinY, int* lpMaxX, int*lpMaxY, int* lpStepX, int* lpStepY);


ADL_EXTERNC int EXPOSED ADL2_Display_Position_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int iX, int iY);

ADL_EXTERNC int EXPOSED ADL_Display_Position_Set(int iAdapterIndex, int iDisplayIndex, int iX, int iY);

ADL_EXTERNC int EXPOSED ADL2_Display_Size_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int* lpWidth, int* lpHeight, int* lpDefaultWidth, int* lpDefaultHeight, 
                                                                        int* lpMinWidth, int* lpMinHeight, int* lpMaxWidth, int*lpMaxHeight, int* lpStepWidth, int* lpStepHeight);

ADL_EXTERNC int EXPOSED ADL_Display_Size_Get(int iAdapterIndex, int iDisplayIndex, int* lpWidth, int* lpHeight, int* lpDefaultWidth, int* lpDefaultHeight, 
                                                                        int* lpMinWidth, int* lpMinHeight, int* lpMaxWidth, int*lpMaxHeight, int* lpStepWidth, int* lpStepHeight);

ADL_EXTERNC int EXPOSED ADL2_Display_Size_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int iWidth, int iHeight);

ADL_EXTERNC int EXPOSED ADL_Display_Size_Set(int iAdapterIndex, int iDisplayIndex, int iWidth, int iHeight);

ADL_EXTERNC int EXPOSED ADL2_Display_AdjustCaps_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int* lpInfo);

ADL_EXTERNC int EXPOSED ADL_Display_AdjustCaps_Get(int iAdapterIndex, int iDisplayIndex, int* lpInfo);

ADL_EXTERNC int EXPOSED ADL2_Display_Capabilities_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int* lpNumberOfControlers, int* lpNumberOfDisplays);

ADL_EXTERNC int EXPOSED ADL_Display_Capabilities_Get(int iAdapterIndex, int* lpNumberOfControlers, int* lpNumberOfDisplays);

ADL_EXTERNC int EXPOSED ADL2_Display_ConnectedDisplays_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int * lpConnections);

ADL_EXTERNC int EXPOSED ADL_Display_ConnectedDisplays_Get(int iAdapterIndex, int * lpConnections);

ADL_EXTERNC int EXPOSED ADL2_Display_DeviceConfig_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, ADLDisplayConfig * lpDisplayConfig);

ADL_EXTERNC int EXPOSED ADL_Display_DeviceConfig_Get(int iAdapterIndex, int iDisplayIndex, ADLDisplayConfig * lpDisplayConfig);

ADL_EXTERNC int EXPOSED ADL2_Display_Property_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, ADLDisplayProperty * lpDisplayProperty);

ADL_EXTERNC int EXPOSED ADL_Display_Property_Get(int iAdapterIndex, int iDisplayIndex, ADLDisplayProperty * lpDisplayProperty);

ADL_EXTERNC int EXPOSED ADL2_Display_Property_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, ADLDisplayProperty * lpDisplayProperty);

ADL_EXTERNC int EXPOSED ADL_Display_Property_Set(int iAdapterIndex, int iDisplayIndex, ADLDisplayProperty * lpDisplayProperty);


ADL_EXTERNC int EXPOSED ADL2_Display_SwitchingCapability_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int *lpResult);

ADL_EXTERNC int EXPOSED ADL_Display_SwitchingCapability_Get(int iAdapterIndex, int *lpResult);

ADL_EXTERNC int EXPOSED ADL2_Display_DitherState_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int *lpDitherState);

ADL_EXTERNC int EXPOSED ADL_Display_DitherState_Get(int iAdapterIndex, int iDisplayIndex, int *lpDitherState);

ADL_EXTERNC int EXPOSED ADL2_Display_DitherState_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int iDitherState);

ADL_EXTERNC int EXPOSED ADL_Display_DitherState_Set(int iAdapterIndex, int iDisplayIndex, int iDitherState);

ADL_EXTERNC int EXPOSED ADL2_Display_SupportedPixelFormat_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int *lpPixelFormat); 

ADL_EXTERNC int EXPOSED ADL_Display_SupportedPixelFormat_Get(int iAdapterIndex, int iDisplayIndex, int *lpPixelFormat); 

ADL_EXTERNC int EXPOSED ADL2_Display_PixelFormat_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int *lpPixelFormat);

ADL_EXTERNC int EXPOSED ADL_Display_PixelFormat_Get(int iAdapterIndex, int iDisplayIndex, int *lpPixelFormat);

ADL_EXTERNC int EXPOSED ADL2_Display_PixelFormatDefault_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iDisplayIndex, int *lpDefPixelFormat);

ADL_EXTERNC int EXPOSED ADL2_Display_PixelFormat_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int iPixelFormat);

ADL_EXTERNC int EXPOSED ADL_Display_PixelFormat_Set(int iAdapterIndex, int iDisplayIndex, int iPixelFormat);

ADL_EXTERNC int EXPOSED ADL2_Display_SupportedColorDepth_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iDisplayIndex, int *lpColorDepth);

ADL_EXTERNC int EXPOSED ADL_Display_SupportedColorDepth_Get(int iAdapterIndex, int iDisplayIndex, int *lpColorDepth);

ADL_EXTERNC int EXPOSED ADL2_Display_ColorDepth_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iDisplayIndex, int *lpColorDepth);

ADL_EXTERNC int EXPOSED ADL_Display_ColorDepth_Get(int iAdapterIndex, int iDisplayIndex, int *lpColorDepth);

ADL_EXTERNC int EXPOSED ADL2_Display_ColorDepth_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iDisplayIndex, int iColorDepth);

ADL_EXTERNC int EXPOSED ADL_Display_ColorDepth_Set(int iAdapterIndex, int iDisplayIndex, int iColorDepth);

ADL_EXTERNC int EXPOSED ADL2_Display_ODClockInfo_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, ADLAdapterODClockInfo *lpOdClockInfo);

ADL_EXTERNC int EXPOSED ADL_Display_ODClockInfo_Get(int iAdapterIndex, ADLAdapterODClockInfo *lpOdClockInfo);

ADL_EXTERNC int EXPOSED ADL2_Display_ODClockConfig_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex, ADLAdapterODClockConfig *lpOdClockConfig);

ADL_EXTERNC int EXPOSED ADL_Display_ODClockConfig_Set(int iAdapterIndex, ADLAdapterODClockConfig *lpOdClockConfig);

ADL_EXTERNC int EXPOSED ADL2_Display_AdjustmentCoherent_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int *lpAdjustmentCoherentCurrent, int *lpAdjustmentCoherentDefault);

ADL_EXTERNC int EXPOSED ADL_Display_AdjustmentCoherent_Get(int iAdapterIndex, int iDisplayIndex, int *lpAdjustmentCoherentCurrent, int *lpAdjustmentCoherentDefault);

ADL_EXTERNC int EXPOSED ADL2_Display_AdjustmentCoherent_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int iAdjustmentCoherent);

ADL_EXTERNC int EXPOSED ADL_Display_AdjustmentCoherent_Set(int iAdapterIndex, int iDisplayIndex, int iAdjustmentCoherent);

ADL_EXTERNC int EXPOSED ADL2_Display_ReducedBlanking_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int *lpReducedBlankingCurrent, int *lpReducedBlankingDefault);

ADL_EXTERNC int EXPOSED ADL_Display_ReducedBlanking_Get(int iAdapterIndex, int iDisplayIndex, int *lpReducedBlankingCurrent, int *lpReducedBlankingDefault);

ADL_EXTERNC int EXPOSED ADL2_Display_ReducedBlanking_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int iReducedBlanking);

ADL_EXTERNC int EXPOSED ADL_Display_ReducedBlanking_Set(int iAdapterIndex, int iDisplayIndex, int iReducedBlanking);

ADL_EXTERNC int EXPOSED ADL2_Display_FormatsOverride_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int* lpSettingsSupported, int* lpSettingsSupportedEx, int* lpCurSettings);

ADL_EXTERNC int EXPOSED ADL_Display_FormatsOverride_Get(int iAdapterIndex, int iDisplayIndex, int* lpSettingsSupported, int* lpSettingsSupportedEx, int* lpCurSettings);

ADL_EXTERNC int EXPOSED ADL2_Display_FormatsOverride_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int iOverrideSettings);

ADL_EXTERNC int EXPOSED ADL_Display_FormatsOverride_Set(int iAdapterIndex, int iDisplayIndex, int iOverrideSettings);

ADL_EXTERNC int EXPOSED ADL2_Display_MVPUCaps_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, ADLMVPUCaps *lpMvpuCaps);

ADL_EXTERNC int EXPOSED ADL_Display_MVPUCaps_Get(int iAdapterIndex, ADLMVPUCaps *lpMvpuCaps);

ADL_EXTERNC int EXPOSED ADL2_Display_MVPUStatus_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, ADLMVPUStatus *lpMvpuStatus);

ADL_EXTERNC int EXPOSED ADL_Display_MVPUStatus_Get(int iAdapterIndex, ADLMVPUStatus *lpMvpuStatus);

ADL_EXTERNC int EXPOSED ADL2_Display_DummyVirtual_Get(ADL_CONTEXT_HANDLE context, int iVirtualDisplayType, int* iTargetID);

ADL_EXTERNC int EXPOSED ADL2_Display_DummyVirtual_Destroy(ADL_CONTEXT_HANDLE context, int iTargetID);

ADL_EXTERNC int EXPOSED ADL2_Adapter_VariBright_Caps(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int * iSupported, int * iEnabled, int * iVersion);

ADL_EXTERNC int EXPOSED ADL_Adapter_VariBright_Caps(int iAdapterIndex, int * iSupported, int * iEnabled, int * iVersion);

ADL_EXTERNC int EXPOSED ADL2_Adapter_VariBrightEnable_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iEnabled);

ADL_EXTERNC int EXPOSED ADL_Adapter_VariBrightEnable_Set(int iAdapterIndex, int iEnabled);

ADL_EXTERNC int EXPOSED ADL2_Adapter_VariBrightLevel_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int * iDefaultLevel, int * iNumberOfLevels, int * iStep, int * iCurrentLevel);

ADL_EXTERNC int EXPOSED ADL_Adapter_VariBrightLevel_Get(int iAdapterIndex, int * iDefaultLevel, int * iNumberOfLevels, int * iStep, int * iCurrentLevel);

ADL_EXTERNC int EXPOSED ADL2_Adapter_VariBrightLevel_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iCurrentLevel, int iApplyImmediately);

ADL_EXTERNC int EXPOSED ADL_Adapter_VariBrightLevel_Set(int iAdapterIndex, int iCurrentLevel, int iApplyImmediately);




ADL_EXTERNC int EXPOSED ADL2_Display_ControllerOverlayAdjustmentCaps_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, ADLControllerOverlayInput *lpOverlayInput, ADLControllerOverlayInfo *lpCapsInfo);

ADL_EXTERNC int EXPOSED ADL_Display_ControllerOverlayAdjustmentCaps_Get(int iAdapterIndex, ADLControllerOverlayInput *lpOverlayInput, ADLControllerOverlayInfo *lpCapsInfo);

ADL_EXTERNC int EXPOSED ADL2_Display_ControllerOverlayAdjustmentData_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, ADLControllerOverlayInput * lpOverlay);

ADL_EXTERNC int EXPOSED ADL_Display_ControllerOverlayAdjustmentData_Get(int iAdapterIndex, ADLControllerOverlayInput * lpOverlay);

ADL_EXTERNC int EXPOSED ADL2_Display_ControllerOverlayAdjustmentData_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex, ADLControllerOverlayInput * lpOverlay);

ADL_EXTERNC int EXPOSED ADL_Display_ControllerOverlayAdjustmentData_Set(int iAdapterIndex, ADLControllerOverlayInput * lpOverlay);






ADL_EXTERNC int EXPOSED ADL2_Display_ViewPort_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, ADLControllerMode * lpControllerMode);

ADL_EXTERNC int EXPOSED ADL_Display_ViewPort_Set(int iAdapterIndex, int iDisplayIndex, ADLControllerMode * lpControllerMode);

ADL_EXTERNC int EXPOSED ADL2_Display_ViewPort_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, ADLControllerMode * lpControllerMode);

ADL_EXTERNC int EXPOSED ADL_Display_ViewPort_Get(int iAdapterIndex, int iDisplayIndex, ADLControllerMode * lpControllerMode);

ADL_EXTERNC int EXPOSED ADL2_Display_ViewPort_Cap(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int* lpSupported);

ADL_EXTERNC int EXPOSED ADL_Display_ViewPort_Cap(int iAdapterIndex, int* lpSupported);




ADL_EXTERNC int EXPOSED ADL2_Display_WriteAndReadI2CRev_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int *lpMajor, int *lpMinor);

ADL_EXTERNC int EXPOSED ADL_Display_WriteAndReadI2CRev_Get(int iAdapterIndex, int *lpMajor, int *lpMinor);

ADL_EXTERNC int EXPOSED ADL2_Display_WriteAndReadI2C(ADL_CONTEXT_HANDLE context,int iAdapterIndex, ADLI2C *plI2C);

ADL_EXTERNC int EXPOSED ADL_Display_WriteAndReadI2C(int iAdapterIndex, ADLI2C *plI2C);

ADL_EXTERNC int EXPOSED ADL2_Display_DDCBlockAccess_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int iOption, int iCommandIndex,int iSendMsgLen, char *lpucSendMsgBuf, int *lpulRecvMsgLen, char *lpucRecvMsgBuf);

ADL_EXTERNC int EXPOSED ADL_Display_DDCBlockAccess_Get(int iAdapterIndex, int iDisplayIndex, int iOption, int iCommandIndex,int iSendMsgLen, char *lpucSendMsgBuf, int *lpulRecvMsgLen, char *lpucRecvMsgBuf);

ADL_EXTERNC int EXPOSED ADL2_Display_DDCInfo_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, ADLDDCInfo* lpInfo);

ADL_EXTERNC int EXPOSED ADL_Display_DDCInfo_Get(int iAdapterIndex, int iDisplayIndex, ADLDDCInfo* lpInfo);

ADL_EXTERNC int EXPOSED ADL2_Display_DDCInfo2_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, ADLDDCInfo2* lpInfo);

ADL_EXTERNC int EXPOSED ADL_Display_DDCInfo2_Get(int iAdapterIndex, int iDisplayIndex, ADLDDCInfo2* lpInfo);


ADL_EXTERNC int EXPOSED ADL2_Display_EdidData_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, ADLDisplayEDIDData *lpEDIDData);

ADL_EXTERNC int EXPOSED ADL_Display_EdidData_Get(int iAdapterIndex, int iDisplayIndex, ADLDisplayEDIDData *lpEDIDData);




ADL_EXTERNC int EXPOSED ADL2_Display_ColorCaps_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int* lpCaps, int* lpValids);

ADL_EXTERNC int EXPOSED ADL_Display_ColorCaps_Get(int iAdapterIndex, int iDisplayIndex, int* lpCaps, int* lpValids);

ADL_EXTERNC int EXPOSED ADL2_Display_Color_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int iColorType, int iCurrent);

ADL_EXTERNC int EXPOSED ADL_Display_Color_Set(int iAdapterIndex, int iDisplayIndex, int iColorType, int iCurrent);

ADL_EXTERNC int EXPOSED ADL2_Display_Color_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int iColorType, int* lpCurrent, int* lpDefault, int* lpMin, int* lpMax, int* lpStep);

ADL_EXTERNC int EXPOSED ADL_Display_Color_Get(int iAdapterIndex, int iDisplayIndex, int iColorType, int* lpCurrent, int* lpDefault, int* lpMin, int* lpMax, int* lpStep);

ADL_EXTERNC int EXPOSED ADL2_Display_ColorTemperatureSource_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int *lpTempSource);

ADL_EXTERNC int EXPOSED ADL_Display_ColorTemperatureSource_Get(int iAdapterIndex, int iDisplayIndex, int *lpTempSource);

ADL_EXTERNC int EXPOSED ADL2_Display_ColorTemperatureSourceDefault_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int *lpTempSourceDefault);

ADL_EXTERNC int EXPOSED ADL2_Display_ColorTemperatureSource_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int iTempSource);

ADL_EXTERNC int EXPOSED ADL_Display_ColorTemperatureSource_Set(int iAdapterIndex, int iDisplayIndex, int iTempSource);

ADL_EXTERNC int EXPOSED ADL2_Display_Gamut_Caps(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iDisplayIndex, ADLGamutReference gamut, ADLGamutInfo *lpCap);

ADL_EXTERNC int EXPOSED ADL_Display_Gamut_Caps(int iAdapterIndex, int iDisplayIndex, ADLGamutReference gamut, ADLGamutInfo *lpCap);

ADL_EXTERNC int EXPOSED ADL2_Display_Gamut_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iDisplayIndex, ADLGamutReference gamut, ADLGamutData *lpSource);

ADL_EXTERNC int EXPOSED ADL_Display_Gamut_Get(int iAdapterIndex, int iDisplayIndex, ADLGamutReference gamut, ADLGamutData *lpSource);

ADL_EXTERNC int EXPOSED ADL2_Display_Gamut_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iDisplayIndex, ADLGamutReference gamut, const ADLGamutData *lpSource);

ADL_EXTERNC int EXPOSED ADL_Display_Gamut_Set(int iAdapterIndex, int iDisplayIndex, ADLGamutReference gamut, const ADLGamutData *lpSource);




ADL_EXTERNC int EXPOSED ADL2_Display_ModeTimingOverride_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, ADLDisplayMode *lpModeIn, ADLDisplayModeInfo *lpModeInfoOut);

ADL_EXTERNC int EXPOSED ADL_Display_ModeTimingOverride_Get(int iAdapterIndex, int iDisplayIndex, ADLDisplayMode *lpModeIn, ADLDisplayModeInfo *lpModeInfoOut);

ADL_EXTERNC int EXPOSED ADL2_Display_ModeTimingOverride_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, ADLDisplayModeInfo *lpMode, int iForceUpdate);

ADL_EXTERNC int EXPOSED ADL_Display_ModeTimingOverride_Set(int iAdapterIndex, int iDisplayIndex, ADLDisplayModeInfo *lpMode, int iForceUpdate);

ADL_EXTERNC int EXPOSED ADL2_Display_ModeTimingOverrideList_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int iMaxNumOfOverrides, ADLDisplayModeInfo *lpModeInfoList, int *lpNumOfOverrides);

ADL_EXTERNC int EXPOSED ADL_Display_ModeTimingOverrideList_Get(int iAdapterIndex, int iDisplayIndex, int iMaxNumOfOverrides, ADLDisplayModeInfo *lpModeInfoList, int *lpNumOfOverrides);

ADL_EXTERNC int EXPOSED ADL2_Display_ModeTimingOverrideX2_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iDisplayIndex, ADLDisplayModeInfoX2 *lpMode, int iForceUpdate);

ADL_EXTERNC int EXPOSED ADL2_Display_ModeTimingOverrideX3_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLDisplayID displayID, ADLDisplayModeX2 *lpModeIn, ADLDisplayModeInfoX2 *lpModeInfoOut);

ADL_EXTERNC int EXPOSED ADL2_Display_ModeTimingOverrideListX3_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLDisplayID displayID, int *lpNumOfModes, ADLDisplayModeInfoX2 **lpModeInfoList);

ADL_EXTERNC int EXPOSED ADL2_Adapter_ModeTimingOverride_Caps(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int *lpSupported);

ADL_EXTERNC int EXPOSED ADL_Adapter_ModeTimingOverride_Caps(int iAdapterIndex, int *lpSupported);

ADL_EXTERNC int EXPOSED ADL2_Display_ModeTimingOverrideX2_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLDisplayID displayID, ADLDisplayModeX2 *lpModeIn, ADLDisplayModeInfo *lpModeInfoOut);

ADL_EXTERNC int EXPOSED ADL_Display_ModeTimingOverrideX2_Get(int iAdapterIndex, ADLDisplayID displayID, ADLDisplayModeX2 *lpModeIn, ADLDisplayModeInfo *lpModeInfoOut);

ADL_EXTERNC int EXPOSED ADL2_Display_ModeTimingOverrideListX2_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLDisplayID displayID, int *lpNumOfModes, ADLDisplayModeInfo **lpModeInfoList);

ADL_EXTERNC int EXPOSED ADL_Display_ModeTimingOverrideListX2_Get(int iAdapterIndex, ADLDisplayID displayID, int *lpNumOfModes, ADLDisplayModeInfo **lpModeInfoList);

ADL_EXTERNC int EXPOSED ADL2_Display_ModeTimingOverride_Delete(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLDisplayID displayID, ADLDisplayModeX2 *lpMode, int iForceUpdate);

ADL_EXTERNC int EXPOSED ADL_Display_ModeTimingOverride_Delete(int iAdapterIndex, ADLDisplayID displayID, ADLDisplayModeX2 *lpMode, int iForceUpdate);




ADL_EXTERNC int EXPOSED ADL2_Display_CustomizedModeListNum_Get (ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int* lpListNum);

ADL_EXTERNC int EXPOSED ADL_Display_CustomizedModeListNum_Get (int iAdapterIndex, int iDisplayIndex, int* lpListNum);

ADL_EXTERNC int EXPOSED ADL2_Display_CustomizedModeList_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex,  ADLCustomMode* lpCustomModeList, int iBuffSize);

ADL_EXTERNC int EXPOSED ADL_Display_CustomizedModeList_Get(int iAdapterIndex, int iDisplayIndex,  ADLCustomMode* lpCustomModeList, int iBuffSize);

ADL_EXTERNC int EXPOSED ADL2_Display_CustomizedMode_Add (ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, ADLCustomMode customMode);

ADL_EXTERNC int EXPOSED ADL_Display_CustomizedMode_Add (int iAdapterIndex, int iDisplayIndex, ADLCustomMode customMode);

ADL_EXTERNC int EXPOSED ADL2_Display_CustomizedMode_Delete (ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int iIndex);

ADL_EXTERNC int EXPOSED ADL_Display_CustomizedMode_Delete (int iAdapterIndex, int iDisplayIndex, int iIndex);

ADL_EXTERNC int EXPOSED ADL2_Display_CustomizedMode_Validate(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, ADLCustomMode customMode, int *lpValid);

ADL_EXTERNC int EXPOSED ADL_Display_CustomizedMode_Validate(int iAdapterIndex, int iDisplayIndex, ADLCustomMode customMode, int *lpValid);





ADL_EXTERNC int EXPOSED ADL2_Display_UnderscanSupport_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int *lpSupport);

ADL_EXTERNC int EXPOSED ADL2_Display_UnderscanState_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int *lpCurrent, int *lpDefault);

ADL_EXTERNC int EXPOSED ADL2_Display_UnderscanState_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int iUnderscanEnabled);

ADL_EXTERNC int EXPOSED ADL2_Display_Underscan_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int iCurrent);

ADL_EXTERNC int EXPOSED ADL_Display_Underscan_Set(int iAdapterIndex, int iDisplayIndex, int iCurrent);

ADL_EXTERNC int EXPOSED ADL2_Display_Underscan_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int* lpCurrent, int* lpDefault, int* lpMin, int* lpMax, int* lpStep);
 
ADL_EXTERNC int EXPOSED ADL_Display_Underscan_Get(int iAdapterIndex, int iDisplayIndex, int* lpCurrent, int* lpDefault, int* lpMin, int* lpMax, int* lpStep);

ADL_EXTERNC int EXPOSED ADL2_Display_Overscan_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int iCurrent);

ADL_EXTERNC int EXPOSED ADL_Display_Overscan_Set(int iAdapterIndex, int iDisplayIndex, int iCurrent);





ADL_EXTERNC int EXPOSED ADL2_Display_Overscan_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int *lpCurrent, int *lpDefualt, int *lpMin, int *lpMax, int *lpStep);

ADL_EXTERNC int EXPOSED ADL_Display_Overscan_Get(int iAdapterIndex, int iDisplayIndex, int *lpCurrent, int *lpDefualt, int *lpMin, int *lpMax, int *lpStep);




ADL_EXTERNC int EXPOSED ADL2_DFP_BaseAudioSupport_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex,int* lpSupport);

ADL_EXTERNC int EXPOSED ADL_DFP_BaseAudioSupport_Get(int iAdapterIndex, int iDisplayIndex,int* lpSupport);

ADL_EXTERNC int EXPOSED ADL2_DFP_HDMISupport_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex,int* lpSupport);

ADL_EXTERNC int EXPOSED ADL_DFP_HDMISupport_Get(int iAdapterIndex, int iDisplayIndex,int* lpSupport);

ADL_EXTERNC int EXPOSED ADL2_DFP_MVPUAnalogSupport_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex,int* lpSupport);

ADL_EXTERNC int EXPOSED ADL_DFP_MVPUAnalogSupport_Get(int iAdapterIndex, int iDisplayIndex,int* lpSupport);

ADL_EXTERNC int EXPOSED ADL2_DFP_PixelFormat_Caps(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int *lpValidBits, int *lpValidCaps);

ADL_EXTERNC int EXPOSED ADL_DFP_PixelFormat_Caps(int iAdapterIndex, int iDisplayIndex, int *lpValidBits, int *lpValidCaps);

ADL_EXTERNC int EXPOSED ADL2_DFP_PixelFormat_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int *lpCurState, int *lpDefault);

ADL_EXTERNC int EXPOSED ADL_DFP_PixelFormat_Get(int iAdapterIndex, int iDisplayIndex, int *lpCurState, int *lpDefault);

ADL_EXTERNC int EXPOSED ADL2_DFP_PixelFormat_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int iState);

ADL_EXTERNC int EXPOSED ADL_DFP_PixelFormat_Set(int iAdapterIndex, int iDisplayIndex, int iState);

ADL_EXTERNC int EXPOSED ADL2_DFP_GPUScalingEnable_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int *lpSupport, int *lpCurrent, int *lpDefault);

ADL_EXTERNC int EXPOSED ADL_DFP_GPUScalingEnable_Get(int iAdapterIndex, int iDisplayIndex, int *lpSupport, int *lpCurrent, int *lpDefault);

ADL_EXTERNC int EXPOSED ADL2_DFP_GPUScalingEnable_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int iCurrent);

ADL_EXTERNC int EXPOSED ADL_DFP_GPUScalingEnable_Set(int iAdapterIndex, int iDisplayIndex, int iCurrent);

ADL_EXTERNC int EXPOSED ADL2_DFP_AllowOnlyCETimings_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int *lpSupport, int *lpCurrent, int *lpDefault);

ADL_EXTERNC int EXPOSED ADL_DFP_AllowOnlyCETimings_Get(int iAdapterIndex, int iDisplayIndex, int *lpSupport, int *lpCurrent, int *lpDefault);

ADL_EXTERNC int EXPOSED ADL2_DFP_AllowOnlyCETimings_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int iCurrent);

ADL_EXTERNC int EXPOSED ADL_DFP_AllowOnlyCETimings_Set(int iAdapterIndex, int iDisplayIndex, int iCurrent);





ADL_EXTERNC int EXPOSED ADL2_Display_TVCaps_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int* lpcaps);

ADL_EXTERNC int EXPOSED ADL_Display_TVCaps_Get(int iAdapterIndex, int iDisplayIndex, int* lpcaps);

ADL_EXTERNC int EXPOSED ADL2_TV_Standard_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int iCurrent);

ADL_EXTERNC int EXPOSED ADL_TV_Standard_Set(int iAdapterIndex, int iDisplayIndex, int iCurrent);

ADL_EXTERNC int EXPOSED ADL2_TV_Standard_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int *lpCurrent, int *lpDefault, int* lpSupportedStandards);

ADL_EXTERNC int EXPOSED ADL_TV_Standard_Get(int iAdapterIndex, int iDisplayIndex, int *lpCurrent, int *lpDefault, int* lpSupportedStandards);



ADL_EXTERNC int EXPOSED ADL2_CV_DongleSettings_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int* lpDongleSetting, int* lpOverrideSettingsSupported, int* lpCurOverrideSettings);

ADL_EXTERNC int EXPOSED ADL_CV_DongleSettings_Get(int iAdapterIndex, int iDisplayIndex, int* lpDongleSetting, int* lpOverrideSettingsSupported, int* lpCurOverrideSettings);

ADL_EXTERNC int EXPOSED ADL2_CV_DongleSettings_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int iOverrideSettings);

ADL_EXTERNC int EXPOSED ADL_CV_DongleSettings_Set(int iAdapterIndex, int iDisplayIndex, int iOverrideSettings);

ADL_EXTERNC int EXPOSED ADL2_CV_DongleSettings_Reset(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex);

ADL_EXTERNC int EXPOSED ADL_CV_DongleSettings_Reset(int iAdapterIndex, int iDisplayIndex);


ADL_EXTERNC int EXPOSED ADL2_Display_UnderScan_Auto_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int *lpCurrent, int *lpDefault, int *lpMin, int *lpMax, int *lpStep);

ADL_EXTERNC int EXPOSED ADL_Display_UnderScan_Auto_Get(int iAdapterIndex, int iDisplayIndex, int *lpCurrent, int *lpDefault, int *lpMin, int *lpMax, int *lpStep);

ADL_EXTERNC int EXPOSED ADL2_Display_UnderScan_Auto_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int iCurrent);

ADL_EXTERNC int EXPOSED ADL_Display_UnderScan_Auto_Set(int iAdapterIndex, int iDisplayIndex, int iCurrent);

ADL_EXTERNC int EXPOSED ADL2_Display_Deflicker_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int *lpCurrent, int *lpDefault, int *lpMin, int *lpMax, int *lpStep);

ADL_EXTERNC int EXPOSED ADL_Display_Deflicker_Get(int iAdapterIndex, int iDisplayIndex, int *lpCurrent, int *lpDefault, int *lpMin, int *lpMax, int *lpStep);

ADL_EXTERNC int EXPOSED ADL2_Display_Deflicker_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayindex, int iCurrent);

ADL_EXTERNC int EXPOSED ADL_Display_Deflicker_Set(int iAdapterIndex, int iDisplayindex, int iCurrent);

ADL_EXTERNC int EXPOSED ADL2_Display_FilterSVideo_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int *lpCurrent, int *lpDefault, int *lpMin, int *lpMax, int *lpStep);

ADL_EXTERNC int EXPOSED ADL_Display_FilterSVideo_Get(int iAdapterIndex, int iDisplayIndex, int *lpCurrent, int *lpDefault, int *lpMin, int *lpMax, int *lpStep);

ADL_EXTERNC int EXPOSED ADL2_Display_FilterSVideo_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int iCurrent);

ADL_EXTERNC int EXPOSED ADL_Display_FilterSVideo_Set(int iAdapterIndex, int iDisplayIndex, int iCurrent);

ADL_EXTERNC int EXPOSED ADL2_Display_DisplayContent_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int iContent);

ADL_EXTERNC int EXPOSED ADL_Display_DisplayContent_Set(int iAdapterIndex, int iDisplayIndex, int iContent);

ADL_EXTERNC int EXPOSED ADL2_Display_DisplayContent_Get(ADL_CONTEXT_HANDLE context,int iAdapterIndex,int iDisplayIndex,int* piContent);

ADL_EXTERNC int EXPOSED ADL_Display_DisplayContent_Get(int iAdapterIndex,int iDisplayIndex,int* piContent);

ADL_EXTERNC int EXPOSED ADL2_Display_DisplayContent_Cap(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int* pCapContent);

ADL_EXTERNC int EXPOSED ADL_Display_DisplayContent_Cap(int iAdapterIndex, int iDisplayIndex, int* pCapContent);

ADL_EXTERNC int EXPOSED ADL2_Display_TargetTiming_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLDisplayID displayID, ADLDisplayModeInfo *lpModeInfoOut);

ADL_EXTERNC int EXPOSED ADL2_Display_TargetTimingX2_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADLDisplayID displayID, ADLDisplayModeInfoX2 *lpModeInfoOut);

ADL_EXTERNC int EXPOSED ADL_Display_TargetTiming_Get(int iAdapterIndex, ADLDisplayID displayID, ADLDisplayModeInfo *lpModeInfoOut);

ADL_EXTERNC int EXPOSED ADL_Display_Downscaling_Caps(int iAdapterIndex, int iDisplayID, int* lpCaps);


ADL_EXTERNC int EXPOSED ADL2_Display_Downscaling_Caps(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iDisplayID, int* lpCaps);


ADL_EXTERNC int EXPOSED ADL_Display_FreeSyncState_Get(int iAdapterIndex, int iDisplayIndex, int *lpCurrent, int *lpDefault, int *lpMinRefreshRateInMicroHz, int *lpMaxRefreshRateInMicroHz);

ADL_EXTERNC int EXPOSED ADL2_Display_FreeSyncState_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iDisplayIndex, int *lpCurrent, int *lpDefault, int *lpMinRefreshRateInMicroHz, int *lpMaxRefreshRateInMicroHz);

ADL_EXTERNC int EXPOSED ADL_Display_FreeSyncState_Set(int iAdapterIndex, int iDisplayIndex, int iSetting, int iRefreshRateInMicroHz);

ADL_EXTERNC int EXPOSED ADL2_Display_FreeSyncState_Set(ADL_CONTEXT_HANDLE context,int iAdapterIndex, int iDisplayIndex, int iSetting, int iRefreshRateInMicroHz);

ADL_EXTERNC int EXPOSED ADL2_Display_DCE_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iDisplayIndex, ADLDceSettings *lpADLDceSettings);

ADL_EXTERNC int EXPOSED ADL_Display_DCE_Set(int iAdapterIndex, int iDisplayIndex, ADLDceSettings *lpADLDceSettings);

ADL_EXTERNC int EXPOSED ADL2_Display_DCE_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iDisplayIndex, ADLDceSettings *lpADLDceSettings);

ADL_EXTERNC int EXPOSED ADL_Display_DCE_Get(int iAdapterIndex, int iDisplayIndex, ADLDceSettings *lpADLDceSettings);


ADL_EXTERNC int EXPOSED ADL2_Display_FreeSync_Cap(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iDisplayIndex, ADLFreeSyncCap *lpFreeSyncCaps);

ADL_EXTERNC int EXPOSED ADL_Display_FreeSync_Cap(int iAdapterIndex, int iDisplayIndex, ADLFreeSyncCap *lpFreeSyncCaps);

ADL_EXTERNC int EXPOSED ADL2_Display_DCE_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iDisplayIndex, ADLDceSettings *lpADLDceSettings);

ADL_EXTERNC int EXPOSED ADL_Display_DCE_Set(int iAdapterIndex, int iDisplayIndex, ADLDceSettings *lpADLDceSettings);

ADL_EXTERNC int EXPOSED ADL2_Display_DCE_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int iDisplayIndex, ADLDceSettings *lpADLDceSettings);

ADL_EXTERNC int EXPOSED ADL_Display_DCE_Get(int iAdapterIndex, int iDisplayIndex, ADLDceSettings *lpADLDceSettings);

ADL_EXTERNC int EXPOSED ADL_CDS_UnsafeMode_Set(int iAdapterIndex, int unsafeMode);

ADL_EXTERNC int EXPOSED ADL2_CDS_UnsafeMode_Set(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int unsafeMode);

ADL_EXTERNC int EXPOSED ADL2_TurboSyncSupport_Get(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int* iTurboSyncSupported);

ADL_EXTERNC int EXPOSED ADL2_User_Settings_Notify(ADL_CONTEXT_HANDLE context, int iAdapterIndex, ADL_USER_SETTINGS iSetting, int iChanged);

#endif /* DISPLAY_H_ */