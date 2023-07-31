import sys
import json
import requests

testing_endpoint = "https://witpa.codelog.co.za/api/fetchMapData"
testing_endpoint = "http://127.0.0.1:8000/api/fetchMapData"
def sendRequest():
    body = {
        "bottomLeft": [-90,-180],
        "topRight": [90,180]
    }
    request = json.dumps(body)
    headers = {
      "Content-Type":"application/json"
    }
    response = requests.post(url=testing_endpoint,data=request,headers=headers)
    print(response.text)

if (__name__ == "__main__"):
    sendRequest()
