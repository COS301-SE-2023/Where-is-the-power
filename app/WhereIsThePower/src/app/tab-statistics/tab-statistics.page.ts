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

      this.processDoughnutChart(data);
      this.processBarChart(data);
    }, 
    (error) => {
        console.error(error);
    });
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

  processBarChart(data: any)
  {
    const daysOfWeek = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    const todayIndex = new Date().getDay(); // 0 for Sunday, 1 for Monday, etc.
    const orderedDaysOfWeek = [...daysOfWeek.slice(todayIndex+1), ...daysOfWeek.slice(0, todayIndex+1)];

    const barData = {
      labels: orderedDaysOfWeek,
      datasets: [
        {
          label: 'Uptime',
          data: orderedDaysOfWeek.map(day => data.result.perDayTimes[day]?.on / 60 || 0), // Uptime(No. of hours without Loadshedding)
          borderColor: '#007A4D',
          backgroundColor: '#007A4D',
        },
        {
          label: 'Downtime',
          data: orderedDaysOfWeek.map(day => data.result.perDayTimes[day]?.off / 60 || 0), // Downtime(Loadshedding hours)
          borderColor: '#DE3831',
          backgroundColor: '#DE3831',
        }
      ]
    };

    this.populateBarChart(barData);
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

