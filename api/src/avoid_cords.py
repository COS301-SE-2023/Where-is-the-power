import requests
import math
import json
import sys
import os

access_token = os.environ.get("MAPBOX_API_KEY")

if access_token is None:
    print("No access token")
    exit(-1)
input = json.loads(sys.argv[1])

# Extract start and end coordinates
start_coords = input["origin"]
end_coords = input["destination"]

# Separate into start_long, start_lat, end_long, and end_lat
start_long, start_lat = start_coords
end_long, end_lat = end_coords

endpoint = f"https://api.mapbox.com/directions/v5/mapbox/driving/{start_long},{start_lat};{end_long},{end_lat}";

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

intersections = []

# Load JSON data into a Python dictionary
data = response.json()
for step in data['routes'][0]['legs'][0]['steps']:
    for intersection in step['intersections']:
        if 'traffic_signal' in intersection:
            intersections.append(intersection["location"])

def point_in_polygon(point, polygon):
    x, y = point

    # Check if the point is outside the bounding box of the polygon
    if(len(polygon) == 0):
        return False
    min_x, min_y = min(polygon, key=lambda p: p[0])
    max_x, max_y = max(polygon, key=lambda p: p[0])

    if x < min_x or x > max_x or y < min_y or y > max_y:
        return False

    # Check if the point is inside the polygon using ray-casting algorithm
    inside = False
    n = len(polygon)
    j = n - 1

    for i in range(n):
        xi, yi = polygon[i]
        xj, yj = polygon[j]

        if ((yi > y) != (yj > y)) and (x < (xj - xi) * (y - yi) / (yj - yi) + xi):
            inside = not inside

        j = i

    return inside


api_url = 'https://witpa.codelog.co.za/api/fetchMapData'

# Define the request payload (body)
body = {
    "bottomLeft": [-90, -180],
    "topRight": [90, 180]
}

# Send a POST request to the API
response = requests.post(api_url, json=body)

witpaData = response.json()
polygons = []
for feature in witpaData['result']['mapPolygons'][0]['features']:
    if feature['properties']['PowerStatus'] == 'off':
        polygons.append(feature['geometry']['coordinates'][0])

newpolygons = []
for polygon in polygons:
    newpolygon = []
    for point in polygon:
        if len(point) == 2: 
            newpolygon.append(point)
    newpolygons.append(newpolygon)

redIntersections = [] #traffic lights on route that are currently facing loadshedding. 
for point in intersections: 
        for polygon in newpolygons:
            if len(point) == 2: 
                if point_in_polygon(point,polygon):
                    redIntersections.append(point)

exclusionArea = ""

first = True
for point in redIntersections:
    if first:
        exclusionArea = f"point({point[0]} {point[1]})"
        first = False
    else:
        exclusionArea = exclusionArea + f",point({point[0]} {point[1]})"


endpoint = f"https://api.mapbox.com/directions/v5/mapbox/driving/{start_long},{start_lat};{end_long},{end_lat}"

# # Prepare the API request
params = {
    "access_token": access_token,
    "alternatives": "true",
    "geometries": "geojson",
    "language": "en",
    "overview": "full",
    "steps": "true",
}
# Check if exclusionArea is not None before adding it to params
if exclusionArea:
    params["exclude"] = exclusionArea

eresponse = requests.get(endpoint, params=params).json()

# print(eresponse)
instructions = []
for step in  eresponse['routes'][0]['legs'][0]['steps']:
    instructions.append(step['maneuver']['instruction'])

duration = eresponse['routes'][0]['duration']
distance = eresponse['routes'][0]['distance']
coordinates = eresponse['routes'][0]['geometry']['coordinates']

response_data = {
    "duration": duration,
    "distance": distance,
    "trafficLightsAvoided": redIntersections,
    "instructions": instructions,
    "coordinates" : coordinates
}
json_response = json.dumps(response_data)
print(json_response)









