import requests


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
            polygons.append(polygon)
    return polygons

def get_exclusion_points(exclude):
    exclusionArea = ""
    first = True

    for robot in exclude:
        point = robot['coordinates']
        if first:
            exclusionArea = f"point({point[0]} {point[1]})"
            first = False
        else:
            exclusionArea = exclusionArea + f",point({point[0]} {point[1]})"
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

#returns all the polygons that are on the route that currently have loadshedding
def get_route_polygons(route): 
    route_polygons = []
    coordinates = route['coordinates']
    polygons = get_loadshedding_polygons()
    shedding_robots = []
    
    for polygon in polygons:
        for coordinate in coordinates:
            if point_in_polygon(coordinate,polygon):
                route_polygons.append(polygon)
                break
    return route_polygons

first_route = get_route([28.300946,-25.729694],[28.322388,-25.741065])
#print(first_route)
polygons = get_route_polygons(first_route)
print(len(polygons))