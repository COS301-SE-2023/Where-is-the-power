import sys
import json
import requests

testing_endpoint = "https://witpa.codelog.co.za/api/fetchSuburbStats"
testing_endpoint = "http://127.0.0.1:8000/api/fetchSuburbStats"
schedule_endpoint = "https://witpa.codelog.co.za/api/fetchScheduleData"
schedule_endpoint = "http://127.0.0.1:8000/api/fetchScheduleData"
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
      "suburbId" :18124 
    }
    request = json.dumps(body)
    headers = {
      "Content-Type":"application/json"
    }
    response = requests.post(url=schedule_endpoint,data=request,headers=headers)
    print(response.text)
if (__name__ == "__main__"):
    sendRequest()
    print("=================================================================================")
    print("=================================================================================")
    print("=================================================================================")
    print("=================================================================================")
    print("=================================================================================")
    sendScheduleRequest()

