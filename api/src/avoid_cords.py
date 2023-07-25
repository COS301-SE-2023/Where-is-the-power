import requests
import math
import json
import sys
import os

access_token = os.environ['MAPBOX_API_KEY']


def find_polygon_center(polygon):
    # Subtract 1 to account for the repeated first point
    num_points = len(polygon) - 1
    sum_x = 0
    sum_y = 0

    for i in range(num_points):
        sum_x += polygon[i][0]
        sum_y += polygon[i][1]

    center_x = sum_x / num_points
    center_y = sum_y / num_points

    return [center_x, center_y]

# polygon = [[28.278153, -25.781812], [28.277781, -25.78166], [28.276252, -25.781039], [28.274805, -25.780169], [28.271878, -25.778368], [28.271868, -25.778362], [28.271357, -25.780567], [28.272005, -25.780674], [28.272028, -25.780909], [28.272131, -25.781988], [28.273088, -25.781879], [28.273176, -25.781889], [28.273249, -25.78257], [28.273317, -25.783255], [28.273336, -25.783449], [28.273329, -25.783493], [28.273279, -25.783745], [28.273191, -25.783733], [28.272402, -25.783829], [28.271852, -25.783882], [28.271883, -25.78414], [28.27194, -25.784489], [28.272039, -25.784601], [28.272161, -25.784687], [28.272318, -25.784763], [28.272524, -25.784863], [28.272837, -25.785091], [28.273073, -25.785357], [28.2732, -25.785413], [28.273372, -25.785439], [28.273481, -25.78545], [28.273512, -25.785341], [28.273574, -25.785166], [28.273687, -25.785057], [28.274178, -25.785052], [28.275, -25.784969], [28.275408, -25.784928], [28.275547, -25.784918], [28.275645, -25.784943], [28.275723, -25.784995], [28.275785, -25.785114], [28.275826, -25.785512], [28.275909, -25.785775], [28.276022, -25.785927], [28.276106, -25.785997], [28.276567, -25.786251], [28.276583, -25.786213], [28.276625, -25.786112], [28.276952, -25.785404], [28.27755, -25.785558], [28.278122, -25.785708], [28.278668, -25.785849], [28.27914, -25.785995], [28.279154, -25.785954], [28.279211, -25.78577], [28.279303, -25.785538], [28.279341, -25.785498], [28.279463, -25.785446], [28.279875, -25.785362], [28.279989, -25.785341], [28.280085, -25.785337], [28.28016, -25.785344], [28.280172, -25.785345], [28.280935, -25.78546], [28.281786, -25.785597], [28.281936, -25.785633], [28.282018, -25.785703], [28.282127, -25.785863], [28.282235, -25.785998], [28.28237, -25.786076], [28.283506, -25.786275], [28.284163, -25.786432], [28.28422, -25.78632], [28.284407, -25.785946], [28.284578, -25.785595], [28.284754, -25.785238], [28.284895, -25.784956], [28.284992, -25.784781], [28.284995, -25.784777], [28.2852, -25.784409], [28.285267, -25.784292], [28.285528, -25.783838], [28.285673, -25.783466], [28.284074, -25.783327], [28.283035, -25.78316], [28.282951, -25.783146], [28.281956, -25.782987], [28.281678, -25.782916], [28.280621, -25.782647], [28.280194, -25.782538], [28.278153, -25.781812]]


parsed_data = json.loads(sys.argv[1])
polygon = parsed_data["polygon"]

# print("polygon",polygon)

lon, lat = find_polygon_center(polygon)


def haversine(lat1, lon1, lat2, lon2):
    # Convert latitude and longitude from degrees to radians
    lat1_rad = math.radians(lat1)
    lon1_rad = math.radians(lon1)
    lat2_rad = math.radians(lat2)
    lon2_rad = math.radians(lon2)

    # Haversine formula
    dlat = lat2_rad - lat1_rad
    dlon = lon2_rad - lon1_rad
    a = math.sin(dlat / 2) ** 2 + math.cos(lat1_rad) * \
        math.cos(lat2_rad) * math.sin(dlon / 2) ** 2
    c = 2 * math.atan2(math.sqrt(a), math.sqrt(1 - a))

    # Radius of the Earth (in meters)
    radius_earth = 6371000

    # Calculate the distance in meters
    distance = radius_earth * c
    return distance


def distance_between_points(point1, point2):
    # Euclidean distance between two points in 2D
    return haversine(point1[1], point1[0], point2[1], point2[0])


def find_radius(polygon):
    # finding the longest_diagonal_length
    # Subtract 1 to account for the repeated first point
    num_points = len(polygon) - 1
    longest_diagonal_length = 0

    for i in range(num_points):
        for j in range(i + 2, num_points):
            diagonal_length = distance_between_points(polygon[i], polygon[j])
            longest_diagonal_length = max(
                longest_diagonal_length, diagonal_length)

    return longest_diagonal_length


radius = find_radius(polygon)

# Set up API credentials and endpoint

endpoint = f"https://api.mapbox.com/v4/mapbox.mapbox-streets-v8/tilequery/{lon},{lat}.json"

# # Prepare the API request
params = {
    "access_token": access_token,
    "layers": "road",
    "radius": radius,
    "limit": 50,
}

# Send the API request
response = requests.get(endpoint, params=params)

# Load JSON data into a Python dictionary
data = response.json()

# Function to extract coordinates with type "traffic_signals"


def extract_traffic_signals_coordinates(data):
    coordinates_list = []
    for feature in data['features']:
        if feature['properties']['type'] == 'traffic_signals':
            coordinates = feature['geometry']['coordinates']
            coordinates_list.append(coordinates)
    return coordinates_list


# Call the function to get the coordinates
traffic_signals_coordinates = extract_traffic_signals_coordinates(data)

print(json.dumps({"coordsToAvoid": traffic_signals_coordinates}))
