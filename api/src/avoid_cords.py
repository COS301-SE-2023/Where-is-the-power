import requests
import math
import json
import sys
import os

access_token = os.environ.get("MAPBOX_API_KEY")


if access_token is None:
    print("No access token")
    exit(-1)


startLong = 28.3
startLat = -25.73
endLong = 28.2651
endLat = -25.7597
# Set up API credentials and endpoint
{startLong},{startLat};{endLong},{endLat}
endpoint = f"https://api.mapbox.com/directions/v5/mapbox/driving/{startLong},{startLat};{endLong},{endLat}";

# # Prepare the API request
params = {
    "access_token": access_token,
    "alternatives": "true",
    "geometries": "geojson",
    "language": "en",
    "overview": "full",
    "steps": "true"
}

# Send the API request
response = requests.get(endpoint, params=params)
# print(response.status_code)

intersections = []

# Load JSON data into a Python dictionary
data = response.json()
for step in data['routes'][0]['legs'][0]['steps']:
    for intersection in step['intersections']:
        if 'traffic_signal' in intersection:
            intersections.append(intersection["location"])
            # print(intersection["location"])

print(json.dumps({"coordsToAvoid": intersections}))
