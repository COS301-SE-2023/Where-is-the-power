import { Component } from '@angular/core';
import { UserLocationService } from '../user-location.service';
import { SavedPlacesService } from '../tab-saved/saved-places.service';

@Component({
  selector: 'app-tab-navigate',
  templateUrl: 'tab-navigate.page.html',
  styleUrls: ['tab-navigate.page.scss']
})
export class TabNavigatePage {

  constructor(private UserLocationService: UserLocationService, private savedPlacesService: SavedPlacesService) { }
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

  ionViewDidLeave() {
    this.savedPlacesService.navigateToPlace.next(false);
  }
}
