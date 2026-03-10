import matplotlib.pyplot as plt
import argparse
from matplotlib.ticker import (MultipleLocator)

parser = argparse.ArgumentParser(
    description="gerar grafico com base nos dados de execução")
parser.add_argument("file_path")
args = parser.parse_args()

file = open(args.file_path, "r")

execution = {
    "CONSTRUCTION": [[], []],
    "LOCAL_SEARCH": [[], []],
    "DATA_MINING": []
}

last_iteration = 0
for line in file:
    if not line.startswith("ITER"):
        continue

    # Expected line structure for construction|local search is:
    #
    # ITER <number> CONSTRUCTION|LOCAL_SEARCH <score>
    #
    # Expected line structure for data mining is:
    #
    # ITER <number> DATA_MINING
    #

    words = line.split()
    iteration = int(words[1])

    # Used to gather data only from the first execution.
    # This condition should be true only when getting a line from the next execution.
    if iteration < last_iteration:
        break
    else:
        last_iteration = iteration

    identifier = words[2]

    if identifier == "DATA_MINING":
        execution[identifier].append(iteration)
    elif identifier in ["CONSTRUCTION", "LOCAL_SEARCH"]:
        score = float(words[3])
        execution[identifier][0].append(iteration)
        execution[identifier][1].append(score)

construction = execution["CONSTRUCTION"]
local_search = execution["LOCAL_SEARCH"]

# Plotting

fig, ax = plt.subplots()

x_locator = MultipleLocator(20)
x_locator.view_limits(0, 500)

ax.set_title("Instance Execution")
ax.set_xlabel("iterations")
ax.set_ylabel("score")

# ax.xaxis.set_major_locator(x_locator)
# ax.yaxis.set_major_locator(MultipleLocator(50))
ax.set_xlim(0, 500)

ax.plot(*construction, label="Construction", marker="x", color="purple")
ax.plot(*local_search, label="Local Search", marker="x", color="green")

# Plot data mining vertical lines
trans = ax.get_xaxis_transform()
data_mining = execution["DATA_MINING"]
for i in data_mining:
    ax.axvline(x=i, linestyle="dotted")

# Plot data mining legend
for i in data_mining[:1]:
    ax.axvline(x=i, label="Data Mining", linestyle="dotted")

ax.legend()
plt.show()
