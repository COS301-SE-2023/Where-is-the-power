import math
import requests
import mapbox_vector_tile
import json

def deg2num(lat_deg, lon_deg, zoom):
  lat_rad = math.radians(lat_deg)
  n = 1 << zoom
  xtile = int((lon_deg + 180.0) / 360.0 * n)
  ytile = int((1.0 - math.asinh(math.tan(lat_rad)) / math.pi) / 2.0 * n)
  return xtile, ytile

def num2deg(xtile, ytile, zoom):
  n = 1 << zoom
  lon_deg = xtile / n * 360.0 - 180.0
  lat_rad = math.atan(math.sinh(math.pi * (1 - 2 * ytile / n)))
  lat_deg = math.degrees(lat_rad)
  return lat_deg, lon_deg

def get_tileset():
    x,y = deg2num(-25.821315,28.260659,10)
    endpoint = f"https://api.mapbox.com/v4/mapbox.mapbox-traffic-v1/10/{x}/{y}.mvt"
    params = {
        "access_token": access_token
    }
    response = requests.get(endpoint, params=params)
    print(response.status_code)
    return response.content

output = mapbox_vector_tile.decode(get_tileset())
json_data = json.dumps(output, indent=4)
# print(output["admin"])

with open("output.json", "w") as f:
    f.write(json_data)

print(f'Data has been saved.')


    