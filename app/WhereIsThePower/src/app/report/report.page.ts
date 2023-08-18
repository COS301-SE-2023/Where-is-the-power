import { Component, OnInit } from '@angular/core';
import { ReportService } from './report.service';
import { Subscription } from 'rxjs';
import { Router } from '@angular/router';

@Component({
  selector: 'app-report',
  templateUrl: './report.page.html',
  styleUrls: ['./report.page.scss'],
})
export class ReportPage implements OnInit {
  private createReportSubscription: Subscription = new Subscription();

  constructor(
    private reportService: ReportService,
    private router: Router
  ) { }

  ngOnInit() {
    this.reportService.getReports().subscribe((data) => {
      console.log(data);
    });
  }

  report(reportType: string) {
    this.createReportSubscription = this.reportService.reportIssue(reportType).subscribe(
      (res: any) => {
        console.log(res);
        this.router.navigate(['/tabs/tab-navigate']);
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


