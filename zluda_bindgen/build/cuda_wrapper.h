#define __CUDA_API_VERSION_INTERNAL
#ifdef _WIN32
#include <windows.h>
#endif
#include <cuda.h>
#include <cudaProfiler.h>
#ifndef _WIN32
#include <cudaGL.h>
#include <cudaEGL.h>
#include <vdpau/vdpau.h>
#include <cudaVDPAU.h>
#endif
#include <library_types.h>
#include <cuComplex.h>

typedef struct IDirect3DDevice9 IDirect3DDevice9;
typedef struct IDirect3DResource9 IDirect3DResource9;
typedef struct IDirect3DVertexBuffer9 IDirect3DVertexBuffer9;
#include "cudaD3D9.h"

typedef struct IDXGIAdapter IDXGIAdapter;
typedef struct ID3D10Device ID3D10Device;
typedef struct ID3D10Resource ID3D10Resource;
#include "cudaD3D10.h"

typedef struct ID3D11Device ID3D11Device;
typedef struct ID3D11Resource ID3D11Resource;
#include "cudaD3D11.h"
