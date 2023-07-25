import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';

@Injectable({
  providedIn: 'root'
})
export class MapSuburbsService {
  apiUrl = 'https://witpa.codelog.co.za/api/fetchMapData'

  body = {
    "bottomLeft": [-90, -180],
    "topRight": [90, 180]
  }

  constructor(private httpClient: HttpClient) { }

  getSuburbData() {
    return this.httpClient.post(this.apiUrl, this.body);
  }
}
