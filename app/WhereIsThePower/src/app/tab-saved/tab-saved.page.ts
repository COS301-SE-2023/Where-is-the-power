import { Component } from '@angular/core';
import { UserLocationService } from '../user-location.service';
import { HttpClient } from '@angular/common/http';

@Component({
  selector: 'app-tab-saved',
  templateUrl: 'tab-saved.page.html',
  styleUrls: ['tab-saved.page.scss']
})
export class TabSavedPage {
  latitude: any;

  constructor(private userLocationService: UserLocationService, private http: HttpClient) {}

  ngOnInit() {
    this.userLocationService.getUserLocation();
  }

  ionViewDidEnter() {
    this.latitude = this.userLocationService.getLatitude();
  }

  input: string | undefined;

  updateResults() {
    this.http.get('https://api.mapbox.com/search/searchbox/v1/suggest?q='+this.input+'&access_token=pk.eyJ1IjoidTE4MDA0ODc0IiwiYSI6ImNsajMzdWh5ZzAwcHAzZXMxc3lveXJmNDgifQ.7P_tuuiC4M_Q1_H5ZF1rTA&session_token&country=za&origin=25,-25').subscribe((data: any) => {
      console.log(data);
    })
  }
  
}
