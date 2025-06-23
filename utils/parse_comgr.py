#!/usr/bin/env python3

import sys
import os
import subprocess
import re
import json
from pathlib import Path

def parse_comgr_logs(log_text):
    actions = []
    block_pattern = r"(?s)amd_comgr_do_action:(.*?)(?=amd_comgr_do_action:|$)"

    line_pattern = re.compile(r'^\s*([\w\s-]+?):\s*(.*)$')

    for block_content in re.finditer(block_pattern, log_text):
        action_data = {}
        lines = block_content.group(1).strip().split('\n')

        for line in lines:
            match = line_pattern.match(line)
            if match:
                key = match.group(1).strip()
                value = match.group(2).strip()

                if key in action_data:
                    if not isinstance(action_data[key], list):
                        action_data[key] = [action_data[key]]
                    action_data[key].append(value)
                else:
                    action_data[key] = value
        
        if action_data:
            actions.append(action_data)

    return actions

def get_files(action):
    arg_pattern = r'("[^"]*")'
    bitcode = None
    shared_obj = None
    for match in re.finditer(arg_pattern, action["Compilation Args"]):
        arg = json.loads(match.group(1))
        if len(arg) == 0:
            continue
        p = Path(arg)
        if p.suffix == '.bc':
            assert bitcode is None
            bitcode = p
        if p.suffix == '.so':
            assert shared_obj is None
            shared_obj = p
    
    return (bitcode, shared_obj)

def dump_bitcode(path):
    command = ["/opt/rocm/llvm/bin/llvm-dis", path, "-o", "-"]
    result = subprocess.run(
        command,
        capture_output=True,
        text=True
    )

    print(result.stdout)

def dump_obj(path):
    command = ["/opt/rocm/llvm/bin/llvm-objdump", "-d", path]
    result = subprocess.run(
        command,
        capture_output=True,
        text=True
    )

    print(result.stdout)

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print(f"Usage: {sys.argv[0]} <command> [args...]", file=sys.stderr)
        sys.exit(1)
            
    command = sys.argv[1:]

    env = os.environ.copy()
    env["AMD_COMGR_SAVE_TEMPS"] = "1"
    env["AMD_COMGR_EMIT_VERBOSE_LOGS"] = "1"
    env["AMD_COMGR_REDIRECT_LOGS"] = "stderr"

    try:
        result = subprocess.run(
            command,
            env=env,
            capture_output=True,
            text=True,
            check=False 
        )

        actions = parse_comgr_logs(result.stderr)

        for action in actions:
            if action["ActionKind"] == "AMD_COMGR_ACTION_COMPILE_SOURCE_TO_EXECUTABLE":
                (bitcode, obj) = get_files(action)
                if bitcode:
                    dump_bitcode(bitcode)
                if obj:
                    dump_obj(obj)

        if result.returncode != 0:
             sys.exit(result.returncode)

    except FileNotFoundError:
        print(f"Error: Command not found: '{command[0]}'", file=sys.stderr)
        sys.exit(127)
    except Exception as e:
        print(f"An unexpected error occurred: {e}", file=sys.stderr)
        sys.exit(1)
