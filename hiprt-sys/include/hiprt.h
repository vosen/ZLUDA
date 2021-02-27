#ifndef HIPRT_H
#define HIPRT_H

#define HIPRT_API_MAJOR_VERSION 0x000001
#define HIPRT_API_MINOR_VERSION 0x000002
#define HIPRT_API_PATCH_VERSION 0x000000
#define HIPRT_API_VERSION HIPRT_API_MAJOR_VERSION * 1000000 + HIPRT_API_MINOR_VERSION * 1000 + HIPRT_API_PATCH_VERSION

#include <hiprt/hiprt_vec.h>
#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#if defined( _MSC_VER )
#ifdef HIPRT_EXPORTS
#define HIPRT_API __declspec( dllexport )
#else
#define HIPRT_API __declspec( dllimport )
#endif
#elif defined( __GNUC__ )
#ifdef HIPRT_EXPORTS
#define HIPRT_API __attribute__( ( visibility( "default" ) ) )
#else
#define HIPRT_API
#endif
#else
#define HIPRT_API
#pragma warning Unknown dynamic link import / export semantics.
#endif

struct _hiprtGeometry;
struct _hiprtGeometryCustom;
struct _hiprtScene;
struct _hiprtContext;
struct _hiprtCustomFuncTable;

typedef void*				   hiprtDevicePtr;
typedef hiprtDevicePtr		   hiprtGeometry;
typedef hiprtDevicePtr		   hiprtScene;
typedef uint32_t			   hiprtBuildFlags;
typedef uint32_t			   hiprtRayMask;
typedef uint32_t			   hiprtCustomType;
typedef _hiprtContext*		   hiprtContext;
typedef _hiprtCustomFuncTable* hiprtCustomFuncTable;

typedef int	  hiprtApiDevice;	// hipDevice, cuDevice
typedef void* hiprtApiCtx;		// hipCtx, cuCtx
typedef void* hiprtApiStream;	// hipStream, cuStream
typedef void* hiprtApiFunction; // hipFunction, cuFunction

/** \brief Various constants.
 *
 */
enum : uint32_t
{
	hiprtInvalidValue		= ~0u,
	hiprtMaxCustomFunctions = 65536,
};

/** \brief Error codes.
 *
 */
typedef enum
{
	hiprtSuccess				= 0,
	hiprtErrorNotImplemented	= 1,
	hiprtErrorInternal			= 2,
	hiprtErrorOutOfHostMemory	= 3,
	hiprtErrorOutOfDeviceMemory = 4,
	hiprtErrorInvalidApiVersion = 5,
	hiprtErrorInvalidParameter	= 6
} hiprtError;

/** \brief Type of geometry/scene build operation.
 *
 * hiprtBuildGeometry/hiprtBuildScene can either build or update
 * an underlying acceleration structure.
 */
typedef enum
{
	hiprtBuildOperationBuild  = 1,
	hiprtBuildOperationUpdate = 2
} hiprtBuildOperation;

/** \brief Hint flags for geometry/scene build functions.
 *
 * hiprtBuildGeometry/hiprtBuildScene use these flags to choose
 * an appropriate build format/algorithm.
 */
typedef enum
{
	hiprtBuildFlagBitPreferFastBuild		= 0,
	hiprtBuildFlagBitPreferBalancedBuild	= 1,
	hiprtBuildFlagBitPreferHighQualityBuild = 2,
	hiprtBuildFlagBitCustomBvhImport		= 3,
	hiprtBuildFlagBitDisableSpatialSplits	= 1 << 2
} hiprtBuildFlagBits;

/** \brief Geometric primitive type.
 *
 * hiprtGeometry can be built from multiple primitive types,
 * such as triangle meshes, AABB lists, line lists, etc. This enum
 * defines primitive type for hiprtBuildGeometry function.
 */
typedef enum
{
	hiprtPrimitiveTypeTriangleMesh,
	hiprtPrimitiveTypeAABBList,
} hiprtPrimitiveType;

/** \brief Traversal state.
 *
 * On-device traversal can be in either hit state (and can be continued using
 * hiprtNextHit) or finished state.
 */
typedef enum
{
	hiprtTraversalStateInit,
	hiprtTraversalStateFinished,
	hiprtTraversalStateHit,
	hiprtTraversalStateStackOverflow
} hiprtTraversalState;

