#ifndef ADL_H_
#define ADL_H_

#include "adl_sdk.h"
//#include "amd_only/amd_structures.h"

#include "adapter.h"
#include "display.h"
//#include "workstation.h"
//#include "displaysmanager.h"

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


ADL_EXTERNC int EXPOSED ADL_Main_ControlX2_Create ( ADL_MAIN_MALLOC_CALLBACK callback, int iEnumConnectedAdapters, ADLThreadingModel threadingModel );

ADL_EXTERNC int EXPOSED ADL2_Main_ControlX2_Create ( ADL_MAIN_MALLOC_CALLBACK callback, int iEnumConnectedAdapters, ADL_CONTEXT_HANDLE* context, ADLThreadingModel threadingModel);

ADL_EXTERNC int EXPOSED ADL2_Main_ControlX3_Create(ADL_MAIN_MALLOC_CALLBACK callback, int iEnumConnectedAdapters, ADL_CONTEXT_HANDLE* context, ADLThreadingModel threadingModel, int adlCreateOptions);

ADL_EXTERNC int EXPOSED ADL_Main_Control_Create ( ADL_MAIN_MALLOC_CALLBACK callback, int iEnumConnectedAdapters );

ADL_EXTERNC int EXPOSED ADL2_Main_Control_Create ( ADL_MAIN_MALLOC_CALLBACK callback, int iEnumConnectedAdapters, ADL_CONTEXT_HANDLE* context);

ADL_EXTERNC int EXPOSED ADL2_Main_Control_Refresh (ADL_CONTEXT_HANDLE context);

ADL_EXTERNC int EXPOSED ADL_Main_Control_Refresh ();

ADL_EXTERNC int EXPOSED ADL_Main_Control_Destroy ();

ADL_EXTERNC int EXPOSED ADL2_Main_Control_Destroy (ADL_CONTEXT_HANDLE context);

ADL_EXTERNC EXPOSED void* ADL2_Main_Control_GetProcAddress (ADL_CONTEXT_HANDLE context, void* lpModule, char* lpProcName );

ADL_EXTERNC EXPOSED void* ADL_Main_Control_GetProcAddress ( void* lpModule, char* lpProcName );


ADL_EXTERNC int EXPOSED ADL2_RegisterEvent(ADL_CONTEXT_HANDLE context, int eventID, void* evntHandle);

ADL_EXTERNC int EXPOSED ADL2_UnRegisterEvent(ADL_CONTEXT_HANDLE context, int eventID, void* evntHandle);



ADL_EXTERNC int EXPOSED ADL2_RegisterEventX2(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int clientID, int eventID, void* evntHandle);

ADL_EXTERNC int EXPOSED ADL2_UnRegisterEventX2(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int clientID, int eventID, void* evntHandle);

ADL_EXTERNC int EXPOSED ADL2_PerGPU_GDEvent_Register(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int clientID, int eventID, void* evntHandle);

ADL_EXTERNC int EXPOSED ADL2_PerGPU_GDEvent_UnRegister(ADL_CONTEXT_HANDLE context, int iAdapterIndex, int clientID, int eventID, void* evntHandle);


#endif /* ADL_H_ */