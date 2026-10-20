match (name, flag) {
    (b"cuD3D10CtxCreate", 0) => {
        if version >= 3020 {
            return cuD3D10CtxCreate_v2 as _;
        }
        if version >= 2010 {
            return cuD3D10CtxCreate as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10CtxCreate", 1) => {
        if version >= 3020 {
            return cuD3D10CtxCreate_v2 as _;
        }
        if version >= 2010 {
            return cuD3D10CtxCreate as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10CtxCreate", 2) => {
        if version >= 3020 {
            return cuD3D10CtxCreate_v2 as _;
        }
        if version >= 2010 {
            return cuD3D10CtxCreate as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10CtxCreateOnDevice", 0) => {
        if version >= 3020 {
            return cuD3D10CtxCreateOnDevice as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10CtxCreateOnDevice", 1) => {
        if version >= 3020 {
            return cuD3D10CtxCreateOnDevice as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10CtxCreateOnDevice", 2) => {
        if version >= 3020 {
            return cuD3D10CtxCreateOnDevice as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10GetDevice", 0) => {
        if version >= 2010 {
            return cuD3D10GetDevice as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10GetDevice", 1) => {
        if version >= 2010 {
            return cuD3D10GetDevice as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10GetDevice", 2) => {
        if version >= 2010 {
            return cuD3D10GetDevice as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10GetDevices", 0) => {
        if version >= 3020 {
            return cuD3D10GetDevices as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10GetDevices", 1) => {
        if version >= 3020 {
            return cuD3D10GetDevices as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10GetDevices", 2) => {
        if version >= 3020 {
            return cuD3D10GetDevices as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10GetDirect3DDevice", 0) => {
        if version >= 3020 {
            return cuD3D10GetDirect3DDevice as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10GetDirect3DDevice", 1) => {
        if version >= 3020 {
            return cuD3D10GetDirect3DDevice as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10GetDirect3DDevice", 2) => {
        if version >= 3020 {
            return cuD3D10GetDirect3DDevice as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10MapResources", 0) => {
        if version >= 2010 {
            return cuD3D10MapResources as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10MapResources", 1) => {
        if version >= 2010 {
            return cuD3D10MapResources as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10MapResources", 2) => {
        if version >= 2010 {
            return cuD3D10MapResources as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10RegisterResource", 0) => {
        if version >= 2010 {
            return cuD3D10RegisterResource as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10RegisterResource", 1) => {
        if version >= 2010 {
            return cuD3D10RegisterResource as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10RegisterResource", 2) => {
        if version >= 2010 {
            return cuD3D10RegisterResource as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10ResourceGetMappedArray", 0) => {
        if version >= 2010 {
            return cuD3D10ResourceGetMappedArray as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10ResourceGetMappedArray", 1) => {
        if version >= 2010 {
            return cuD3D10ResourceGetMappedArray as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10ResourceGetMappedArray", 2) => {
        if version >= 2010 {
            return cuD3D10ResourceGetMappedArray as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10ResourceGetMappedPitch", 0) => {
        if version >= 3020 {
            return cuD3D10ResourceGetMappedPitch_v2 as _;
        }
        if version >= 2010 {
            return cuD3D10ResourceGetMappedPitch as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10ResourceGetMappedPitch", 1) => {
        if version >= 3020 {
            return cuD3D10ResourceGetMappedPitch_v2 as _;
        }
        if version >= 2010 {
            return cuD3D10ResourceGetMappedPitch as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10ResourceGetMappedPitch", 2) => {
        if version >= 3020 {
            return cuD3D10ResourceGetMappedPitch_v2 as _;
        }
        if version >= 2010 {
            return cuD3D10ResourceGetMappedPitch as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10ResourceGetMappedPointer", 0) => {
        if version >= 3020 {
            return cuD3D10ResourceGetMappedPointer_v2 as _;
        }
        if version >= 2010 {
            return cuD3D10ResourceGetMappedPointer as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10ResourceGetMappedPointer", 1) => {
        if version >= 3020 {
            return cuD3D10ResourceGetMappedPointer_v2 as _;
        }
        if version >= 2010 {
            return cuD3D10ResourceGetMappedPointer as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10ResourceGetMappedPointer", 2) => {
        if version >= 3020 {
            return cuD3D10ResourceGetMappedPointer_v2 as _;
        }
        if version >= 2010 {
            return cuD3D10ResourceGetMappedPointer as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10ResourceGetMappedSize", 0) => {
        if version >= 3020 {
            return cuD3D10ResourceGetMappedSize_v2 as _;
        }
        if version >= 2010 {
            return cuD3D10ResourceGetMappedSize as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10ResourceGetMappedSize", 1) => {
        if version >= 3020 {
            return cuD3D10ResourceGetMappedSize_v2 as _;
        }
        if version >= 2010 {
            return cuD3D10ResourceGetMappedSize as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10ResourceGetMappedSize", 2) => {
        if version >= 3020 {
            return cuD3D10ResourceGetMappedSize_v2 as _;
        }
        if version >= 2010 {
            return cuD3D10ResourceGetMappedSize as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10ResourceGetSurfaceDimensions", 0) => {
        if version >= 3020 {
            return cuD3D10ResourceGetSurfaceDimensions_v2 as _;
        }
        if version >= 2010 {
            return cuD3D10ResourceGetSurfaceDimensions as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10ResourceGetSurfaceDimensions", 1) => {
        if version >= 3020 {
            return cuD3D10ResourceGetSurfaceDimensions_v2 as _;
        }
        if version >= 2010 {
            return cuD3D10ResourceGetSurfaceDimensions as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10ResourceGetSurfaceDimensions", 2) => {
        if version >= 3020 {
            return cuD3D10ResourceGetSurfaceDimensions_v2 as _;
        }
        if version >= 2010 {
            return cuD3D10ResourceGetSurfaceDimensions as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10ResourceSetMapFlags", 0) => {
        if version >= 2010 {
            return cuD3D10ResourceSetMapFlags as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10ResourceSetMapFlags", 1) => {
        if version >= 2010 {
            return cuD3D10ResourceSetMapFlags as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10ResourceSetMapFlags", 2) => {
        if version >= 2010 {
            return cuD3D10ResourceSetMapFlags as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10UnmapResources", 0) => {
        if version >= 2010 {
            return cuD3D10UnmapResources as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10UnmapResources", 1) => {
        if version >= 2010 {
            return cuD3D10UnmapResources as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10UnmapResources", 2) => {
        if version >= 2010 {
            return cuD3D10UnmapResources as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10UnregisterResource", 0) => {
        if version >= 2010 {
            return cuD3D10UnregisterResource as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10UnregisterResource", 1) => {
        if version >= 2010 {
            return cuD3D10UnregisterResource as _;
        }
        usize::MAX as _
    }
    (b"cuD3D10UnregisterResource", 2) => {
        if version >= 2010 {
            return cuD3D10UnregisterResource as _;
        }
        usize::MAX as _
    }
    (b"cuD3D11CtxCreate", 0) => {
        if version >= 3020 {
            return cuD3D11CtxCreate_v2 as _;
        }
        if version >= 3000 {
            return cuD3D11CtxCreate as _;
        }
        usize::MAX as _
    }
    (b"cuD3D11CtxCreate", 1) => {
        if version >= 3020 {
            return cuD3D11CtxCreate_v2 as _;
        }
        if version >= 3000 {
            return cuD3D11CtxCreate as _;
        }
        usize::MAX as _
    }
    (b"cuD3D11CtxCreate", 2) => {
        if version >= 3020 {
            return cuD3D11CtxCreate_v2 as _;
        }
        if version >= 3000 {
            return cuD3D11CtxCreate as _;
        }
        usize::MAX as _
    }
    (b"cuD3D11CtxCreateOnDevice", 0) => {
        if version >= 3020 {
            return cuD3D11CtxCreateOnDevice as _;
        }
        usize::MAX as _
    }
    (b"cuD3D11CtxCreateOnDevice", 1) => {
        if version >= 3020 {
            return cuD3D11CtxCreateOnDevice as _;
        }
        usize::MAX as _
    }
    (b"cuD3D11CtxCreateOnDevice", 2) => {
        if version >= 3020 {
            return cuD3D11CtxCreateOnDevice as _;
        }
        usize::MAX as _
    }
    (b"cuD3D11GetDevice", 0) => {
        if version >= 3000 {
            return cuD3D11GetDevice as _;
        }
        usize::MAX as _
    }
    (b"cuD3D11GetDevice", 1) => {
        if version >= 3000 {
            return cuD3D11GetDevice as _;
        }
        usize::MAX as _
    }
    (b"cuD3D11GetDevice", 2) => {
        if version >= 3000 {
            return cuD3D11GetDevice as _;
        }
        usize::MAX as _
    }
    (b"cuD3D11GetDevices", 0) => {
        if version >= 3020 {
            return cuD3D11GetDevices as _;
        }
        usize::MAX as _
    }
    (b"cuD3D11GetDevices", 1) => {
        if version >= 3020 {
            return cuD3D11GetDevices as _;
        }
        usize::MAX as _
    }
    (b"cuD3D11GetDevices", 2) => {
        if version >= 3020 {
            return cuD3D11GetDevices as _;
        }
        usize::MAX as _
    }
    (b"cuD3D11GetDirect3DDevice", 0) => {
        if version >= 3020 {
            return cuD3D11GetDirect3DDevice as _;
        }
        usize::MAX as _
    }
    (b"cuD3D11GetDirect3DDevice", 1) => {
        if version >= 3020 {
            return cuD3D11GetDirect3DDevice as _;
        }
        usize::MAX as _
    }
    (b"cuD3D11GetDirect3DDevice", 2) => {
        if version >= 3020 {
            return cuD3D11GetDirect3DDevice as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9Begin", 0) => {
        if version >= 2000 {
            return cuD3D9Begin as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9Begin", 1) => {
        if version >= 2000 {
            return cuD3D9Begin as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9Begin", 2) => {
        if version >= 2000 {
            return cuD3D9Begin as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9CtxCreate", 0) => {
        if version >= 3020 {
            return cuD3D9CtxCreate_v2 as _;
        }
        if version >= 2000 {
            return cuD3D9CtxCreate as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9CtxCreate", 1) => {
        if version >= 3020 {
            return cuD3D9CtxCreate_v2 as _;
        }
        if version >= 2000 {
            return cuD3D9CtxCreate as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9CtxCreate", 2) => {
        if version >= 3020 {
            return cuD3D9CtxCreate_v2 as _;
        }
        if version >= 2000 {
            return cuD3D9CtxCreate as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9CtxCreateOnDevice", 0) => {
        if version >= 3020 {
            return cuD3D9CtxCreateOnDevice as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9CtxCreateOnDevice", 1) => {
        if version >= 3020 {
            return cuD3D9CtxCreateOnDevice as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9CtxCreateOnDevice", 2) => {
        if version >= 3020 {
            return cuD3D9CtxCreateOnDevice as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9End", 0) => {
        if version >= 2000 {
            return cuD3D9End as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9End", 1) => {
        if version >= 2000 {
            return cuD3D9End as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9End", 2) => {
        if version >= 2000 {
            return cuD3D9End as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9GetDevice", 0) => {
        if version >= 2000 {
            return cuD3D9GetDevice as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9GetDevice", 1) => {
        if version >= 2000 {
            return cuD3D9GetDevice as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9GetDevice", 2) => {
        if version >= 2000 {
            return cuD3D9GetDevice as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9GetDevices", 0) => {
        if version >= 3020 {
            return cuD3D9GetDevices as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9GetDevices", 1) => {
        if version >= 3020 {
            return cuD3D9GetDevices as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9GetDevices", 2) => {
        if version >= 3020 {
            return cuD3D9GetDevices as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9GetDirect3DDevice", 0) => {
        if version >= 2000 {
            return cuD3D9GetDirect3DDevice as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9GetDirect3DDevice", 1) => {
        if version >= 2000 {
            return cuD3D9GetDirect3DDevice as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9GetDirect3DDevice", 2) => {
        if version >= 2000 {
            return cuD3D9GetDirect3DDevice as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9MapResources", 0) => {
        if version >= 2000 {
            return cuD3D9MapResources as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9MapResources", 1) => {
        if version >= 2000 {
            return cuD3D9MapResources as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9MapResources", 2) => {
        if version >= 2000 {
            return cuD3D9MapResources as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9MapVertexBuffer", 0) => {
        if version >= 3020 {
            return cuD3D9MapVertexBuffer_v2 as _;
        }
        if version >= 2000 {
            return cuD3D9MapVertexBuffer as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9MapVertexBuffer", 1) => {
        if version >= 3020 {
            return cuD3D9MapVertexBuffer_v2 as _;
        }
        if version >= 2000 {
            return cuD3D9MapVertexBuffer as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9MapVertexBuffer", 2) => {
        if version >= 3020 {
            return cuD3D9MapVertexBuffer_v2 as _;
        }
        if version >= 2000 {
            return cuD3D9MapVertexBuffer as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9RegisterResource", 0) => {
        if version >= 2000 {
            return cuD3D9RegisterResource as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9RegisterResource", 1) => {
        if version >= 2000 {
            return cuD3D9RegisterResource as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9RegisterResource", 2) => {
        if version >= 2000 {
            return cuD3D9RegisterResource as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9RegisterVertexBuffer", 0) => {
        if version >= 2000 {
            return cuD3D9RegisterVertexBuffer as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9RegisterVertexBuffer", 1) => {
        if version >= 2000 {
            return cuD3D9RegisterVertexBuffer as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9RegisterVertexBuffer", 2) => {
        if version >= 2000 {
            return cuD3D9RegisterVertexBuffer as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9ResourceGetMappedArray", 0) => {
        if version >= 2010 {
            return cuD3D9ResourceGetMappedArray as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9ResourceGetMappedArray", 1) => {
        if version >= 2010 {
            return cuD3D9ResourceGetMappedArray as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9ResourceGetMappedArray", 2) => {
        if version >= 2010 {
            return cuD3D9ResourceGetMappedArray as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9ResourceGetMappedPitch", 0) => {
        if version >= 3020 {
            return cuD3D9ResourceGetMappedPitch_v2 as _;
        }
        if version >= 2000 {
            return cuD3D9ResourceGetMappedPitch as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9ResourceGetMappedPitch", 1) => {
        if version >= 3020 {
            return cuD3D9ResourceGetMappedPitch_v2 as _;
        }
        if version >= 2000 {
            return cuD3D9ResourceGetMappedPitch as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9ResourceGetMappedPitch", 2) => {
        if version >= 3020 {
            return cuD3D9ResourceGetMappedPitch_v2 as _;
        }
        if version >= 2000 {
            return cuD3D9ResourceGetMappedPitch as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9ResourceGetMappedPointer", 0) => {
        if version >= 3020 {
            return cuD3D9ResourceGetMappedPointer_v2 as _;
        }
        if version >= 2000 {
            return cuD3D9ResourceGetMappedPointer as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9ResourceGetMappedPointer", 1) => {
        if version >= 3020 {
            return cuD3D9ResourceGetMappedPointer_v2 as _;
        }
        if version >= 2000 {
            return cuD3D9ResourceGetMappedPointer as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9ResourceGetMappedPointer", 2) => {
        if version >= 3020 {
            return cuD3D9ResourceGetMappedPointer_v2 as _;
        }
        if version >= 2000 {
            return cuD3D9ResourceGetMappedPointer as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9ResourceGetMappedSize", 0) => {
        if version >= 3020 {
            return cuD3D9ResourceGetMappedSize_v2 as _;
        }
        if version >= 2000 {
            return cuD3D9ResourceGetMappedSize as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9ResourceGetMappedSize", 1) => {
        if version >= 3020 {
            return cuD3D9ResourceGetMappedSize_v2 as _;
        }
        if version >= 2000 {
            return cuD3D9ResourceGetMappedSize as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9ResourceGetMappedSize", 2) => {
        if version >= 3020 {
            return cuD3D9ResourceGetMappedSize_v2 as _;
        }
        if version >= 2000 {
            return cuD3D9ResourceGetMappedSize as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9ResourceGetSurfaceDimensions", 0) => {
        if version >= 3020 {
            return cuD3D9ResourceGetSurfaceDimensions_v2 as _;
        }
        if version >= 2000 {
            return cuD3D9ResourceGetSurfaceDimensions as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9ResourceGetSurfaceDimensions", 1) => {
        if version >= 3020 {
            return cuD3D9ResourceGetSurfaceDimensions_v2 as _;
        }
        if version >= 2000 {
            return cuD3D9ResourceGetSurfaceDimensions as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9ResourceGetSurfaceDimensions", 2) => {
        if version >= 3020 {
            return cuD3D9ResourceGetSurfaceDimensions_v2 as _;
        }
        if version >= 2000 {
            return cuD3D9ResourceGetSurfaceDimensions as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9ResourceSetMapFlags", 0) => {
        if version >= 2000 {
            return cuD3D9ResourceSetMapFlags as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9ResourceSetMapFlags", 1) => {
        if version >= 2000 {
            return cuD3D9ResourceSetMapFlags as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9ResourceSetMapFlags", 2) => {
        if version >= 2000 {
            return cuD3D9ResourceSetMapFlags as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9UnmapResources", 0) => {
        if version >= 2000 {
            return cuD3D9UnmapResources as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9UnmapResources", 1) => {
        if version >= 2000 {
            return cuD3D9UnmapResources as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9UnmapResources", 2) => {
        if version >= 2000 {
            return cuD3D9UnmapResources as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9UnmapVertexBuffer", 0) => {
        if version >= 2000 {
            return cuD3D9UnmapVertexBuffer as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9UnmapVertexBuffer", 1) => {
        if version >= 2000 {
            return cuD3D9UnmapVertexBuffer as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9UnmapVertexBuffer", 2) => {
        if version >= 2000 {
            return cuD3D9UnmapVertexBuffer as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9UnregisterResource", 0) => {
        if version >= 2000 {
            return cuD3D9UnregisterResource as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9UnregisterResource", 1) => {
        if version >= 2000 {
            return cuD3D9UnregisterResource as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9UnregisterResource", 2) => {
        if version >= 2000 {
            return cuD3D9UnregisterResource as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9UnregisterVertexBuffer", 0) => {
        if version >= 2000 {
            return cuD3D9UnregisterVertexBuffer as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9UnregisterVertexBuffer", 1) => {
        if version >= 2000 {
            return cuD3D9UnregisterVertexBuffer as _;
        }
        usize::MAX as _
    }
    (b"cuD3D9UnregisterVertexBuffer", 2) => {
        if version >= 2000 {
            return cuD3D9UnregisterVertexBuffer as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsD3D10RegisterResource", 0) => {
        if version >= 3000 {
            return cuGraphicsD3D10RegisterResource as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsD3D10RegisterResource", 1) => {
        if version >= 3000 {
            return cuGraphicsD3D10RegisterResource as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsD3D10RegisterResource", 2) => {
        if version >= 3000 {
            return cuGraphicsD3D10RegisterResource as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsD3D11RegisterResource", 0) => {
        if version >= 3000 {
            return cuGraphicsD3D11RegisterResource as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsD3D11RegisterResource", 1) => {
        if version >= 3000 {
            return cuGraphicsD3D11RegisterResource as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsD3D11RegisterResource", 2) => {
        if version >= 3000 {
            return cuGraphicsD3D11RegisterResource as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsD3D9RegisterResource", 0) => {
        if version >= 3000 {
            return cuGraphicsD3D9RegisterResource as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsD3D9RegisterResource", 1) => {
        if version >= 3000 {
            return cuGraphicsD3D9RegisterResource as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsD3D9RegisterResource", 2) => {
        if version >= 3000 {
            return cuGraphicsD3D9RegisterResource as _;
        }
        usize::MAX as _
    }
    _ => 0usize as _
}
