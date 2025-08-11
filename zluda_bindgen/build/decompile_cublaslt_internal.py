# Modified from here: https://github.com/galoget/ghidra-headless-scripts/
# Usage: analyzeHeadless <PROJECT_PATH> cublaslt -import /usr/local/cuda/lib64/libcublasLt.so -scriptPath . -postScript decompile_cublaslt_internal.py

from ghidra.app.decompiler import DecompInterface
from ghidra.util.task import ConsoleTaskMonitor

EXTERNAL_HEADER = "/usr/local/cuda/include/cublasLt.h"

with open(EXTERNAL_HEADER, 'r'):
    header_content = open(EXTERNAL_HEADER, 'r').read()
    decompinterface = DecompInterface()
    decompinterface.openProgram(currentProgram)
    functions = currentProgram.getFunctionManager().getFunctions(True)
    blaslt_functions = []
    monitor = ConsoleTaskMonitor()
    with open("cublasLt_internal.h", "w") as output_file:
        output_file.write("// GENERATED AUTOMATICALLY BY decompile_cublaslt_internal.py. DO NOT EDIT MANUALLY\n")
        output_file.write("extern \"C\" {\n")
        output_file.write("    #define undefined void")
        output_file.write("    #define undefined1 unsigned char")
        output_file.write("    #define undefined4 unsigned int")
        output_file.write("    #define uint unsigned int")
        output_file.write("    #define undefined8 unsigned long long")
        output_file.write("    #define ulong unsigned long long")
        for function in functions:
            function_name = function.getName()
            if not function_name.startswith("cublasLt"):
                continue
            if function_name.format("{}(") in header_content:
                continue
            decompile_results = decompinterface.decompileFunction(function, 0, monitor)
            signature = decompile_results.getDecompiledFunction().getSignature()
            # Ghidra disssasembles cublasLtShutdownCtx to return void, but
            # looking at the assembly I'm convinced it returns a value
            # On the other hand, cublasLtLegacyGemmUtilization* seem to return void
            # TODO: fail if there is a new void-return function
            if function_name == "cublasLtShutdownCtx":
                signature = signature.replace("void", "undefined4")
            output_file.write("    CUBLASWINAPI {}\n".format(signature))
        output_file.write("}\n")
