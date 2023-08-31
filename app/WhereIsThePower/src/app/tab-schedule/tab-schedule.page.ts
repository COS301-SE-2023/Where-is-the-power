import { Component } from '@angular/core';
import { StatisticsService } from '../tab-statistics/statistics.service';
import { HttpClient } from '@angular/common/http';

@Component({
  selector: 'app-tab-schedule',
  templateUrl: 'tab-schedule.page.html',
  styleUrls: ['tab-schedule.page.scss']
})
export class TabSchedulePage {

  searchItems: any[] = [];
  filteredItems: any[] = [];
  geojsonData: any;
  showResultsList = false;

  constructor(private statisticsService: StatisticsService, private http: HttpClient) {
    this.http.get('assets/suburbs.json').subscribe(data => {
      this.geojsonData = data;
      this.searchItems = this.geojsonData.features.map((feature: any) => ({
        name: feature.properties.SP_NAME,
        id: feature.id
      }));
      this.filteredItems = [...this.searchItems];
    });
    const suburbId = 17959;
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
    console.log(this.filteredItems);
  }

  selectSuburb(selectedSuburb: any) {
    //this.clearAllCharts();
    console.log(selectedSuburb.name); // Logs the suburb name
    console.log(selectedSuburb.id); // Logs the suburb id
    this.showResultsList = false;


    this.statisticsService.getSuburbData(selectedSuburb.id).subscribe((data) => {
      console.log("statisticsService: ", data);

    },
      (error) => {
        console.error(error);
      });
  }
}
