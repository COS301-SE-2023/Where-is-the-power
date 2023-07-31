import { Component } from '@angular/core';
import { UserLocationService } from '../user-location.service';

@Component({
  selector: 'app-tab-navigate',
  templateUrl: 'tab-navigate.page.html',
  styleUrls: ['tab-navigate.page.scss']
})
export class TabNavigatePage {

  constructor(private UserLocationService: UserLocationService) { }
  isLocationProvide = false;

  async ionViewDidEnter() {
    // Attempt to get location
    this.UserLocationService.getUserLocation();

    // Check if the location is available
    this.UserLocationService.isLocationAvailable.subscribe((isLocationAvailable) => {
      console.log('isLocationAvailable', isLocationAvailable);
      this.isLocationProvide = isLocationAvailable;
    });
  }

  onLocateUser() {
    this.UserLocationService.getUserLocation();
  }
}