/** \brief Ray traversal type.
 *
 */
enum hiprtTraversalType
{
	/*!< 0 or 1 element iterator with any hit along the ray */
	hiprtTraversalTerminateAtAnyHit = 1,
	/*!< 0 or 1 element iterator with a closest hit along the ray */
	hiprtTraversalTerminateAtClosestHit = 2,
};

/** \brief Bvh node type.
 *
 */
enum hiprtBvhNodeType
{
	/*!< Leaf node */
	hiprtBvhNodeTypeInternal = 0,
	/*!< Internal node */
	hiprtBvhNodeTypeLeaf = 1,
};

/** \brief Ray data structure.
 *
 */
struct hiprtRay
{
	/*!< Ray origin */
	hiprtFloat3 origin;
	/*!< Ray time for motion blur */
	float time;
	/*!< Ray direction */
	hiprtFloat3 direction;
	/*!< Ray maximum distance */
	float maxT;
};

/** \brief Ray hit data structure.
 *
 */
struct hiprtHit
{
	/*!< Instance ID */
	uint32_t instanceID;
	/*!< Primitive ID */
	uint32_t primID;
	/*!< Texture coordinates */
	hiprtFloat2 uv;
	/*!< Geeometric normal (not normalized) */
	hiprtFloat3 normal;
	/*!< Distance */
	float t;
};

/** \brief Insersection function for custom primitives.
 *
 * \param ray Ray.
 * \param primID Primtive ID.
 * \param data User data.
 * \param payload Payload for additional outputs.
 * \param uv Output texture coordinates.
 * \param normal Output normal.
 * \param t Output distance.
 * \return A flag indicating hit.
 */
typedef bool ( *hiprtIntersectFunc )(
	const hiprtRay& ray,
	uint32_t		primID,
	const void*		data,
	void*			payload,
	hiprtFloat2&	uv,
	hiprtFloat3&	normal,
	float&			t );

/** \brief Set of functions for custom primitives.
 *
 */
typedef struct
{
	hiprtIntersectFunc intersectFunc;
	const void*		   intersectFuncData;
} hiprtCustomFuncSet;

/** \brief Device type.
 *
 */
enum hiprtDeviceType
{
	/*!< AMD device */
	hiprtDeviceAMD,
	/*!< Nvidia device */
	hiprtDeviceNVIDIA,
};

/** \brief Context creation input.
 *
 */
typedef struct
{
	/*!< HIPRT API context */
	hiprtApiCtx ctxt;
	/*!< HIPRT API device */
	hiprtApiDevice device;
	/*!< HIPRT API device type */
	hiprtDeviceType deviceType;
} hiprtContextCreationInput;

/** \brief Various flags controlling scene/geometry build process.
 *
 */
typedef struct
{
	/*!< Build flags */
	hiprtBuildFlags buildFlags;
} hiprtBuildOptions;

/** \brief Triangle mesh primitive.
 *
 * Triangle mesh primitive is represented as an indexed vertex array.
 * Vertex and index arrays are defined using device pointers and strides.
 * Each vertex has to have 3 components: (x, y, z) coordinates.
 * Indices are organized into triples (i0, i1, i2) - one for each triangle.
 */
typedef struct
{
	/*!< Device pointer to vertex data */
	hiprtDevicePtr vertices;
	/*!< Number of vertices in vertex array */
	uint32_t vertexCount;
	/*!< Stride in bytes between two vertices */
	uint32_t vertexStride;

	/*!< Device pointer to index data */
	hiprtDevicePtr triangleIndices;
	/*!< Number of trinagles in index array */
	uint32_t triangleCount;
	/*!< Stride in bytes between two triangles */
	uint32_t triangleStride;
} hiprtTriangleMeshPrimitive;

/** \brief AABB list primitive.
 *
 * AABB list is an array of axis aligned bounding boxes, represented
 * by device memory pointer and stride between two consequetive boxes.
 * Each AABB is a pair of float4 values (xmin, ymin, zmin, unused), (xmax, ymax,
 * zmax, unused).
 */
