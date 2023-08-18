import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Router } from '@angular/router';
import { AuthService } from '../authentication/auth.service';

@Injectable({
  providedIn: 'root'
})
export class ReportService {
  apiUrl = 'https://witpa.codelog.co.za/api/reports'
  private headers: HttpHeaders = new HttpHeaders();

  constructor(
    private http: HttpClient,
    private router: Router,
    private authService: AuthService
  ) { }

  getReports() {
    this.headers = this.authService.getAuthHeaders(); // get the auth headers
    return this.http.get(this.apiUrl, { headers: this.headers });
  }

  reportIssue(type: string) {
    let body =
    {
      "report_type": type,
      "timestamp": Date.now()
    }
    this.http.post(this.apiUrl, body, { headers: this.headers }).subscribe((res: any) => {
      console.log(res);
    });
  }
}
