import { Component, ElementRef, OnInit, ViewChild } from '@angular/core';
import { Chart, registerables } from 'chart.js';
Chart.register(...registerables)

@Component({
  selector: 'app-tab-statistics',
  templateUrl: './tab-statistics.page.html',
  styleUrls: ['./tab-statistics.page.scss'],
})
export class TabStatisticsPage implements OnInit {
  @ViewChild('barChart') barChartRef!: ElementRef;
  @ViewChild('doughnutChart') doughnutChartRef!: ElementRef;

  chart: any;
  constructor() { }
  ngOnInit() {
    // Data for Doughnut Chart (Uptime/Downtime for Today)
    const doughnutData = {
      labels: ['Uptime', 'Downtime'],
      datasets: [{
        label: 'Loadshedding',
        data: [20, 4], // Uptime vs Downtime
        borderWidth: 0,
        backgroundColor: [
          '#007A4D',
          '#DE3831',
        ],
      }]
    };
    this.populateDoughnutChart(doughnutData);


    // Data for Bar Chart (Uptime/Downtime for the week)
    const labels = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    const barData = {
      labels: labels,
      datasets: [
        {
          label: 'Uptime',
          data: [20, 16, 20, 20, 12, 20, 24], // Uptime(No. of hours without Loadshedding)
          borderColor: '#007A4D',
          backgroundColor: '#007A4D',
        },
        {
          label: 'Downtime',
          data: [-4, -8, -4, -4, -12, -4, 0], // Downtime(Loadshedding hours)
          borderColor: '#DE3831',
          backgroundColor: '#DE3831',
        }
      ]
    };

    this.populateBarChart(barData);
  }

  populateDoughnutChart(doughnutData: any) {
    this.chart = new Chart("doughnutChart", {
      type: 'doughnut',
      data: doughnutData,
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

  populateBarChart(barData: any) {
    new Chart("barChart", {
      type: 'bar',
      data: barData,
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
