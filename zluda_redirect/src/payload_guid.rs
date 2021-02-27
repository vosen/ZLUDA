#[allow(dead_code)]
const PAYLOAD_NVCUDA_GUID: detours_sys::GUID = detours_sys::GUID {
    Data1: 0xC225FC0C,
    Data2: 0x00D7,
    Data3: 0x40B8,
    Data4: [0x93, 0x5A, 0x7E, 0x34, 0x2A, 0x93, 0x44, 0xC1],
};

#[allow(dead_code)]
const PAYLOAD_NVML_GUID: detours_sys::GUID = detours_sys::GUID {
    Data1: 0x75B54759,
    Data2: 0xB6F1,
    Data3: 0x49C2,
    Data4: [0xA2, 0x09, 0x68, 0x54, 0x96, 0xBD, 0x70, 0xC0],
};

#[allow(dead_code)]
const PAYLOAD_NVAPI_GUID: detours_sys::GUID = detours_sys::GUID {
    Data1: 0xc01fff7f,
    Data2: 0x310d,
    Data3: 0x4ac4,
    Data4: [0xAF, 0x47, 0x11, 0x4d, 0x9a, 0xef, 0x7a, 47],
};

#[allow(dead_code)]
const PAYLOAD_NVOPTIX_GUID: detours_sys::GUID = detours_sys::GUID {
    Data1: 0x629e45e5,
    Data2: 0xf4d1,
    Data3: 0x4649,
    Data4: [0x9d, 0xb4, 0x7e, 0x17, 0xf6, 0x74, 0xc2, 0xad],
};
