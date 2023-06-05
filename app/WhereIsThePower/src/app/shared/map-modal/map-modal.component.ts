import {
  Component,
  OnInit,
  AfterViewInit,
  ViewChild,
  ElementRef,
  Renderer2
} from '@angular/core';
import { ModalController } from '@ionic/angular';
import { MapModalModule } from './map-modal.module';
import { environment } from 'src/environments/environment';
import { HttpClient } from '@angular/common/http';
import { tap } from 'rxjs/operators';
declare const google: any;

interface Coordinate {
  lat: number;
  lng: number;
}
@Component({
  selector: 'app-map-modal',
  templateUrl: './map-modal.component.html',
  styleUrls: ['./map-modal.component.scss'],
})
export class MapModalComponent implements OnInit, AfterViewInit {
  @ViewChild('map', { static: false }) mapElementRef!: ElementRef;

  constructor(
    private modalCtrl: ModalController,
    private renderer: Renderer2,
    private http: HttpClient 
  ) { }
  map: any;
  coordinates: Coordinate[] = [];
  
  ngOnInit() {   
    const filePath = '/assets/coordinates.txt';
    this.loadCoordinatesFromFile(filePath).subscribe(coordinates => {
      console.log(coordinates); // Log the coordinates once the request is complete
    });
  }

    loadCoordinatesFromFile(filePath: string) {
      return this.http.get(filePath, { responseType: 'text' }).pipe(
        tap(data => {
          const coordinatePairs = data.trim().split(' ');
          for (const pair of coordinatePairs) {
            const [lngStr, latStr] = pair.split(',');
            const lat = parseFloat(latStr);
            const lng = parseFloat(lngStr);
            const coordinate = { lat, lng };
            this.coordinates.push(coordinate);
          }
        })
      );
    }


  convertToCoordinateFormat(data: string): Coordinate {
    const [latStr, lngStr] = data.split(' ');
    const lat = parseFloat(latStr);
    const lng = parseFloat(lngStr);
    return { lat, lng };
  }

  ngAfterViewInit() {
    this.getGoogleMaps()
      .then(googleMaps => {
        const mapEl = this.mapElementRef.nativeElement;
        const map = new googleMaps.Map(mapEl, {
          center: { lat: -25.774, lng: 28.261 },
          zoom: 10
        });

        googleMaps.event.addListenerOnce(map, 'idle', () => {
          this.renderer.addClass(mapEl, 'visible');
        });

        //console.log(this.coordinates);

        // Define the LatLng coordinates for the polygon's path.
        const triangleCoords =  this.coordinates;
        // Construct the polygon.
        const Triangle = new google.maps.Polygon({
          paths: triangleCoords,
          strokeColor: "#FF0000",
          strokeOpacity: 0.8,
          strokeWeight: 2,
          fillColor: "#FF0000",
          fillOpacity: 0.35,
        });
        Triangle.setMap(map);

        // tap for locations
        map.addListener('click', (event: { latLng: { lat: () => any; lng: () => any; }; }) => {
          const selectedCoords = {
            lat: event.latLng.lat(),
            lng: event.latLng.lng()
          };
          this.modalCtrl.dismiss(selectedCoords);
        });
      })
      .catch(err => {
        console.log(err);
      });

  }

  onCancel() {
    this.modalCtrl.dismiss();
  }

  private getGoogleMaps(): Promise<any> {
    const win = window as any;
    const googleModule = win.google;
    if (googleModule && googleModule.maps) {
      return Promise.resolve(googleModule.maps);
    }
    return new Promise((resolve, reject) => {
      const script = document.createElement('script');
      script.src =
        'https://maps.googleapis.com/maps/api/js?key=' + environment.googleMapsApiKey;
      script.async = true;
      script.defer = true;
      document.body.appendChild(script);
      script.onload = () => {
        const loadedGoogleModule = win.google;
        if (loadedGoogleModule && loadedGoogleModule.maps) {
          resolve(loadedGoogleModule.maps);
        } else {
          reject('Google maps SDK not available.');
        }
      };
    });
  }
}
