import seaborn as sns
import matplotlib.pyplot as plt
import pandas as pd
from scipy.ndimage import gaussian_filter1d
import json
import sys
import numpy as np
import os

proj = sys.argv[1]

log = os.path.join(proj, "log.json")

sns.set_palette("husl")
sns.set_style("whitegrid")

log = json.load(open(log, "r"))

# initial scores
init = np.array(log["initial_scores"])
# Sqrt each
init = -np.sqrt(-init)
init[0] /= 10
print(init)

fini = np.array(log["solutions_scores"])

for ss in fini:
    ss[0] /= 10
    # calculate delta
    ss = -np.sqrt(-ss)
    delta = ss - init
    print(delta.sum())

data = pd.DataFrame(log["steps"])
print(data.keys())

print(data[["i", "average_scores"]].head())


def process(x):
    x = np.array(x)
    x[0] /= 10
    x = -np.sqrt(-x)
    return np.average(x)


# create sum_average column
data["sum_average"] = data["average_scores"].apply(process)

data["sum_max"] = data["max_scores"].apply(process)

# plot against iteration
plt.figure(figsize=(9, 6))

sns.lineplot(data=data, x="i", y="sum_average", label="Average of scores")
sns.lineplot(data=data, x="i", y="sum_max", label="Average of max scores")

plt.ylabel("")
plt.xlim(0, len(data))
plt.xlabel("Iteration")

plt.tight_layout()
plt.savefig("./max_average.png")
