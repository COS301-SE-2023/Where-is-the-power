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

  constructor(private _httpClient: HttpClient) { }

  getSuburbData() {

  }
}