typedef struct
{
	/*!< Device pointer to AABB data */
	hiprtDevicePtr aabbs;
	/*!< Number of AABBs in the array */
	uint32_t aabbCount;
	/*!< Stride in bytes between two AABBs */
	uint32_t aabbStride;
} hiprtAABBListPrimitive;

/** \brief Bvh node for custom import Bvh.
 *
 */
typedef struct
{
	/*!< Child indices (empty slot needs to be marked by hiprtInvalidValue) */
	uint32_t childIndices[4];
	/*!< Child node types */
	hiprtBvhNodeType childNodeTypes[4];
	/*!< Node bounding box min */
	hiprtFloat3 boundingBoxMin;
	/*!< Node bounding box max */
	hiprtFloat3 boundingBoxMax;

	int pad[2];
} hiprtBvhNode;

/** \brief Bvh node list.
 *
 */
typedef struct
{
	/*!< Array of hiprtBvhNode's */
	hiprtDevicePtr nodes;
	/*!< Number of nodes */
	uint32_t nodeCount;
} hiprtBvhNodeList;

/** \brief Input for geometry build/update operation.
 *
 * Build input defines concrete primitive type and a pointer to an actual
 * primitive description.
 */
typedef struct
{
	/*!< Primitive type */
	hiprtPrimitiveType type;
	/*!< Defines the following union */
	union
	{
		struct
		{
			/*!< Triangle mesh */
			hiprtTriangleMeshPrimitive* primitive;
		} triangleMesh;
		struct
		{
			/*!< Bounding boxes of custom primitives */
			hiprtAABBListPrimitive* primitive;
			/*!< Type of custom primitives */
			hiprtCustomType customType;
		} aabbList;
	};
	/*!< Custom Bvh nodes (optional) */
	hiprtBvhNodeList* nodes;
} hiprtGeometryBuildInput;

/** \brief Build input for the scene.
 *
 * Scene consists of a set of instances. Each of the instances is defined by:
 *  - Root pointer of the corresponding geometry
 *  - Transformation header
 *  - Mask
 *
 * Instances can refer to the same geometry but with different transformations
 * (essentially implementing instancing). Mask is used to implement ray
 * masking: ray mask is bitwise &ded with an instance mask, and no intersections
 * are evaluated with the primitive of corresponding instance if the result is
 * 0. The transformation header defines the offset and the number of consecutive 
 * transformation frames in the frame array for each instance. More than one frame 
 * is interpreted as motion blur. If the transformation headers is NULL, it 
 * assumes one frame per instance. Optionally, it is possible to import a custom 
 * BVH by setting nodes and the corresponding build flag.
 */
typedef struct
{
	/*!< Array of instanceCount pointers to geometries */
	hiprtDevicePtr instanceGeometries;
	/*!< Array of instanceCount transform headers (optional: per object frame assumed if NULL) */
	hiprtDevicePtr instanceTransformHeaders;
	/*!< Array of frameCount frames (supposed to be ordered according to time) */
	hiprtDevicePtr instanceFrames;
	/*!< Per object bit masks for instance masking (optional: if NULL masks treated as 0xFFFFFFFF) */
	hiprtDevicePtr instanceMasks;
	/*!< Custom Bvh nodes (optional) */
	hiprtBvhNodeList* nodes;
	/*!< Number of instances */
	uint32_t instanceCount;
	/*!< Number of frames (such that instanceCount <= frameCount) */
	uint32_t frameCount;
} hiprtSceneBuildInput;

/** \brief Transformation frame.
 *
 * Defines scale, translation, rotation, and time.
 */
typedef struct
{
	/*!< Rotation (axis and angle) */
	hiprtFloat4 rotation;
	/*!< Scale */
	hiprtFloat3 scale;
	/*!< Translation */
	hiprtFloat3 translation;
	/*!< Time */
	float time;

	int pad;
} hiprtFrame;

/** \brief Transformation header.
 *
 * Defines defines the index to the array of frames and the number of frames.
 */
typedef struct
{
	/*!< Frame index */
	uint32_t frameIndex;
	/*!< Number of frames */
	uint32_t frameCount;
} hiprtTransformHeader;

