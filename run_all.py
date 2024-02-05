import os

for d in os.listdir("./converted/"):
    if not os.path.exists(os.path.join("./converted/", d, "log.json")):
        print("Running", d)
        os.system("./target/release/ntimetable ./converted/" + d)
