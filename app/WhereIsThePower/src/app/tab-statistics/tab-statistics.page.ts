import { Component, ElementRef, OnInit, ViewChild } from '@angular/core';
import { Chart, registerables } from 'chart.js';
import { StatisticsService } from './statistics.service';
Chart.register(...registerables)

@Component({
  selector: 'app-tab-statistics',
  templateUrl: './tab-statistics.page.html',
  styleUrls: ['./tab-statistics.page.scss'],
})
export class TabStatisticsPage implements OnInit {
  @ViewChild('barChart') barChartRef!: ElementRef;
  @ViewChild('doughnutChart') doughnutChartRef!: ElementRef;

  doughnutChart: any = null;
  barChart: any = null;
  constructor(private statisticsService: StatisticsService) { }
  ngOnInit() {
    const suburbId = 17959;
    this.statisticsService.getSuburbData(suburbId).subscribe((data) => {
      console.log("statisticsService: ",data);
      this. processDoughnutChart(data);
    }, 
    (error) => {
        console.error(error);
    });


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

  processDoughnutChart(data: any)
  {
     // Get today's day name (e.g., "Mon", "Tue", etc.)
     const today = new Date().toLocaleDateString('en-US', { weekday: 'short' });

      // Get the on and off values for today's day from the data
      const todayOnValue = data.result.perDayTimes[today]?.on || 0;
      const todayOffValue = data.result.perDayTimes[today]?.off || 0;
  
      // Convert total uptime and downtime to hours
        const uptimeToday = Math.floor(todayOnValue / 60);
        const downtimeToday = Math.floor(todayOffValue / 60);

      // Data for Doughnut Chart (Uptime/Downtime for Today)
      const doughnutData = {
        labels: ['Uptime', 'Downtime'],
        datasets: [{
          label: 'Loadshedding',
          data: [uptimeToday, downtimeToday], // Uptime vs Downtime
          borderWidth: 0,
          backgroundColor: [
            '#007A4D',
            '#DE3831',
          ],
        }]
      };
      this.populateDoughnutChart(doughnutData);
  }

  populateDoughnutChart(doughnutData: any) {
    this.doughnutChart = new Chart("doughnutChart", {
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
    this.barChart = new Chart("barChart", {
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

  clearDoughnutChart() {
    this.doughnutChart = null;
  }

  clearBarChart() {
    this.barChart = null;
  }

  clearAllCharts() {
    this.clearBarChart();
    this.clearDoughnutChart();
  }
}

