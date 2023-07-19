import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';

@Injectable({
  providedIn: 'root'
})
export class MapSuburbsService {
  apiUrl = 'https://witpa.codelog.co.za:8000/api/fetchMapData'

  httpOptions = {
    headers: new HttpHeaders({
      ContentType: 'application/json'
    })
  }

  body = {
    "bottomLeft": [-90, -180],
    "topRight": [90, 180]
  };

  constructor(private httpClient: HttpClient) { }

  getSuburbData() {
    this.httpClient.post(this.apiUrl, this.body, this.httpOptions).subscribe((data) => {
      console.log(data);
    });
  }
}
