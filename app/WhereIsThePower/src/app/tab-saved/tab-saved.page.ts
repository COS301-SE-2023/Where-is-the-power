import { Component } from '@angular/core';
import { Geolocation } from '@capacitor/geolocation'
import { UserLocationService } from '../user-location.service';

@Component({
  selector: 'app-tab-saved',
  templateUrl: 'tab-saved.page.html',
  styleUrls: ['tab-saved.page.scss']
})
export class TabSavedPage {
  latitude: any;

  constructor(private userLocationService: UserLocationService) {}

  ngOnInit() {
    this.userLocationService.getUserLocation();
  }

  ionViewDidEnter() {
    this.latitude = this.userLocationService.getLatitude();
  }

  
  
}
