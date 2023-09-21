import { Component } from '@angular/core';
import { Subscription } from 'rxjs';
import { UserLocationService } from '../user-location.service';
import { ScheduleService } from './schedule.service';
import { HttpClient } from '@angular/common/http';
import { IScheduleTime } from './schedule-time';

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
  isLocationProvided = false;
  isAreaFound = false;
  suburbName = "";
  searchTerm: string = "";
  loadsheddingStage: number = 0;
  chipColor: string = "success";
  months: string[] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
  days: string[] = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];

  loadshedTimes: IScheduleTime[] = [];

  // Subscriptions
  suburbDataSubscription: Subscription = new Subscription();
  listSuburbsSubscription: Subscription = new Subscription();
  loadsheddingStageSubscription: Subscription = new Subscription();
  isLocationAvailableSubscription: Subscription = new Subscription();

  constructor(private userLocationService: UserLocationService,
    private scheduleService: ScheduleService,
    private http: HttpClient,
    ) {}

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

    this.loadsheddingStageSubscription = this.scheduleService.getLoadSheddingStage().subscribe((stage: any) => {
      console.log(stage);
      this.loadsheddingStage = stage;
    });
  }

  async ionViewWillEnter(){
    this.isLocationAvailableSubscription = this.userLocationService.isLocationAvailable.subscribe((isLocationAvailable) => {
      this.isLocationProvided = isLocationAvailable;
      console.log("isLocationAvailable (Schedule page): ", this.isLocationProvided);
    });
  }

  onSearch(event: any) {
    if (this.searchTerm.length > 0) {
      this.showResultsList = true;
    }
    else {
      this.showResultsList = false;
    }
    console.log(this.searchTerm);
    // Reset items back to all of the items
    this.filteredItems = [...this.searchItems];

    // if the value is an empty string, don't filter the items
    if (!this.searchTerm) return;

    this.filteredItems = this.searchItems.filter(item => {
      if (item.name && this.searchTerm) {
        return item.name.toLowerCase().includes(this.searchTerm.toLowerCase());
      }
      return false;
    });
    console.log("Filtered Items: ", this.filteredItems);
  }

  onBlur() {
    console.log("Search Bar Blurred");
    setTimeout(() => {
      this.showResultsList = false;
    }, 200); // 200ms delay
  }

  selectSuburb(selectedSuburb: any) {
    //this.clearAllCharts();
    console.log(selectedSuburb.name); // Logs the suburb name
    console.log(selectedSuburb.id); // Logs the suburb id
    this.showResultsList = false;
    this.isAreaFound = true;

    this.suburbDataSubscription = this.scheduleService.getScheduleData(selectedSuburb.id).subscribe((data: any) => {
      console.log("ScheduleService: ", data);
      if (data.result != null) {
        this.suburbName = selectedSuburb.name;
        this.searchTerm = selectedSuburb.name;
        
        this.loadshedTimes = [];

        data.result.timesOff.forEach((timeOff: any) => {
          let tempScheduleTimes: IScheduleTime = {
            startTime: this.convertToDateTime(timeOff.start),
            endTime: this.convertToDateTime(timeOff.end)
          }

          this.loadshedTimes.push(tempScheduleTimes);
        });

        console.log(this.loadshedTimes);
        
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

  convertToDateTime(utcTime: number) {
    return new Date(1000 * utcTime);
  }

  formatTime(unformattedTime: number) {
    if(unformattedTime < 10) return '0' + unformattedTime;
    return unformattedTime;
  }

  ngOnDestroy() {
    this.suburbDataSubscription.unsubscribe();
    this.listSuburbsSubscription.unsubscribe();
  }
}
