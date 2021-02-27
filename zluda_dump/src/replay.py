import pycuda.autoinit as cu_autoinit
import pycuda.driver as drv
import pycuda.tools as py_tools
from pathlib import PurePath
import numpy as np
from os import path
import os
import itertools
import sys
import json


def format_size(array_format):
    if array_format == drv.array_format.UNSIGNED_INT8 or array_format == drv.array_format.SIGNED_INT8:
        return 1
    if array_format == drv.array_format.UNSIGNED_INT16 or array_format == drv.array_format.SIGNED_INT16 or array_format == drv.array_format.HALF:
        return 2
    if array_format == drv.array_format.UNSIGNED_INT32 or array_format == drv.array_format.SIGNED_INT32 or array_format == drv.array_format.FLOAT:
        return 4
    raise NotImplementedError


def array_bytes_width(cu_descriptor):
    return cu_descriptor.width * cu_descriptor.num_channels * format_size(cu_descriptor.format)


def texref_set_data(kernel_launch, buffers, texref, texref_details):
    allocation = texref_details['allocation']
    if 'Array' in texref_details['address']:
        set_array = texref_details['address']['Array']
        array_details = kernel_launch['allocations'][str(allocation['buffer_key'])]
        cu_descriptor = drv.ArrayDescriptor3D()
        cu_descriptor.width = array_details['Width']
        cu_descriptor.height = array_details['Height']
        cu_descriptor.depth = array_details['Depth']
        cu_descriptor.format = drv.array_format(array_details['Format'])
        cu_descriptor.num_channels = array_details['NumChannels']
        cu_descriptor.flags = array_details['Flags']
        array = drv.Array(cu_descriptor)
        if allocation['offset_into_buffer'] != 0:
            raise NotImplementedError
        copy_desc = drv.Memcpy3D()
        copy_desc.set_dst_array(array)
        copy_desc.set_src_device(buffers[str(allocation['buffer_key'])][0])
        copy_desc.width_in_bytes = array_bytes_width(cu_descriptor)
        copy_desc.depth = max(cu_descriptor.depth, 1)
        copy_desc.height = max(cu_descriptor.height, 1)
        copy_desc()
        texref_format, texref_num_channels = texref.get_format()
        if set_array["flags"] != 0 or texref_format != array_details['Format'] or texref_num_channels != \
                array_details['NumChannels']:
            raise NotImplementedError
        texref.set_array(array)
    elif 'OneD' in texref_details['address']:
        if allocation['offset_into_buffer'] != 0:
            raise NotImplementedError
        buffer = buffers[str(allocation['buffer_key'])][0]
        texref.set_address(buffer, texref_details['address']['OneD']['bytes'])
        pass
    elif 'TwoD' in texref_details['address']:
        buffer = int(buffers[str(allocation['buffer_key'])][0]) + allocation['offset_into_buffer']
        twod_set = texref_details['address']['TwoD']
        desc = drv.ArrayDescriptor()
        desc.width = twod_set['width']
        desc.height = twod_set['height']
        desc.format = drv.array_format(twod_set['format'])
        desc.num_channels = twod_set['channels']
        texref.set_address_2d(buffer, desc, texref_details['address']['TwoD']['pitch'])
        pass
    else:
        raise NotImplementedError


def load_buffers(input_path, subpath, mapper):
    buffer_dir = path.join(input_path, subpath)
    result = {}
    for buffer in os.scandir(buffer_dir):
        with open(buffer.path, "rb") as buffer_file:
            arg_bytes = buffer_file.read()
        result[buffer.name] = (mapper(arg_bytes), len(arg_bytes))
    return result


def global_copy_data(module, buffers, global_name, global_details):
    devptr, global_size = module.get_global(global_name)
    buffer, buffer_size = buffers[str(global_details['buffer_key'])]
    drv.memcpy_dtod(devptr, int(buffer) + global_details['offset_into_buffer'], global_size)
    #drv.memcpy_dtod(devptr, buffer, global_size)


