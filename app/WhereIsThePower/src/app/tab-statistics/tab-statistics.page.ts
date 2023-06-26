import { Component, ElementRef, OnInit, ViewChild } from '@angular/core';
import { Chart, registerables } from 'chart.js';
Chart.register(...registerables)

@Component({
  selector: 'app-tab-statistics',
  templateUrl: './tab-statistics.page.html',
  styleUrls: ['./tab-statistics.page.scss'],
})
export class TabStatisticsPage implements OnInit {
  @ViewChild('barChartRef') barChartRef!: ElementRef;
  //@ViewChild('doughnutChart') doughnutChartRef!: ElementRef;

  chart: any;
  constructor() { }
  ngOnInit() { }

  ionViewDidEnter() {
    this.chart = new Chart("doughnutChart", {
      type: 'doughnut',
      data: {
        labels: ['Uptime', 'Downtime'],
        datasets: [{
          label: 'Loadshedding',
          data: [20, 4],
          borderWidth: 0,
          backgroundColor: [
            '#007A4D',
            '#DE3831',
          ],
        }]
      },
      options: {
        responsive: true,
        plugins: {
          legend: {
            position: 'top',
          },
        }
      }
    });


  }
}
