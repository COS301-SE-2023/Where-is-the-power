import sys
import requests
import json
import os

congestion_levels = {
    "low": 1,
    "moderate": 2,
    "heavy": 3,
    "severe": 4
}
congestion_levels_reverse = {v: k for k, v in congestion_levels.items()}


access_token = os.environ.get("MAPBOX_API_KEY")

def get_loadshedding_polygons():
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
        if feature['properties']['PowerStatus'] == 'off':
            polygon = [point for point in feature['geometry']['coordinates'][0] if len(point) == 2]
            sub_polygon = [point for point in feature['geometry']['coordinates'][0] if len(point) > 2]
            temp_polygon = {
                "coordinates" : polygon, 
                "id" : feature['id']
            }
            polygons.append(temp_polygon)
    

    return polygons

def get_exclusion_points(exclude):
    exclusionArea = ""
    first = True
    for robot in exclude:
        if first:
            exclusionArea = f"point({robot[0]} {robot[1]})"
            first = False
        else:
            exclusionArea = exclusionArea + f",point({robot[0]} {robot[1]})"
    return exclusionArea

def get_route(start_coords, end_coords, exclude = None ):
    # Separate into start_long, start_lat, end_long, and end_lat
    start_long, start_lat = start_coords
    end_long, end_lat = end_coords

    endpoint = f"https://api.mapbox.com/directions/v5/mapbox/driving-traffic/{start_long},{start_lat};{end_long},{end_lat}"
    params = {
        "access_token": access_token,
        "alternatives": "true",
        "geometries": "geojson",
        "language": "en",
        "overview": "full",
        "steps": "true"
    }

    if exclude:
        params["exclude"] = get_exclusion_points(exclude)

    response = requests.get(endpoint, params=params).json()

    instructions = []
    for step in  response['routes'][0]['legs'][0]['steps']:
        instructions.append(step['maneuver']['instruction'])

    route = {
        "duration": response['routes'][0]['duration'],
        "distance": response['routes'][0]['distance'],
        "trafficLightsAvoided": exclude,
        "instructions": instructions,
        "coordinates" : response['routes'][0]['geometry']['coordinates']
    }
    return route

def point_in_polygon(point, polygon_coordinates):
    x, y = point

    # Check if the point is outside the bounding box of the polygon
    if(len(polygon_coordinates) == 0):
        return False
    min_x, min_y = min(polygon_coordinates, key=lambda p: p[0])
    max_x, max_y = max(polygon_coordinates, key=lambda p: p[0])

    if x < min_x or x > max_x or y < min_y or y > max_y:
        return False

    # Check if the point is inside the polygon using ray-casting algorithm
    inside = False
    n = len(polygon_coordinates)
    j = n - 1

    for i in range(n):
        xi, yi = polygon_coordinates[i]
        xj, yj = polygon_coordinates[j]

        if ((yi > y) != (yj > y)) and (x < (xj - xi) * (y - yi) / (yj - yi) + xi):
            inside = not inside

        j = i
    return inside

#returns all the polygons that are on the route that currently have loadshedding
def get_route_polygon_ids(route): 
    route_polygons = []
    coordinates = route['coordinates']
    polygons = get_loadshedding_polygons()
    
    
    for polygon in polygons:
        for coordinate in coordinates:
            if point_in_polygon(coordinate,polygon["coordinates"]):
                route_polygons.append(polygon["id"])
                break
    return route_polygons

def get_congestion(coordinate):
    traffic_endpoint = f"https://api.mapbox.com/v4/mapbox.mapbox-traffic-v1/tilequery/{coordinate[0]},{coordinate[1]}.json"
    traffic_params = {
        "access_token": access_token,
        "radius": 1,
        "dedupe": "false"
    }
    traffic_response = requests.get(traffic_endpoint, traffic_params).json()
    congestion_level = 1
    for feature in traffic_response['features']:
        congestion = feature['properties']['congestion']
        if congestion_levels[congestion] > congestion_level:
            congestion_level = congestion_levels[congestion]
    return congestion_levels_reverse[congestion_level]

def get_bad_robots(robots):
    bad_robots = []
    for robot in robots:
        if get_congestion(robot) in ["heavy", "severe"]:
            bad_robots.append(robot)
    return bad_robots

def get_route_robots(route):
    #only returns the robots with loadshedding, along the route
    ids = get_route_polygon_ids(route)
    robots = []
    polygons = []
    with open("robots.json", 'r') as file:
        polygons = json.load(file)["robots"]

    for polygon in polygons:
        if polygon["polygon_id"]:
            if polygon["polygon_id"] in ids:
                robots.extend(polygon["coordinates"])
    


    return get_bad_robots(robots)


            


parsed_data = json.loads(sys.argv[1])
origin = parsed_data["origin"]
destination = parsed_data["destination"]

# first_route = get_route([28.300946,-25.729694],[28.322388,-25.741065])
# second_route = get_route([28.300946,-25.729694],[28.322388,-25.741065],get_route_robots(first_route))

first_route = get_route(origin,destination)
second_route = get_route(origin,destination,get_route_robots(first_route))

json_response = json.dumps(second_route)
print(json_response)

