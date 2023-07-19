import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';

@Injectable({
  providedIn: 'root'
})
export class MapSuburbsService {
  apiUrl = 'http://witpa.codelog.co.za/api/fetchMapData'

  httpOptions = {
    headers: new HttpHeaders({
      'Content-Type': 'application/json'
    })
  }

  body = {
    "bottomLeft": [-90, -180],
    "topRight": [90, 180]
  }

  constructor(private httpClient: HttpClient) { }

  getSuburbData() {
    return this.httpClient.post(this.apiUrl, this.body, this.httpOptions);
  }
}
