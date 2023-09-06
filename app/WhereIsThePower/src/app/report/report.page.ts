import { Component, OnInit } from '@angular/core';
import { ReportService } from './report.service';
import { Subscription } from 'rxjs';
import { Router } from '@angular/router';
import { AuthService } from '../authentication/auth.service';

@Component({
  selector: 'app-report',
  templateUrl: './report.page.html',
  styleUrls: ['./report.page.scss'],
})
export class ReportPage implements OnInit {
  private createReportSubscription: Subscription = new Subscription();
  isLoggedIn: boolean = false;

  constructor(
    private reportService: ReportService,
    private router: Router,
    private authService: AuthService
  ) { }

  ngOnInit() {
    this.reportService.getReports().subscribe((data) => {
      console.log(data);
    });
  }

  async ionViewWillEnter() {
    this.isLoggedIn = await this.authService.isUserLoggedIn();
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


