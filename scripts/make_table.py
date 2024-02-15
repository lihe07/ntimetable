import os
import json
import pandas as pd
import numpy as np


def process(x):
    x = np.array(x)
    x[0] /= 10
    x = -np.sqrt(-x)
    return np.average(x)


def summary(proj: str, index: int):
    log = os.path.join(proj, "log.json")
    log = json.load(open(log, "r"))
    # print(log.keys())

    # print("i", process(log["initial_scores"]))
    initial = process(log["initial_scores"])

    steps = pd.DataFrame(log["steps"])
    # print(steps.keys())
    avg_time = steps["neighborhood_grading_time"].mean()

    avg_scores = []

    # Find final scores
    for s in log["solutions_scores"]:
        avg_scores.append(process(s))

    # Index, Steps, Time, Initial Avg, Max Duality Gap, Avg Scores
    avg = " & ".join([f"{x:.2f}" for x in avg_scores[0:10]])
    print(
        f"${index}$ & {avg_time:.2f} & {initial:.2f} & {np.max(avg_scores) - initial:.2f} & {avg} \\\\"
    )


if __name__ == "__main__":
    NUM_SOLS = 10

    header = " & ".join([f"S{i + 1}" for i in range(NUM_SOLS)])
    print(f"Index & IT & IA & MDG & {header} \\\\")
    # summary("./converted/comp03", 3)
    for i in range(1, 22):
        s = str(i).zfill(2)
        summary(f"./converted/comp{s}", i)
