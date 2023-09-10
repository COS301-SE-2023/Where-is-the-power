import requests
import json
import math

# length = 120648.3281136583


# 120648.3281136583 =  r.sqrt(2)

# r = 120648.3281136583/sqrt(2)
# = 85311.250948


# 120648.3281136583/2 = 60324.1640568

# Center:
# lat =  (-25.1096 + -26.0781)/2 = -25.59385
# lon = (29.0984 +27.8904)/2 = 28.4944

# maxY: 29.0984
# minY:27.8904
# maxX:-25.1096
# minX:-26.0781

#100 


# access_token = "pk.eyJ1IjoiZG9jdG9yYnVpbGRlciIsImEiOiJjbGtjZGtzYWQwMDdtM3Bsdm90bDJ6dmFpIn0.Z3CBinZUO0h3Pi96_8wBPw"
# lon = 28.2938
# lat = -25.7446
# endpoint = f"https://api.mapbox.com/v4/mapbox.mapbox-streets-v8/tilequery/{lon},{lat}.json"

# # # Prepare the API request
# params = {
#     "access_token": access_token,
#     "layers": "road",
#     "radius": 100,
#     "limit": 50,
# }

# # Send the API request
# response = requests.get(endpoint, params=params).json()



def offset(value, distance_in_meters):
    # Earth radius in meters
    earth_radius = 6371000

    # Calculate the angular distance covered by the given distance
    angular_distance = distance_in_meters / earth_radius

    # Calculate the new value
    lat2 = math.radians(value) + angular_distance

    # Convert the new value from radians back to degrees
    new_value = math.degrees(lat2)

    return new_value

# with open("test.json", "w") as f:
#     f.write(json.dumps(response, indent=2))

squares = []

x_step = 141.4213562373095
y_step = 141.4213562373095

x = -26.0781
while x < -25.1096:
    y = 27.8904
    xo = offset(x,x_step)
    while y < 29.0984:      
        yo = offset(y,y_step)
        xend = xo if xo < 25.1096 else -25.1096
        yend = yo if yo < 29.0984 else 29.0984
        center = [ (y + yend) / 2,(x + xend) / 2]
        squares.append(center)
        y = yo
    x = xo

with open("robots.json", "w") as f:
  f.write(json.dumps(squares,indent=2))

print(len(squares))