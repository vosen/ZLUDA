import os, sys, subprocess

def main(path):
    dirs = os.listdir(path)
    for file in dirs:
        if not file.endswith(".spvtxt"):
            continue
        full_file = os.path.join(path, file)
        print(file)
        spv_file = f"/tmp/{file}.spv"
        # We nominally emit spv1.3, but use spv1.4 feature (OpEntryPoint interface changes in 1.4)
        proc1 = subprocess.run(["spirv-as", "--target-env", "spv1.4", full_file, "-o", spv_file])
        proc2 = subprocess.run(["spirv-dis", spv_file, "-o", f"{spv_file}.dis.txt"])
        proc3 = subprocess.run(["spirv-val", spv_file ])
        if proc1.returncode != 0 or proc2.returncode != 0 or proc3.returncode != 0:
            print(proc1.returncode)
            print(proc2.returncode)
            print(proc3.returncode)

if __name__ == "__main__":
    main(sys.argv[1])
