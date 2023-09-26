import { Injectable } from '@angular/core';
import { Geolocation } from '@capacitor/geolocation';
import { BehaviorSubject } from 'rxjs';
import { AlertController } from '@ionic/angular';
import { HttpClient } from '@angular/common/http';

@Injectable({
  providedIn: 'root'
})
export class UserLocationService {
  latitude: number = 0;
  longitude: number = 0;
  isLocationAvailable: BehaviorSubject<boolean> = new BehaviorSubject<boolean>(false);
  suburbData: any;

  constructor(private alertController: AlertController, private http: HttpClient) { }


  getLatitude() {
    return this.latitude;
  }

  getLongitude() {
    return this.longitude;
  }

  getUserLocation = async () => {
    try {
      const coordinates = await Geolocation.getCurrentPosition();

      this.isLocationAvailable.next(true);
      this.latitude = coordinates.coords.latitude;
      this.longitude = coordinates.coords.longitude;
    }
    catch (error) {
      console.error("Error getting user location:", error);
      this.showLocationErrorAlert();
    }
  };

  async showLocationErrorAlert() {
    const alert = await this.alertController.create({
      header: 'Device Location',
      message: 'Please enable location access to use this app.',
      buttons: [
        {
          text: 'OK',
          handler: () => {
            // Do something when the user dismisses the alert
          }
        }
      ]
    });

    await alert.present();
  }

  async getArea() {
    // Fetch the suburb data
    const data = await this.http.get('assets/suburbs.json').toPromise();
    this.suburbData = data;

    console.log("suburbData:", this.suburbData);

    // Check if the user's location is inside any of the polygons and find SP_NAME
    for (const feature of this.suburbData.features) {
      const polygon = feature.geometry.coordinates[0]; // Assuming it's the first ring for simplicity

      if (this.isPointInPolygon(this.latitude, this.longitude, polygon)) {
        return feature; // Found the area, return its SP_NAME
      }
    }

    return null; // Location is not in any known SP_NAME area
  }


  isPointInPolygon(userLat: number, userLon: number, polygon: number[][]) {
    let inside = false;

    for (let i = 0, j = polygon.length - 1; i < polygon.length; j = i++) {
      const xi = polygon[i][0], yi = polygon[i][1];
      const xj = polygon[j][0], yj = polygon[j][1];

      const intersect = ((yi > userLat) !== (yj > userLat)) &&
        (userLon < (xj - xi) * (userLat - yi) / (yj - yi) + xi);
      if (intersect) inside = !inside;
    }

    return inside;
  }
}
