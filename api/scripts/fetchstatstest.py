import sys
import json
import requests

testing_endpoint = "http://127.0.0.1:8000/api/fetchSuburbStats"
testing_endpoint = "https://witpa.codelog.co.za/api/fetchSuburbStats"
schedule_endpoint = "https://witpa.codelog.co.za/api/fetchScheduleData"
schedule_endpoint = "http://127.0.0.1:8000/api/fetchScheduleData"
maponoff_endpoint = "https://witpa.codelog.co.za/api/fetchTimeForPolygon"
maponoff_endpoint = "http://127.0.0.1:8000/api/fetchTimeForPolygon"
def sendRequest():
    body = {
      "suburbId" : 17959
    }
    request = json.dumps(body)
    headers = {
      "Content-Type":"application/json"
    }
    response = requests.post(url=testing_endpoint,data=request,headers=headers)
    print(response.text)

def sendScheduleRequest():
    body = {
      #"suburbId" :18231
      "suburbId" : 18186
    }
    request = json.dumps(body)
    headers = {
      "Content-Type":"application/json"
    }
    response = requests.post(url=schedule_endpoint,data=request,headers=headers)
    print(response.text)

def sendTimeForPolygonRequest():
    body = {
      #"suburbId" :18057
      #"suburbId" : 18231
      "suburbId" : 18196
    }
    print(body)
    request = json.dumps(body)
    headers = {
      "Content-Type":"application/json"
    }
    response = requests.post(url=maponoff_endpoint,data=request,headers=headers)
    print(response.text)
if (__name__ == "__main__"):
    #sendRequest()
    print("=================================================================================")
    sendScheduleRequest()
    print("=================================================================================")
    #sendTimeForPolygonRequest()

