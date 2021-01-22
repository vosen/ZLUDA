import pycuda.autoinit
import pycuda.driver as drv
import pycuda.tools as py_tools
from pathlib import PurePath
import numpy as np
from os import path
import os
import itertools
import sys


# It's impossible to discern what is the type of a buffer, here you can override equality checks
def assert_array_equal_override(kname, idx, arr1, arr2):
    if kname == 'knn_match' and idx == 6:
        arr1_view = np.frombuffer(arr1, dtype=np.dtype([('f1', np.uint32), ('f2', np.uint32), ('f3', np.uint32)]))
        np.ndarray.sort(arr1_view)
        arr2_view = np.frombuffer(arr2, dtype=np.dtype([('f1', np.uint32), ('f2', np.uint32), ('f3', np.uint32)]))
        np.ndarray.sort(arr2_view)
    return np.testing.assert_array_equal(arr1, arr2)


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

def verify_single_dump(input_path):
    print(input_path)
    kernel_name = path.basename(input_path).split("_", 1)[1]
    with open(path.join(input_path, "launch.txt"), "r") as launch_f:
        launch_lines = list(map(int, launch_f.readlines()))
    assert launch_lines[6] == 0  # we don't support dynamic shared memory in replay yet
    module = drv.module_from_file(path.join(input_path, "module.ptx"))
    kernel = module.get_function(kernel_name)
    pre_args = parse_arguments(input_path, "pre")
    kernel_pre_args, host_pre_args = zip(*pre_args)
    kernel(*list(kernel_pre_args), grid=tuple(launch_lines[:3]), block=tuple(launch_lines[3:6]))
    post_args = parse_arguments(input_path, "post")
    _, host_post_args_args = zip(*post_args)
    for idx, (pre_arg, post_arg) in enumerate(zip(host_pre_args, host_post_args_args)):
        if pre_arg is None:
            continue
        try:
            assert_array_equal_override(kernel_name, idx, pre_arg, post_arg)
        except Exception as e:
            print(f"{idx}: {e}")

def main(argv):
    device = drv.Device(0)
    print(device.name())
    input_path = argv[1]
    if os.path.exists(path.join(input_path, "launch.txt")):
        verify_single_dump(input_path)
    else:
        for input_subdir in [path.join(input_path, dir) for dir in os.listdir(input_path)]:
            verify_single_dump(input_subdir)


if __name__ == "__main__":
    main(sys.argv)
