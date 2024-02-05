# Convert ITC data to custom format
from icecream import ic
import os
import shutil
import json
import numpy as np


def write_json(data, dest: str):
    with open(dest, "w") as f:
        f.write(json.dumps(data, indent=4))


def convert(src: str, dest: str):
    ic(src, dest)
    os.mkdir(dest)
    lines = []
    meta = {}
    with open(src, "r") as f:
        lines = f.readlines()
    # Read first 7 lines, Key: value
    for line in lines[:7]:
        key, value = line.split(":")
        key = key.strip().lower()
        value = value.strip()
        if value.isdigit():
            meta[key] = int(value)
        else:
            meta[key] = value

    ic(meta)
    # 1. dest/config.json
    config = {
        "days": meta["days"],
        "slots_per_day": meta["periods_per_day"],
        # Consts
        "max_iter": 1000,
        "max_iter_initial": 1000,
        "tabu_size": 50,
        "initial_method": "tabu",
        "population_size": 20,
        "initial_temperature": 1000,
        "penalty_threshold": 0.08,
        "penalty_factor": 10,
        "expected_graded_num": 3000,
        "history_size": 1000,
    }
    write_json(config, os.path.join(dest, "config.json"))

    # 2. courses
    courses = []

    teachers = []

    flag = False
    while True:
        line = lines.pop(0)

        if line.strip() == "COURSES:":
            flag = True
            continue

        if flag:
            if line.strip() == "":
                break

            line = line.strip()
            elems = line.split()
            # <CourseID> <Teacher> <# Lectures> <MinWorkingDays> <# Students>
            courses.append(
                {"name": elems[0], "num_per_week": int(elems[2]), "room_kind": "normal"}
            )
            teachers.append({"name": elems[1], "attend": [elems[0]]})

    write_json(courses, os.path.join(dest, "events.json"))

    # 3. Rooms
    rooms = {}

    flag = False
    while True:
        line = lines.pop(0)
        if line.strip() == "ROOMS:":
            flag = True
            continue
        if flag:
            if line.strip() == "":
                break
            line = line.strip()
            elems = line.split()
            # <RoomID> <Capacity>
            rooms[elems[0]] = "normal"

    write_json(rooms, os.path.join(dest, "rooms.json"))

    # 4. Adjacency Matrix, generate randomly
    len_rooms = len(rooms)
    adjacency_matrix = np.random.rand(len_rooms, len_rooms)
    adjacency_matrix = (adjacency_matrix + adjacency_matrix.T) / 2
    adjacency_matrix = adjacency_matrix * 100
    # Convert to int
    adjacency_matrix = adjacency_matrix.astype(int)
    # make diagonal 0
    np.fill_diagonal(adjacency_matrix, 0)
    ic(adjacency_matrix)

    keys = list(rooms.keys())

    # write to dest/rooms_adj.csv
    with open(os.path.join(dest, "rooms_adj.csv"), "w") as f:
        f.write(",".join(keys) + "\n")
        for i in range(len_rooms):
            f.write(keys[i] + "," + ",".join(map(str, adjacency_matrix[i])) + "\n")

    # 5. Students
    students = []
    flag = False
    while True:
        line = lines.pop(0)
        if line.strip() == "CURRICULA:":
            flag = True
            continue
        if flag:
            if line.strip() == "":
                break
            line = line.strip()
            elems = line.split()
            # <CurriculumID> <# Courses> <MemberID> ... <MemberID>
            students.append({"name": elems[0], "attend": elems[2:]})

    write_json(students + teachers, os.path.join(dest, "people.json"))

    # Criteria
    criteria = {
        "room_distance": [{}],
    }

    # UNAVAILABILITY_CONSTRAINTS
    event_times = []

    flag = False
    while True:
        line = lines.pop(0)
        if line.strip() == "UNAVAILABILITY_CONSTRAINTS:":
            flag = True
            continue
        if flag:
            if line.strip() == "":
                break
            line = line.strip()
            elems = line.split()
            # <CourseID> <Day> <Period>
            event_times.append(
                {
                    "kind": "max",
                    "event": elems[0],
                    "time": int(elems[2]),
                }
            )

    criteria["event_time_distance"] = event_times

    write_json(criteria, os.path.join(dest, "criteria.json"))


def convert_all(dir: str):
    shutil.rmtree("./converted", ignore_errors=True)
    os.mkdir("./converted")

    for file in os.listdir(dir):
        if file.endswith(".ctt") and file.startswith("comp"):
            src = os.path.join(dir, file)
            dest = os.path.join("./converted", file.split(".")[0])
            convert(src, dest)


if __name__ == "__main__":
    # shutil.rmtree("./converted", ignore_errors=True)
    # os.mkdir("./converted")
    # convert("./datasets/comp01.ctt", "./converted/comp01")
    convert_all("./datasets")
