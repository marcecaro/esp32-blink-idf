#!/usr/bin/env python3
import json
import sys
import os
import shlex
import subprocess

def get_default_includes(compiler):
    try:
        result = subprocess.run(
            [compiler, "-x", "c++", "-E", "-v", "-"],
            input="",
            capture_output=True,
            text=True,
            check=True
        )
        includes = []
        in_block = False
        for line in result.stderr.splitlines():
            if "#include" in line and "search starts here" in line:
                in_block = True
                continue
            if in_block:
                if "End of search list." in line:
                    break
                path = line.strip()
                if path:
                    includes.append(f"-I{path}")
        return includes
    except Exception as e:
        print(f"⚠️ Could not extract default includes: {e}")
        return []

def extract_flags(compile_commands_path, source_file, output_file):
    with open(compile_commands_path, "r") as f:
        data = json.load(f)

    for entry in data:
        if entry["file"].endswith(source_file) or source_file == "all":
            directory = entry["directory"]
            cmd = entry.get("command") or entry.get("arguments")
            args = shlex.split(cmd) if isinstance(cmd, str) else cmd

            resolved_args = []
            compiler = args[0]
            i = 0
            while i < len(args):
                arg = args[i]
                if arg in ("-I", "-D"):
                    if i + 1 < len(args):
                        val = args[i + 1]
                        if arg == "-I":
                            full_path = os.path.abspath(os.path.join(directory, val)) if not os.path.isabs(val) else val
                            resolved_args.append(f"-I{full_path}")
                        else:
                            resolved_args.append(f"-D{val}")
                        i += 2
                        continue
                elif arg.startswith("-I"):
                    val = arg[2:]
                    full_path = os.path.abspath(os.path.join(directory, val)) if not os.path.isabs(val) else val
                    resolved_args.append(f"-I{full_path}")
                elif arg.startswith("-D"):
                    resolved_args.append(arg)
                i += 1

            # 🔧 Add implicit include paths
            implicit = get_default_includes(compiler)
            print(f"🔍 Default includes from toolchain: {implicit}")
            resolved_args.extend(implicit)

            with open(output_file, "w") as out:
                out.write(" ".join(resolved_args))

            print(f"✅ Extracted {len(resolved_args)} flags to {output_file}")
            return

    print(f"❌ Could not find {source_file} in compile_commands.json")
    sys.exit(1)

if __name__ == "__main__":
    if len(sys.argv) != 4:
        print("Usage: extract_bindgen_flags.py <compile_commands.json> <source_file> <output_file>")
        sys.exit(1)

    extract_flags(sys.argv[1], sys.argv[2], sys.argv[3])
