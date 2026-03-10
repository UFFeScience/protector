#! /usr/bin/python3

# Call this script to run all instances present in the "instances" folder.
# Pass anything as argument to make the script do a release run.
# If you want to pass options to the binary's execution, pass them after a '--'
#
# Examples (all from the root of the repository):
#
# ./scripts/run_instances.py > output.txt # It runs as debug by default
# 
# ./scripts/run_instances.py -- -q > output.txt # runs as debug and pass the '-q' flag to the binary.
# 
# ./scripts/run_instances.py 1 > output.txt # It runs as release because I passed something ("1") to the script.
#
# ./scripts/run_instances.py release -- -q -v > output.txt # Runs as release and pass the flags ['-q', '-v'] to the binary.

import os
import sys

args = sys.argv[1:]

try:
    index = args.index('--')
except:
    index = None

if index != None:
    script_args, program_args = args[:index], args[index + 1:]
else:
    script_args = args
    program_args = []

# defaults to Debug
compilation_mode = ""
binary = "./target/debug/main"

if len(script_args) > 0:
    compilation_mode = "--release"
    binary = "./target/release/main"

os.system(f"cargo build {compilation_mode} --bin main")

instances = os.listdir('./instances')

for instance in instances:
    print(f"\n{instance.upper()}\n", flush=True)

    # Get value of the optimal solution to compare with.
    try:
        optimal = open(f"./instances/{instance}/optimal_solution")
        print(optimal.readline(), flush=True)
        optimal.close()
    except:
        pass

    os.system(f"{binary} instances/{instance}/graph {' '.join(program_args)}")
