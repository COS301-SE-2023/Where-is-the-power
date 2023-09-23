import requests
import json

def get_polygons():
    api_url = 'https://witpa.codelog.co.za/api/fetchMapData'

    # Define the request payload (body)
    body = {
        "bottomLeft": [-90, -180],
        "topRight": [90, 180]
    }

    # Send a POST request to the API
    witpaData = requests.post(api_url, json=body).json()

    polygons = []
    for feature in witpaData['result']['mapPolygons'][0]['features']:
            poly_cords = [point for point in feature['geometry']['coordinates'][0] if len(point) == 2]
            sub_polygon = [point for point in feature['geometry']['coordinates'][0] if len(point) > 2]
            polygon = {
                    "id" : feature['id'],
                    "coordinates" : poly_cords,
                    "robots" : []
                }
            polygons.append(polygon)
    return polygons

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

def get_robots():
    robots = []
    # Open the file in read mode
    with open("robots.txt", 'r') as file:
        for line in file:
            lon,lat = line.strip().split(',')
            robots.append([float(lon),float(lat)])  # Use strip() to remove leading/trailing whitespace
    return robots

polygons = get_polygons()
robots = get_robots()

output = {
    "robots" : [{}]
}

for polygon in polygons:
    data = {
        "coordinates": [],
        "polygon_id": polygon["id"]
    }
    for robot in robots:
        if point_in_polygon(robot,polygon["coordinates"]):
            print("yes!")
            data["coordinates"].append(robot)
            output["robots"].append(data)

with open("robots.json", "w") as json_file:
    json_file.write(json.dumps(output, indent=2))
     
