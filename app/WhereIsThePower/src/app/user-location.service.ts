import { Injectable } from '@angular/core';
import { Geolocation } from '@capacitor/geolocation'

@Injectable({
  providedIn: 'root'
})
export class UserLocationService {
  latitude: number = 0;
  longitude: number = 0;

  constructor() { 
  }

  getLatitude() {
    return this.latitude;
  }

  getLongitude() {
    return this.longitude;
  }

  getUserLocation = async () => {
    const coordinates = await Geolocation.getCurrentPosition();
    this.latitude = coordinates.coords.latitude;
    this.longitude = coordinates.coords.longitude;
  };
}
