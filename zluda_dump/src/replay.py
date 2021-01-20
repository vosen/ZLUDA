import pycuda.autoinit
import pycuda.driver as drv
import pycuda.tools as py_tools
from pathlib import PurePath
import numpy as np
from os import path
import os
import itertools


def load_arguments(arg_path):
    is_buffer = arg_path.endswith(".buffer")
    with open(arg_path, "rb") as f:
        arg_bytes = f.read()
    if not is_buffer:
        if len(arg_bytes) == 1:
            return np.frombuffer(arg_bytes, dtype=np.uint8)[0], None
        elif len(arg_bytes) == 2:
            return np.frombuffer(arg_bytes, dtype=np.uint16)[0], None
        elif len(arg_bytes) == 4:
            return np.frombuffer(arg_bytes, dtype=np.uint32)[0], None
        elif len(arg_bytes) == 8:
            return np.frombuffer(arg_bytes, dtype=np.uint64)[0], None
        else:
            raise Exception('Incorrect size of {}: {}'.format(arg_path, len(arg_bytes)))
    else:
        buff = np.frombuffer(bytearray(arg_bytes), dtype=np.uint8)
        buff.setflags(write=1, align=1)
        return drv.InOut(buff), buff


def parse_arguments(dump_path, prefix):
    dir = path.join(dump_path, prefix)
    arg_files = os.listdir(dir)
    return [load_arguments(path.join(dir, f)) for f in sorted(arg_files)]


PATH = r"D:\temp\zluda_dump\geekbench_x86_64.exe\4440_knn_match"
kernel_name = path.basename(PATH).split("_", 1)[1]
with open(path.join(PATH, "launch.txt"), "r") as launch_f:
    launch_lines = list(map(int, launch_f.readlines()))
assert launch_lines[6] == 0  # we don't support dynamic shared memory in replay yet
device = drv.Device(0)
print(device.name())
module = drv.module_from_file(path.join(PATH, "module.ptx"))
kernel = module.get_function(kernel_name)
pre_args = parse_arguments(PATH, "pre")
post_args = parse_arguments(PATH, "pre")
kernel_pre_args, host_pre_args = zip(*pre_args)
kernel(*list(kernel_pre_args), block=tuple(launch_lines[:3]), grid=tuple(launch_lines[3:6]))
_, host_post_args_args = zip(*post_args)
for idx, (pre_arg, post_arg) in enumerate(zip(host_pre_args, host_post_args_args)):
    if pre_arg is None:
        continue
    try:
        np.testing.assert_array_equal(pre_arg, post_arg)
    except Exception as e:
        print(f"{idx}: {e}")

