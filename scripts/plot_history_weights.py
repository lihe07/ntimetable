import seaborn as sns
import matplotlib.pyplot as plt
import pandas as pd
from scipy.ndimage import gaussian_filter1d
import json
import sys
import os

proj = sys.argv[1]

log = os.path.join(proj, "log.json")

# sns.set_style("darkgrid")
sns.set_palette("husl")
sns.set_style("whitegrid")

names = [
    "relocation",
    "greedy_room",
    "room_only",
    "time_only",
    "time_room",
]

# with open("./history_weights.txt") as f:
#     history = f.readlines()  # [...] \n [...] \n [...] \n
#
#
# history = [eval(x) for x in history]  # convert string to list
# history = np.array(history)  # convert list to numpy array
log = json.load(open(log, "r"))

history = []
for step in log["steps"]:
    history.append(step["weights"])

data = pd.DataFrame(history, columns=names)  # convert numpy array to dataframe


# Draw line plot

plt.figure(figsize=(12, 6))

# with smooth (gaussian filter)
for name in names:
    plt.plot(
        gaussian_filter1d(data[name], sigma=3),
        label=name,
    )


plt.xlabel("Iteration")
plt.xlim(0, len(history))
plt.legend()

plt.tight_layout()

plt.savefig("./history_weights.png")
