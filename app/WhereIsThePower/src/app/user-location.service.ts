import { Injectable } from '@angular/core';
import { Geolocation, GeolocationOptions } from '@capacitor/geolocation';
import { BehaviorSubject } from 'rxjs';
import { AlertController } from '@ionic/angular';

@Injectable({
  providedIn: 'root'
})
export class UserLocationService {
  latitude: number = 0;
  longitude: number = 0;
  isLocationAvailable: BehaviorSubject<boolean> = new BehaviorSubject<boolean>(false);

  constructor(private geolocation: Geolocation, private alertController: AlertController) { }


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
    catch (error) 
    {
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
}
