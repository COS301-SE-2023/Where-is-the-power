import { Component, OnInit } from '@angular/core';
import { ReportService } from './report.service';
import { Subscription } from 'rxjs';
import { UserLocationService } from '../user-location.service';

@Component({
  selector: 'app-report',
  templateUrl: './report.page.html',
  styleUrls: ['./report.page.scss'],
})
export class ReportPage implements OnInit {
  private createReportSubscription: Subscription = new Subscription();
  latitude: any;
  longitude: any;

  constructor(
    private reportService: ReportService,
    private userLocationService: UserLocationService  
  ) { }

  ngOnInit() {
    this.latitude = this.userLocationService.getLatitude();
    this.longitude = this.userLocationService.getLongitude();

    this.reportService.getReports().subscribe((data) => {
      console.log(data);
    });
  }

  report(reportType: string) {
    this.createReportSubscription = this.reportService.reportIssue(reportType).subscribe(
      (res: any) => {
        console.log(res);
      },
      (error: any) => {
        console.error('An error occurred:', error);
      }
    );
  }

  ngOnDestroy() {
    if (this.createReportSubscription) {
      this.createReportSubscription.unsubscribe();
    }
  }
}


