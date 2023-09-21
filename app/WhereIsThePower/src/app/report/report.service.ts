import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Router } from '@angular/router';
import { AuthService } from '../authentication/auth.service';
import { UserLocationService } from '../user-location.service';
import { BehaviorSubject, tap } from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class ReportService {
  apiUrl = 'https://witpa.codelog.co.za/api/reports'
  private headers: HttpHeaders = new HttpHeaders();
  latitude: any;
  longitude: any;
  reports: BehaviorSubject<any[]> = new BehaviorSubject<any[]>([]);

  constructor(
    private http: HttpClient,
    private router: Router,
    private authService: AuthService,
    private userLocationService: UserLocationService
  ) { }

  getReports() {
    this.headers = this.authService.getAuthHeaders(); // get the auth headers
    return this.http.get(this.apiUrl, { headers: this.headers }).pipe(tap((res: any) => {
      this.reports.next(res.result);
      console.log("getReports (service file)", res.result);
    }));
  }

  reportIssue(type: string) {
    this.headers = this.authService.getAuthHeaders(); // get the auth headers

    // Get the current user location
    this.latitude = this.userLocationService.getLatitude();
    this.longitude = this.userLocationService.getLongitude();
    let report =
    {
      "report_type": type,
      "timestamp": Date.now(),
      "latitude": this.latitude,
      "longitude": this.longitude
    }
    return this.http.post(this.apiUrl, report, { headers:  this.headers}).pipe(tap((res: any) => {
        if(res) {
          let currentReports = this.reports.getValue();
          console.log("====================================");
          console.log(" this.reports",  this.reports);
          console.log("currentReports", currentReports);
          console.log("report", report);

          console.log("====================================");

          this.reports.next([...currentReports, report]);
        }
    }));
  }
}