/** \brief Create HIPRT API context.
 *
 * All HIPRT functions expect context as their first argument. Context
 * keeps global data required by HIPRT session. Calls made from different
 * threads with different HIPRT contexts are safe. Calls with the same context
 * should be externally synchronized by the client.
 *
 * \param hiprtApiVersion API version.
 * \param outContext Created context.
 * \return HIPRT error in case of a failure, hiprtSuccess otherwise.
 */
HIPRT_API hiprtError hiprtCreateContext( uint32_t hiprtApiVersion, hiprtContextCreationInput& input, hiprtContext* outContext );

/** \brief Destory HIPRT API context.
 *
 * Destroys all the global resources used by HIPRT session. Further calls
 * with this context are prohibited.
 *
 * \param context API context.
 * \return HIPRT error in case of a failure, hiprtSuccess otherwise.
 */
HIPRT_API hiprtError hiprtDestroyContext( hiprtContext context );

/** \brief Create a geometry.
 *
 * This function creates
 * hiprtGeometry representing acceleration structure topology.
 *
 * \param context HIPRT API context.
 * \param buildInput Describes input primitive to build geometry from.
 * \param buildOptions Various flags controlling build process.
 * \param outGeometry Resulting geometry.
 * \return HIPRT error in case of a failure, hiprtSuccess otherwise.
 */
HIPRT_API hiprtError hiprtCreateGeometry(
	hiprtContext				   context,
	const hiprtGeometryBuildInput* buildInput,
	const hiprtBuildOptions*	   buildOptions,
	hiprtGeometry*				   outGeometry );

/** \brief Destroy a geometry.
 *
 * This function destroys
 * hiprtGeometry representing acceleration structure topology.
 *
 * \param context HIPRT API context.
 * \param outGeometry Resulting geometry.
 * \return HIPRT error in case of a failure, hiprtSuccess otherwise.
 */
HIPRT_API hiprtError hiprtDestroyGeometry( hiprtContext context, hiprtGeometry outGeometry );

/** \brief Build or update a geometry.
 *
 * Given geometry description from the client, this function builds
 * hiprtGeometry representing acceleration structure topology (in case of a
 * build) or updates acceleration structure keeping topology intact (update).
 *
 * \param context HIPRT API context.
 * \param buildOperation Type of build operation.
 * \param buildInput Describes input primitive to build geometry from.
 * \param buildOptions Various flags controlling build process.
 * \param attributeOutputs Describes additional values written into vidmem.
 * \param attributeOutputCount Number of additional attributes, can be 0.
 * \param temporaryBuffer Temporary buffer for build operation.
 * \param stream to run acceleration structure build command.
 * \param outGeometry Resulting geometry.
 * \return HIPRT error in case of a failure, hiprtSuccess otherwise.
 */
HIPRT_API hiprtError hiprtBuildGeometry(
	hiprtContext				   context,
	hiprtBuildOperation			   buildOperation,
	const hiprtGeometryBuildInput* buildInput,
	const hiprtBuildOptions*	   buildOptions,
	hiprtDevicePtr				   temporaryBuffer,
	hiprtApiStream				   stream,
	hiprtGeometry				   outGeometry );

/** \brief Get temporary storage requirements for geometry build.
 *
 * \param context HIPRT API context.
 * \param buildInput Describes input primitive to build geometry from.
 * \param buildOptions Various flags controlling build process.
 * \param outSize Pointer to write result to.
 * \return HIPRT error in case of a failure, hiprtSuccess otherwise.
 */
HIPRT_API hiprtError hiprtGetGeometryBuildTemporaryBufferSize(
	hiprtContext context, const hiprtGeometryBuildInput* buildInput, const hiprtBuildOptions* buildOptions, size_t* outSize );

/** \brief Get temporary storage requirements for scene trace.
 *
 * \param context HIPRT API context.
 * \param scene Built scene for trace.
 * \param numRays Rays to be issued.
 * \param outSize Pointer to write result to.
 * \return HIPRT error in case of a failure, hiprtSuccess otherwise.
 */
HIPRT_API hiprtError
hiprtGetGeometryTraceTemporaryBufferSize( hiprtContext context, hiprtGeometry scene, uint32_t numRays, size_t* outSize );

