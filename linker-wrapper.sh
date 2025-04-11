#!/bin/bash

# This is a wrapper script to handle the ldproxy arguments and pass the appropriate ones to the real linker
# Ignore ldproxy-specific arguments
args=()
i=1
while [ $i -le $# ]; do
  arg="${!i}"
  case "$arg" in
    --ldproxy-linker)
      # Skip this argument and its value
      i=$((i+1))
      ;;
    --ldproxy-cwd)
      # Skip this argument and its value
      i=$((i+1))
      ;;
    *)
      # Keep this argument
      args+=("$arg")
      ;;
  esac
  i=$((i+1))
done

# Call the real linker with the filtered arguments
exec xtensa-esp32-elf-gcc "${args[@]}"
