import json
# data = {}

# # Open the JSON file in read mode
# with open("robots.json", 'r') as file:
#     # Use the json.load() function to load the JSON data from the file
#     data = json.load(file)

# for d in data:
#     if len(d["robots"]) >0:
#         print(d["robots"])


def get_robots():
    robots = []
    # Open the file in read mode
    with open("robots.txt", 'r') as file:
        for line in file:
            lon,lat = line.strip().split(',')
            robots.append([float(lon),float(lat)])  # Use strip() to remove leading/trailing whitespace
    return robots



with open("robots.json", "w") as json_file:
    json_file.write(json.dumps(get_robots(), indent=2))