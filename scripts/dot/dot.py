#! /usr/bin/python3
#
# Script to export dot files from graph output

import sys
import exporters as export

args = sys.argv[1:]

if len(args) >= 2:
    input_file = args[0]
    solution_files = args[1:]
    export.full_response(input_file, solution_files)
else:
    print(
        "Expected at least two file inputs: <input> <solution> [solution...]")
