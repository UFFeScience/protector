#! /usr/bin/python3
#
# Automatically generates the .dot for an instance and generates its PDF.
#
# # USAGE
#
# Call it from the root of the directory and pass the instance name.
#
# `./scripts/generate_image.py [INSTANCE_NAME]`
#

import sys
import os

instance = sys.argv[1]
folder = f"instances/{instance}"

args = [f"{folder}/graph"]

for solution in ["heuristic_solution", "optimal_solution"]:
    if solution in os.listdir(folder):
        args.append(f"{folder}/{solution}")

os.system(f"./scripts/dot/dot.py {' '.join(args)}")

has_other_args = len(sys.argv[2:]) > 0

if has_other_args:
    # for big graphs
    os.system(
        f"sfdp -x -Goverlap=scale -Gsep=3 -Tpdf -o {folder}/graph.pdf {folder}/graph.dot")
else:
    # for small graphs
    os.system(f"dot -Tpdf -o {folder}/graph.pdf {folder}/graph.dot")