/** \brief Create a scene.
 *
 * This function creates
 * hiprtScene representing acceleration structure topology.
 *
 * \param context HIPRT API context.
 * \param buildInput Decribes input geometires to build scene for.
 * \param buildOptions Various flags controlling build process.
 * \param outScene Resulting scene.
 * \return HIPRT error in case of a failure, hiprtSuccess otherwise.
 */
HIPRT_API hiprtError hiprtCreateScene(
	hiprtContext context, const hiprtSceneBuildInput* buildInput, const hiprtBuildOptions* buildOptions, hiprtScene* outScene );

/** \brief Destroy a scene.
 *
 * This function destroys
 * hiprtScene representing acceleration structure topology.
 *
 * \param context HIPRT API context.
 * \param outScene Resulting scene.
 * \return HIPRT error in case of a failure, hiprtSuccess otherwise.
 */
HIPRT_API hiprtError hiprtDestroyScene( hiprtContext context, hiprtScene outScene );

/** \brief Build or update a scene.
 *
 * Given a number of hiprtGeometries from the client, this function builds
 * hiprtScene representing top level acceleration structure topology (in case of
 * a build) or updates acceleration structure keeping topology intact (update).
 *
 * \param context HIPRT API context.
 * \param buildOperation Type of build operation.
 * \param buildInput Decribes input geometires to build scene for.
 * \param buildOptions Various flags controlling build process.
 * \param temporaryBuffer Temporary buffer for build operation.
 * \param stream to run acceleration structure build command.
 * \param outScene Resulting scene.
 * \return HIPRT error in case of a failure, hiprtSuccess otherwise.
 */
HIPRT_API hiprtError hiprtBuildScene(
	hiprtContext				context,
	hiprtBuildOperation			buildOperation,
	const hiprtSceneBuildInput* buildInput,
	const hiprtBuildOptions*	buildOptions,
	hiprtDevicePtr				temporaryBuffer,
	hiprtApiStream				stream,
	hiprtScene					outScene );

/** \brief Get temporary storage requirements for scene build.
 *
 * \param context HIPRT API context.
 * \param buildInput Decribes input geometires to build scene for.
 * \param buildOptions Various flags controlling build process.
 * \param outSize Pointer to write result to.
 * \return HIPRT error in case of a failure, hiprtSuccess otherwise.
 */
HIPRT_API hiprtError hiprtGetSceneBuildTemporaryBufferSize(
	hiprtContext context, const hiprtSceneBuildInput* buildInput, const hiprtBuildOptions* buildOptions, size_t* outSize );

/** \brief Get temporary storage requirements for scene trace.
 *
 * \param context HIPRT API context.
 * \param scene Built scene for trace.
 * \param numRays Rays to be issued.
 * \param outSize Pointer to write result to.
 * \return HIPRT error in case of a failure, hiprtSuccess otherwise.
 */
HIPRT_API hiprtError
hiprtGetSceneTraceTemporaryBufferSize( hiprtContext context, hiprtScene scene, uint32_t numRays, size_t* outSize );

/** \brief Creates a custom function table (for custom geometry).
 *
 * \param context HIPRT API context.
 * \param outFuncTable Resulting table.
 * \return HIPRT error in case of a failure, hiprtSuccess otherwise.
 */
HIPRT_API hiprtError hiprtCreateCustomFuncTable( hiprtContext context, hiprtCustomFuncTable* outFuncTable );

/** \brief Sets a custom function table.
 *
 * \param context HIPRT API context.
 * \param outFuncTable Resulting table.
 * \param index Index of the set in the table.
 * \param set Function set to be set.
 * \return HIPRT error in case of a failure, hiprtSuccess otherwise.
 */
HIPRT_API hiprtError
hiprtSetCustomFuncTable( hiprtContext context, hiprtCustomFuncTable outFuncTable, uint32_t index, hiprtCustomFuncSet set );

/** \brief Destroys a custom function table.
 *
 * \param context HIPRT API context.
 * \param outFuncTable Resulting table.
 * \return HIPRT error in case of a failure, hiprtSuccess otherwise.
 */
HIPRT_API hiprtError hiprtDestroyCustomFuncTable( hiprtContext context, hiprtCustomFuncTable outFuncTable );

/** \brief Saves hiprtGeometry to a binary file.
 *
 * \param context HIPRT API context.
 * \param inGeometry Geometry to be saved.
 * \param filename File name with full path.
 * \return HIPRT error in case of a failure, hiprtSuccess otherwise.
 */
