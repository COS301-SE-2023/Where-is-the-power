# This is a scrapper that connects to www.tshwane.gov.za's last known addresses
#   of the loadshedding schedules. It will then send the data that it has proccessed
#   to the rust rocket server through localhost at a dedicated endpoint. The rocket
#   server will then update the data in the database using this script

import sys
import json
import requests
import pandas as pd
from lxml import etree

from config import *

# Globals
excel_url = "https://www.tshwane.gov.za/wp-admin/admin-ajax.php?juwpfisadmin=false&action=wpfd&task=file.download&wpfd_category_id=293&wpfd_file_id=38390"
groups_url = "https://www.tshwane.gov.za/?page_id=1124#293-293-wpfd-top"
groups:dict[str,list] = {} # group number, areas in group
loadSheddingTimes:dict[str,dict[str,list]] = {} # level, [group, times]

def scrapeHTML():
  html_content = requests.get(groups_url)
  root = etree.HTML(html_content.text)

  # Find all table elements
  table_elements = root.xpath("//table")
  # Find the largest table based on number of rows
  largest_table = max(table_elements, key=lambda table: len(table.xpath(".//tr")))

  # Iterate through rows in the largest table
  for row in largest_table.iter("tr"):
    # Process each row as needed

    cells:list[etree.Element] = row.xpath(".//td")
    cellStrings:list[str] = []
    for cell in cells:
      # get inner text of cells
      celltext = cell.text
      if celltext is not None:
        cellStrings.append(cell.text)
      else:
        cellStrings.append("")

    area:str = cellStrings[0]
    if (area == ""): continue
    area.capitalize()

    # add area to relevant group
    group = cellStrings[1]
    if group not in groups:
      groups[group] = [area]
    else:
      groups[group].append(area)
  if debug: print(groups)

def formatTime(timeIn):
  time = str(timeIn.hour) + ":" + str(timeIn.minute)
  padded_time_string = ":".join([f"{int(x):02d}" for x in time.split(":")])
  return padded_time_string

def scrapeXLSX():
  excel_content = requests.get(excel_url).content
  excel_data = pd.read_excel(excel_content, sheet_name=None)
  #excel_data = pd.ExcelFile("~/Downloads/Tshwane-2-hour-Schedule-and-Loadshedding-Tool-Rev-0-6-Sept-2020.xlsx")
  #print(excel_data.keys())
  schedule = excel_data["Schedule"]
  #schedule = excel_data.parse("Schedule")
  lastTime = ""
  for index,row in schedule.iterrows():
    # skip first 3 rows
    data = row.array.tolist()
    if index >= 2:
      if (index-2)%8 == 0:
        # build data string
        timePeriod = formatTime(data[0]) + "-" + formatTime(data[1])
        lastTime = timePeriod
        # add data
        loadSheddingTimes[timePeriod] = {data[2]:data[3:]}
        continue
      loadSheddingTimes[lastTime].update({data[2]:data[3:]})

  if debug: print(loadSheddingTimes)

# --debug: print out debug information
# --update: send results to the server for updates
# --dry: view output that will be sent to server
if (__name__ == "__main__"):
  if len(sys.argv) < 2: 
    print("see -h for help")
    exit()
  arg1 = sys.argv[1]
  if arg1 == "--debug":
    debug = True
  elif arg1 == "--update":
    update = True
  elif arg1 == "--dry":
    dry = True
  else:
    print("Usage: python tswane_data_scraper.py [mode]")
    print("Mode is mandatory, availible modes are:")
    print(" --debug: print out debug information")
    print(" --update: send results to the server for updates")
    print(" --dry: view output that will be sent to server")
    exit()

  ## impliment multiprocessing
  scrapeHTML()
  scrapeXLSX()
  toSend = {
    "municipality":"tshwane",
    "groups":groups,
    "times":loadSheddingTimes
  }
  jsonData = json.dumps(toSend)
  headers = {
    "Content-Type":"application/json"
  }
  if dry:
    print(jsonData)
  if (update):
    response = requests.post(url=SERVER_IP,data=jsonData,headers=headers)
    print(response)