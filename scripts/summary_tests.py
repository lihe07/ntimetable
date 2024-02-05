import os
import json


for project in os.listdir("converted"):
    if os.path.isdir(os.path.join("converted", project)):
        index = int(project.strip("comp"))

        # 1. events.json
        events = len(json.load(open(os.path.join("converted", project, "events.json"))))

        # 2. Rooms
        rooms = len(json.load(open(os.path.join("converted", project, "rooms.json"))))
        # 3. People
        #
        people = len(json.load(open(os.path.join("converted", project, "people.json"))))
        # 4. Criterias
        criterias = (
            len(
                json.load(open(os.path.join("converted", project, "criteria.json")))[
                    "event_time_distance"
                ]
            )
            + 1
        )

        print(f"${index}$ & {events} & {rooms} & {people} & {criterias} \\\\")
