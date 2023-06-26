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

    const labels = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    const data = {
      labels: labels,
      datasets: [
        {
          label: 'Uptime',
          data: [20, 16, 20, 20, 12, 20, 24],
          borderColor: '#007A4D',
          backgroundColor: '#007A4D',
        },
        {
          label: 'Downtime',
          data: [-4, -8, -4, -4, -12, -4, 0],
          borderColor: '#DE3831',
          backgroundColor: '#DE3831',
        }
      ]
    };

    new Chart(this.barChartRef.nativeElement, {
      type: 'bar',
      data: data,
      options: {
        responsive: true,
        plugins: {
          legend: {
            position: 'top',
          },
        },
        scales: {
          x: {
            grid: {
              display: false
            }
          },
          y: {
            grid: {
              display: true
            }
          }
        },
        layout: {
          padding: {
            left: 10,
            right: 10,
            top: 0,
            bottom: 0
          }
        },
      },
    });
  }
}
