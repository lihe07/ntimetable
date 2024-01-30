import seaborn as sns
import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
from scipy.ndimage import gaussian_filter1d

sns.set_style("darkgrid")

names = [
    "relocation",
    "greedy_room",
    "room_only",
    "time_only",
    "time_room",
]

with open("./history_weights.txt") as f:
    history = f.readlines()  # [...] \n [...] \n [...] \n


history = [eval(x) for x in history]  # convert string to list
history = np.array(history)  # convert list to numpy array
data = pd.DataFrame(history, columns=names)  # convert numpy array to dataframe


# Draw line plot

plt.figure(figsize=(10, 6))

# with smooth (gaussian filter)
for name in names:
    plt.plot(
        gaussian_filter1d(data[name], sigma=3),
        label=name,
    )


plt.legend()

plt.tight_layout()

plt.savefig("./history_weights.png")