HIPRT_API hiprtError hiprtSaveGeometry( hiprtContext context, hiprtGeometry inGeometry, const char* filename );

/** \brief Loads hiprtGeometry to a binary file.
 *
 * \param context HIPRT API context.
 * \param outGeometry Geometry to be loaded.
 * \param filename File name with full path.
 * \return HIPRT error in case of a failure, hiprtSuccess otherwise.
 */
HIPRT_API hiprtError hiprtLoadGeometry( hiprtContext context, hiprtGeometry* outGeometry, const char* filename );

/** \brief Saves hiprtScene to a binary file.
 *
 * \param context HIPRT API context.
 * \param inScene Scene to be saved.
 * \param filename File name with full path.
 * \return HIPRT error in case of a failure, hiprtSuccess otherwise.
 */
HIPRT_API hiprtError hiprtSaveScene( hiprtContext context, hiprtScene inScene, const char* filename );

/** \brief Loads hiprtScene to a binary file.
 *
 * \param context HIPRT API context.
 * \param outScene Scene to be loaded.
 * \param filename File name with full path.
 * \return HIPRT error in case of a failure, hiprtSuccess otherwise.
 */
HIPRT_API hiprtError hiprtLoadScene( hiprtContext context, hiprtScene* outScene, const char* filename );

/** \brief Output scene's AABB.
 *
 * \param context HIPRT API context.
 * \param inGeometry Geometry to be queried.
 * \param outAabbMin The bounding box min. bound.
 * \param outAabbMax The bounding box max. bound.
 * \return HIPRT error in case of a failure, hiprtSuccess otherwise.
 */
HIPRT_API hiprtError hiprtExportGeometryAabb( hiprtContext context, hiprtGeometry inGeometry, hiprtFloat3& outAabbMin, hiprtFloat3& outAabbMax );

/** \brief Output scene's AABB.
 *
 * \param context HIPRT API context.
 * \param inScene Scene to be queried.
 * \param outAabbMin The bounding box min. bound.
 * \param outAabbMax The bounding box max. bound.
 * \return HIPRT error in case of a failure, hiprtSuccess otherwise.
 */
HIPRT_API hiprtError hiprtExportSceneAabb( hiprtContext context, hiprtScene inScene, hiprtFloat3& outAabbMin, hiprtFloat3& outAabbMax );

/** \brief Get Program instance with HIPRT routines.
 * \param functionName function to which handle will be returned, cannot be NULL.
 * \param context HIPRT API context.
 * \param src HIP program source.
 * \param name Program source filename.
 * \param numHeaders Number of headers, numHeaders must be greater than or equal to 0.
 * \param headers Sources of the headers, headers can be NULL when numHeaders is 0.
 * \param includeNames Name of each header by which they can be included in the HIP program source, includeNames can be NULL
 * when numHeaders is 0. 
 * \param options Compiler options, can be NULL.
 * \param progOut Output build program instance.
 * \return HIPRT error in case of a failure, hiprtSuccess otherwise.
 */
HIPRT_API hiprtError hiprtBuildTraceProgram(
	hiprtContext	  context,
	const char*		  functionName,
	const char*		  src,
	const char*		  name,
	int				  numHeaders,
	const char**	  headers,
	const char**	  includeNames,
	const char**	  options,
	int				  nOptions,
	void* progOut );

/** \brief Get binary with HIPRT routines.
 *
 * \param prog program instance.
 * \param size Output size of binary .
 * \param binary Output if NULL function returns size of parameter else returned binary(application should allocate for binary)..
 * \return HIPRT error in case of a failure, hiprtSuccess otherwise.
 */
HIPRT_API hiprtError hiprtBuildTraceGetBinary(
	void* prog, 
	size_t* size, 
	void* binary);

/** \brief Setting log level.
 *
 * \param path user defined path to cache kernels.
 */
HIPRT_API void hiprtSetCacheDirPath( 
	const char* path );


/** \brief Setting log level.
 *
 * \param level Desired log level.
 */
HIPRT_API void hiprtSetLogLevel( int level = 0 );

#ifdef __cplusplus
}
#endif

#endif
