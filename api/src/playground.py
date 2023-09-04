import requests
congestion_levels = {
    "low": 1,
    "moderate": 2,
    "heavy": 3,
    "severe": 4
}
congestion_levels_reverse = {v: k for k, v in congestion_levels.items()}




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
        params["exclude"] = exclude
    return requests.get(endpoint, params=params).json()

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

def get_robots(data):
    robots = []
    for step in data['routes'][0]['legs'][0]['steps']:
        for intersection in step['intersections']:
            if 'traffic_signal' in intersection:
                robot = {
                    "coordinates" : intersection["location"],
                    "congestion" : get_congestion(intersection["location"])
                }
                robots.append(robot)
    return robots

def print_robot(robot):
    print(robot['coordinates'], '-', robot["congestion"]) 

def print_robots(robots):
    for robot in robots:
        print_robot(robot)
    
def get_exclusion_points(robots):
    exclusionArea = ""
    first = True

    for robot in robots:
        point = robot['coordinates']
        if first:
            exclusionArea = f"point({point[0]} {point[1]})"
            first = False
        else:
            exclusionArea = exclusionArea + f",point({point[0]} {point[1]})"
    return exclusionArea

def get_bad_robots(robots):
    bad_robots = []
    for robot in robots:
        if robot["congestion"] in ["heavy", "severe"]:
            bad_robots.append(robot)
    return bad_robots

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

#traffic lights on route that are currently facing loadshedding.
def get_shedding_robots(robots):  
    shedding_robots = []
    for robot in robots:
            point = robot['coordinates'] 
            for polygon in get_loadshedding_polygons():
                if point_in_polygon(point,polygon):
                    shedding_robots.append(point)
    return shedding_robots

routeData1 = get_route([28.233795,-25.801693],[28.269476,-25.827567])
robots1 = get_robots(routeData1)
print("---------robots of route 1---------- \n")
print_robots(robots1)

print("---------shedding robots---------- \n")
print_robots(get_shedding_robots(robots1))

# routeData2 = mapbox([28.233795,-25.801693],[28.269476,-25.827567],get_exclusion_points(get_bad_robots(robots1)))
# print("---------robots of route 2---------- \n")
# print_robots(get_robots(routeData2))


# duration1 = routeData1['routes'][0]['duration']
# duration2 = routeData2['routes'][0]['duration']
# print("duration1: ", duration1)
# print("duration2: ", duration2)


