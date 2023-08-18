import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Router } from '@angular/router';
import { AuthService } from '../authentication/auth.service';
import { UserLocationService } from '../user-location.service';

@Injectable({
  providedIn: 'root'
})
export class ReportService {
  apiUrl = 'https://witpa.codelog.co.za/api/reports'
  private headers: HttpHeaders = new HttpHeaders();
  latitude: any;
  longitude: any;

  constructor(
    private http: HttpClient,
    private router: Router,
    private authService: AuthService,
    private userLocationService: UserLocationService
  ) { }

  getReports() {
    this.headers = this.authService.getAuthHeaders(); // get the auth headers
    return this.http.get(this.apiUrl, { headers: this.headers });
  }

  reportIssue(type: string) {
    // Get the current user location
    this.latitude = this.userLocationService.getLatitude();
    this.longitude = this.userLocationService.getLongitude();

    let body =
    {
      "report_type": type,
      "timestamp": Date.now(),
      "latitude": this.latitude,
      "longitude": this.longitude
    }
    return this.http.post(this.apiUrl, body, { headers: this.headers });
  }
}
