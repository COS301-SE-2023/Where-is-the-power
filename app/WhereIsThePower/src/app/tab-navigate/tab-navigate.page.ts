import { Component } from '@angular/core';
import { UserLocationService } from '../user-location.service';
import { SavedPlacesService } from '../tab-saved/saved-places.service';
import { ViewChild } from '@angular/core';
import { MapModalComponent } from '../shared/map-modal/map-modal.component';

@Component({
  selector: 'app-tab-navigate',
  templateUrl: 'tab-navigate.page.html',
  styleUrls: ['tab-navigate.page.scss']
})
export class TabNavigatePage {
  @ViewChild('mapModalComponent', { static: false }) mapModalComponent!: MapModalComponent;

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

    if(this.mapModalComponent && this.mapModalComponent.map)
      this.mapModalComponent.map.resize();
  }

  onLocateUser() {
    this.UserLocationService.getUserLocation();
  }

  ionViewDidLeave() {
    if (this.mapModalComponent && this.mapModalComponent.searchBar) {
      this.mapModalComponent.searchBar.value = "";
    }
    this.savedPlacesService.navigateToPlace.next(false);
    this.savedPlacesService.navigateToSavedPlace.next(false);
  }
}
