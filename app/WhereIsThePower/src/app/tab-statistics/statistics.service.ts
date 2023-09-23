import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { BehaviorSubject } from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class StatisticsService {
  apiUrl = 'https://witpa.codelog.co.za/api/fetchSuburbStats'

  constructor(private httpClient: HttpClient) { }

  getSuburbData(suburb: number) {
    let body = {
      "suburbId": suburb,  
    }
  
    return this.httpClient.post(this.apiUrl, body);
  }
}

