import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class ScheduleService {
  apiUrl = 'https://witpa.codelog.co.za/api/fetchScheduleData'

  constructor(private httpClient: HttpClient) { }

  getScheduleData(suburb: number) {
    let body = {
      "suburbId": suburb
    }

    return this.httpClient.post<number>(this.apiUrl, body);
  }

  getLoadSheddingStage() {
    return this.httpClient.get('https://witpa.codelog.co.za/api/fetchCurrentStage');
  }
}