def round_to_nearest(n, m):
    return (n + m - 1) // m * m


# pycuda does not respect alignment when packing arguments, we have to do it ourselves
def insert_padding(value, start, align):
    offset = round_to_nearest(start, align) - start
    value[0:0] = bytearray(offset)
    return offset


def verify_single_dump(input_path):
    print(input_path)
    with open(path.join(input_path, "kernel_launch.json"), "r") as launch_f:
        kernel_launch = json.load(launch_f)
    module = drv.module_from_file(path.join(input_path, "module.ptx"))
    pre = load_buffers(input_path, "pre", drv.to_device)
    post = load_buffers(input_path, "post", lambda x: x)
    kernel = module.get_function(kernel_launch['name'])
    grid = (kernel_launch['parameters']['gridDimX'], kernel_launch['parameters']['gridDimY'],
            kernel_launch['parameters']['gridDimZ'])
    block = (kernel_launch['parameters']['blockDimX'], kernel_launch['parameters']['blockDimY'],
             kernel_launch['parameters']['blockDimZ'])
    shared = kernel_launch['parameters']['sharedMemBytes']
    for (texref_name, texref_details) in kernel_launch['texrefs'].items():
        texref = module.get_texref(texref_name)
        texref.set_format(drv.array_format(texref_details['format']), texref_details['num_channels'])
        texref.set_address_mode(0, drv.address_mode(texref_details['address_mode'][0]))
        texref.set_address_mode(1, drv.address_mode(texref_details['address_mode'][1]))
        texref.set_flags(texref_details['flags'])
        # other setters are not implemented by pycuda
        texref_set_data(kernel_launch, pre, texref, texref_details)
    for (global_name, global_details) in kernel_launch['globals'].items():
        global_copy_data(module, pre, global_name, global_details)
    explicit_args = []
    start = 0
    for explicit_arg in kernel_launch['explicit_arguments']:
        value = bytearray(explicit_arg['data']['ptr'])
        size = explicit_arg['data']['layout']['size']
        align = explicit_arg['data']['layout']['align']
        for buffer_use in explicit_arg['buffers']:
            arg_offset = buffer_use['offset_into_argument']
            buff_offset = buffer_use['buffer']['offset_into_buffer']
            if buff_offset != 0:
                raise NotImplementedError
            buff_key = buffer_use['buffer']['buffer_key']
            new_devptr = int(pre[str(buff_key)][0]).to_bytes(8, sys.byteorder)
            value[arg_offset:arg_offset + len(new_devptr)] = new_devptr
            pass
        start += insert_padding(value, start, align)
        start += size
        explicit_args.append(np.frombuffer(value, dtype=np.uint8))
    kernel(*explicit_args, grid=grid, block=block, shared=shared)
    cu_autoinit.context.synchronize()
    for (buffer_name, (expected_buffer, _)) in post.items():
        device_buffer = pre[buffer_name]
        host_buffer = drv.from_device(device_buffer[0], (device_buffer[1],), np.uint8)
        expected_buffer_np = np.frombuffer(expected_buffer, dtype=np.uint8)
        #host_buffer = np.frombuffer(host_buffer, dtype=np.single)
        #expected_buffer_np = np.frombuffer(expected_buffer_np, dtype=np.single)
        try:
            np.testing.assert_array_equal(expected_buffer_np, host_buffer)
        except Exception as e:
            print(f"{buffer_name}: {e}")
            #np.save("f:\\temp\\buffer", host_buffer)


def main(argv):
    device = drv.Device(0)
    print(device.name())
    input_path = argv[1]
    if os.path.exists(path.join(input_path, "kernel_launch.json")):
        verify_single_dump(input_path)
    else:
        for input_subdir in sorted([path.join(input_path, dir_name) for dir_name in os.listdir(input_path)]):
            if os.path.isdir(input_subdir):
                verify_single_dump(input_subdir)


if __name__ == "__main__":
    main(sys.argv)
