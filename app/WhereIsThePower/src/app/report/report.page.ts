import { Component, OnInit } from '@angular/core';
import { ReportService } from './report.service';

@Component({
  selector: 'app-report',
  templateUrl: './report.page.html',
  styleUrls: ['./report.page.scss'],
})
export class ReportPage implements OnInit {

  constructor(
    private reportService: ReportService
  ) { }

  ngOnInit() {
  }

  report(reportType: string)
  {
    this.reportService.reportIssue(reportType);
  }
}


