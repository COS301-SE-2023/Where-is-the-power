import { Injectable } from '@angular/core';
import { Geolocation } from '@capacitor/geolocation'
import { BehaviorSubject } from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class UserLocationService {
  latitude: number = 0;
  longitude: number = 0;
  isLocationAvailable: BehaviorSubject<boolean> = new BehaviorSubject<boolean>(false);

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
    this.isLocationAvailable.next(true);
    this.latitude = coordinates.coords.latitude;
    this.longitude = coordinates.coords.longitude;
  };
}
