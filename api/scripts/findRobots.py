#this script finds all the Traffic lights in the city of Tswhane
import requests
import json
import numpy as np
from scipy.spatial import Delaunay 

def get_all_polygons():
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
            polygon = [point for point in feature['geometry']['coordinates'][0] if len(point) == 2]
            sub_polygon = [point for point in feature['geometry']['coordinates'][0] if len(point) > 2]
            polygons.append(polygon)
    return polygons

polygons = get_all_polygons()
coordinates = []

for polygon in polygons:
     for point in polygon:
          coordinates.append(point)

points = np.array(coordinates)
tri = Delaunay(points)
hull = tri.convex_hull
convex_points = points[hull]

print(convex_points)

# def eliminate_points_inside_hull(points, convex_hull):
#     a, b, c = np.cross(points[convex_hull[0]], points[convex_hull[1]])
#     mask = np.dot(points, np.array([a, b])) + c <= 0
#     return points[mask]

# eliminate_points_inside_hull(points, convex_points)
# print(eliminate_points_inside_hull(points, convex_points))
# # Save the coordinates to a file (e.g., "big_polygon_coords.json")
# with open("tshwane", "w") as output_file:
#     json.dump(convex_points.tolist(), output_file, indent=2)