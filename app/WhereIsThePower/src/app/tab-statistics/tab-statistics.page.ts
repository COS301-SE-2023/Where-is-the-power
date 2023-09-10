import { Component, ElementRef, OnInit, ViewChild } from '@angular/core';
import { registerables } from 'chart.js';
import { StatisticsService } from './statistics.service';
import { HttpClient } from '@angular/common/http';
import { Chart } from 'chart.js/auto'
import { Renderer2 } from '@angular/core';
import { UserLocationService } from '../user-location.service';
import { Subscription } from 'rxjs';

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
  searchItems: any[] = [];
  filteredItems: any[] = [];
  geojsonData: any;
  showResultsList = false;
  isLocationProvided = false;
  isAreaFound = false;
  suburbName = "";

  // Subscriptions
  isLocationAvailableSubscription: Subscription = new Subscription();
  suburbDataSubscription: Subscription = new Subscription();
  listSuburbsSubscription: Subscription = new Subscription();

  constructor(
    private statisticsService: StatisticsService,
    private http: HttpClient,
    private renderer: Renderer2,
    private userLocationService: UserLocationService
  ) { }
  async ngOnInit() {
    this.listSuburbsSubscription = this.http.get('assets/suburbs.json').subscribe(data => {
      this.geojsonData = data;
      this.searchItems = this.geojsonData.features.map((feature: any) => ({
        name: feature.properties.SP_NAME,
        id: feature.id
      }));
      this.filteredItems = [...this.searchItems];
      console.log("Search Items:", this.filteredItems);

    });
  }

  async ionViewWillEnter() {
    // Attempt to get location
    await this.userLocationService.getUserLocation();

    // Check if the location is available
    this.isLocationAvailableSubscription = this.userLocationService.isLocationAvailable.subscribe((isLocationAvailable) => {
      this.isLocationProvided = isLocationAvailable;
      console.log("isLocationAvailable (Stats page): ", this.isLocationProvided);
    });

    // Default Statistics: Area stats on user location
    let area = await this.userLocationService.getArea();
    console.log("Area: ", area);
    if (area != null) {
      console.log("Area Name: ", area.properties.SP_NAME);
      console.log("Area ID: ", area.id);
      this.selectSuburb(
        {
          "id": area.id,
          "name": area.properties.SP_NAME
        }
      );
    }
    else {
      console.log("Area is not available outside of City of Tshwane.");
    }
  }

  processDoughnutChart(data: any) {
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
    if (this.doughnutChart) {
      this.doughnutChart.clear();
      this.doughnutChart.destroy();
    }

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

  processBarChart(data: any) {
    const daysOfWeek = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    const todayIndex = new Date().getDay(); // 0 for Sunday, 1 for Monday, etc.
    const orderedDaysOfWeek = [...daysOfWeek.slice(todayIndex + 1), ...daysOfWeek.slice(0, todayIndex + 1)];

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
    if (this.barChart) {
      this.barChart.clear();
      this.barChart.destroy();
    }

    this.barChart = new Chart(this.barChartRef.nativeElement, {
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
    });/*
    console.log("????: ", this.barChart);

    if (this.barChart) {
      this.barChart.clear();

      this.barChart.destroy();
    }*/
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

  onSearch(event: any) {
    const searchTerm = event.srcElement.value;

    if (event.target.value.length > 0) {
      this.showResultsList = true;
    }
    else {
      this.showResultsList = false;
    }
    console.log(searchTerm);
    // Reset items back to all of the items
    this.filteredItems = [...this.searchItems];

    // if the value is an empty string, don't filter the items
    if (!searchTerm) return;

    this.filteredItems = this.searchItems.filter(item => {
      if (item.name && searchTerm) {
        return item.name.toLowerCase().includes(searchTerm.toLowerCase());
      }
      return false;
    });
    console.log("Filtered Items: ", this.filteredItems);
  }

  onBlur() {
    console.log("Search Bar Blurred");
    setTimeout(() => {
      this.showResultsList = false;
    }, 500); // 500ms delay
  }

  selectSuburb(selectedSuburb: any) {
    //this.clearAllCharts();
    console.log(selectedSuburb.name); // Logs the suburb name
    console.log(selectedSuburb.id); // Logs the suburb id
    this.showResultsList = false;
    this.isAreaFound = true;

    this.suburbDataSubscription = this.statisticsService.getSuburbData(selectedSuburb.id).subscribe((data: any) => {
      console.log("statisticsService: ", data);
      if (data.result != null) {
        this.suburbName = selectedSuburb.name;
        this.processDoughnutChart(data);
        this.processBarChart(data);
      }
      else {
        this.isAreaFound = false;
      }
    },
      (error) => {
        console.error(error);
        this.isAreaFound = false;
      });
  }

  ngOnDestroy() {
    this.isLocationAvailableSubscription.unsubscribe();
    this.suburbDataSubscription.unsubscribe();
    this.listSuburbsSubscription.unsubscribe();
  }
}

