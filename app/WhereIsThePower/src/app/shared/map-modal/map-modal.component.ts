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

declare const google: any;

@Component({
  selector: 'app-map-modal',
  templateUrl: './map-modal.component.html',
  styleUrls: ['./map-modal.component.scss'],
})
export class MapModalComponent implements OnInit, AfterViewInit {
  @ViewChild('map', { static: false }) mapElementRef!: ElementRef;

  constructor(
    private modalCtrl: ModalController,
    private renderer: Renderer2
  ) { }
  map: any;

  ngOnInit() { }

  ngAfterViewInit() {
    this.getGoogleMaps()
      .then(googleMaps => {
        const mapEl = this.mapElementRef.nativeElement;
        const map = new googleMaps.Map(mapEl, {
          center: { lat: -25.774, lng: 28.261 },
          zoom: 16
        });

        googleMaps.event.addListenerOnce(map, 'idle', () => {
          this.renderer.addClass(mapEl, 'visible');
        });

        // Define the LatLng coordinates for the polygon's path.
        const triangleCoords = [
          { lat: -25.925, lng: 28.2712 },
          { lat: -25.9227, lng: 28.2384 },
          { lat: -25.9034, lng: 28.185 },
          { lat: -25.9195, lng: 28.181 },
          { lat: -25.9449, lng: 28.1102 },
          { lat: -25.9249, lng: 28.1055 },
          { lat: -25.9274, lng: 28.0908 },
          { lat: -25.9243, lng: 28.0902 },
          { lat: -25.9252, lng: 28.0805 },
          { lat: -25.9209, lng: 28.0772 },
          { lat: -25.9292, lng: 28.0751 },
          { lat: -25.9315, lng: 28.0457 },
          { lat: -25.9264, lng: 28.0392 },
          { lat: -25.9111, lng: 28.0299 },
          { lat: -25.934, lng: 27.9879 },
          { lat: -25.9028, lng: 27.984 },
          { lat: -25.9109, lng: 27.9669 },
          { lat: -25.9105, lng: 27.9629 },
          { lat: -25.9072, lng: 27.9623 },
          { lat: -25.9062, lng: 27.9502 },
          { lat: -25.9079, lng: 27.9493 },
          { lat: -25.908, lng: 27.9467 },
          { lat: -25.9149, lng: 27.9467 },
          { lat: -25.9176, lng: 27.9225 },
          { lat: -25.9079, lng: 27.9011 },
          { lat: -25.9053, lng: 27.901 },
          { lat: -25.9057, lng: 27.8938 },
          { lat: -25.8839, lng: 27.8904 },
          { lat: -25.8828, lng: 27.8995 },
          { lat: -25.8597, lng: 27.8982 },
          { lat: -25.8408, lng: 27.9005 },
          { lat: -25.8368, lng: 27.9395 },
          { lat: -25.8241, lng: 27.9258 },
          { lat: -25.8197, lng: 27.9401 },
          { lat: -25.816, lng: 27.9602 },
          { lat: -25.8005, lng: 27.9572 },
          { lat: -25.7991, lng: 27.9636 },
          { lat: -25.7919, lng: 27.9622 },
          { lat: -25.7871, lng: 27.9632 },
          { lat: -25.7869, lng: 27.9546 },
          { lat: -25.7598, lng: 27.9493 },
          { lat: -25.7596, lng: 27.9527 },
          { lat: -25.7566, lng: 27.9558 },
          { lat: -25.7568, lng: 27.9732 },
          { lat: -25.7514, lng: 27.9795},
          { lat: -25.7514, lng: 27.9482 },
          { lat: -25.6952, lng: 27.9575 },
          { lat: -25.6937, lng: 27.9538 },
          { lat: -25.6729, lng: 27.9519 },
          { lat: -25.6713, lng: 27.9523 },
          { lat: -25.6535, lng: 27.9573 },
          { lat: -25.6477, lng: 27.9571 },
          { lat: -25.6437, lng: 27.9652 },
          { lat: -25.6454, lng: 27.9649 },
          { lat: -25.6348, lng: 27.9806 },
          { lat: -25.6328, lng: 27.9722 },
          { lat: -25.5979, lng: 27.9576 },
          { lat: -25.5926, lng: 27.9815 },
          { lat: -25.5427, lng: 28.0306 },
          { lat: -25.5584, lng: 28.0495 },
          { lat: -25.5596, lng: 28.0524 },
          { lat: -25.5358, lng: 28.0244 },
          { lat: -25.5307, lng: 27.9924 },
          { lat: -25.518, lng: 28.0075 },
          { lat: -25.4841, lng: 28.0119 },
          { lat: -25.4805, lng: 28.0359 },
          { lat: -25.4706, lng: 27.9917 },
          { lat: -25.4245, lng: 27.9697 },
          { lat: -25.4391, lng: 27.9604 },
          { lat: -25.4273, lng: 27.9582 },
          { lat: -25.4263, lng: 27.9456 },
          { lat: -25.4067, lng: 27.9708 },
          { lat: -25.3629, lng: 28.0004 },
          { lat: -25.3594, lng: 28.0447 },
          { lat: -25.4017, lng: 28.0555 },
          { lat: -25.4, lng: 28.0834 },
          { lat: -25.3848, lng: 28.108 },
          { lat: -25.367, lng: 28.1107 },
          { lat: -25.3789, lng: 28.1767 },
          { lat: -25.3659, lng: 28.1768 },
          { lat: -25.3286, lng: 28.2015 },
          { lat: -25.3237, lng: 28.2283 },
          { lat: -25.3542, lng: 28.2826 },
          { lat: -25.3426, lng: 28.2834 },
          { lat: -25.3381, lng: 28.2895 },
          { lat: -25.3383, lng: 28.2896 },
          { lat: -25.3412, lng: 28.2981 },
          { lat: -25.3393, lng: 28.2982 },
          { lat: -25.3104, lng: 28.3792 },
          { lat: -25.2921, lng: 28.3826 },
          { lat: -25.3211, lng: 28.4337 },
          { lat: -25.3224, lng: 28.4393 },
          { lat: -25.3102, lng: 28.5036 },
          { lat: -25.3088, lng: 28.5012 },
          { lat: -25.2654, lng: 28.5133 },
          { lat: -25.2561, lng: 28.5643 },
          { lat: -25.2521, lng: 28.5637 },
          { lat: -25.2494, lng: 28.5948 },
          { lat: -25.2494, lng: 28.6029 },
          { lat: -25.2542, lng: 28.6047 },
          { lat: -25.2535, lng: 28.6044 },
          { lat: -25.2521, lng: 28.6313 },
          { lat: -25.2802, lng: 28.6326 },
          { lat: -25.28, lng: 28.6593 },
          { lat: -25.2348, lng: 28.6643 },
          { lat: -25.2518, lng: 28.6712 },
          { lat: -25.2477, lng: 28.6691 },
          { lat: -25.2414, lng: 28.6757 },
          { lat: -25.2347, lng: 28.6756 },
          { lat: -25.227, lng: 28.6799 },
          { lat: -25.2202, lng: 28.6768 },
          { lat: -25.2171, lng: 28.6791 },
          { lat: -25.212, lng: 28.6786 },
          { lat: -25.2108, lng: 28.6553 },
          { lat: -25.2135, lng: 28.6381 },
          { lat: -25.1866, lng: 28.6381 },
          { lat: -25.1839, lng: 28.6159 },
          { lat: -25.1516, lng: 28.6683 },
          { lat: -25.1385, lng: 28.7101 },
          { lat: -25.1161, lng: 28.7446 },
          { lat: -25.1096, lng: 28.7592 },
          { lat: -25.1497, lng: 28.7574 },
          { lat: -25.1499, lng: 28.7765 },
          { lat: -25.1837, lng: 28.7754 },
          { lat: -25.1838, lng: 28.7846 },
          { lat: -25.2287, lng: 28.7791 },
          { lat: -25.2308, lng: 28.7638 },
          { lat: -25.2211, lng: 28.7324 },
          { lat: -25.2692, lng: 28.7396 },
          { lat: -25.2885, lng: 28.7329 },
          { lat: -25.2827, lng: 28.6956 },
          { lat: -25.2945, lng: 28.6792 },
          { lat: -25.303, lng: 28.6413 },
          { lat: -25.3276, lng: 28.6332 },
          { lat: -25.3315, lng: 28.6226 },
          { lat: -25.3337, lng: 28.6389 },
          { lat: -25.394, lng: 28.6198 },
          { lat: -25.4114, lng: 28.6253 },
          { lat: -25.4476, lng: 28.6295 },
          { lat: -25.4594, lng: 28.6208 },
          { lat: -25.4632, lng: 28.6154 },
          { lat: -25.4679, lng: 28.621 },
          { lat: -25.4707, lng: 28.6321 },
          { lat: -25.4884, lng: 28.634 },
          { lat: -25.5199, lng: 28.6257 },
          { lat: -25.5231, lng: 28.6267 },
          { lat: -25.5312, lng: 28.635 },
          { lat: -25.5592, lng: 28.6455 },
          { lat: -25.5858, lng: 28.6473 },
          { lat: -25.5856, lng: 28.6425 },
          { lat: -25.5899, lng: 28.6747 },
          { lat: -25.6588, lng: 28.6727 },
          { lat: -25.665, lng: 28.6838 },
          { lat: -25.6754, lng: 28.7088 },
          { lat: -25.6828, lng: 28.7221 },
          { lat: -25.6442, lng: 28.7239 },
          { lat: -25.6318, lng: 28.7382 },
          { lat: -25.6339, lng: 28.7663 },
          { lat: -25.647, lng: 28.759 },
          { lat: -25.678, lng: 28.7528 },
          { lat: -25.6939, lng: 28.7574 },
          { lat: -25.6953, lng: 28.7572 },
          { lat: -25.7059, lng: 28.7649 },
          { lat: -25.7036, lng: 28.7653 },
          { lat: -25.7006, lng: 28.7681 },
          { lat: -25.7009, lng: 28.7693 },
          { lat: -25.6988, lng: 28.7793 },
          { lat: -25.7019, lng: 28.7927 },
          { lat: -25.7024, lng: 28.8082 },
          { lat: -25.689, lng: 28.8193 },
          { lat: -25.6951, lng: 28.847 },
          { lat: -25.6824, lng: 28.8229 },
          { lat: -25.6589, lng: 28.81 },
          { lat: -25.6591, lng: 28.8115 },
          { lat: -25.643, lng: 28.836 },
          { lat: -25.6374, lng: 28.8355 },
          { lat: -25.6456, lng: 28.8416 },
          { lat: -25.6545, lng: 28.8457 },
          { lat: -25.6523, lng: 28.8693},
          { lat: -25.6581, lng: 28.872},
          { lat: -25.6416, lng: 28.8629},
          { lat: -25.6355, lng: 28.8584},
          { lat: -25.6348, lng: 28.8431},
          { lat: -25.5664, lng: 28.8085},
          { lat: -25.5732, lng: 28.7971},
          { lat: -25.542, lng: 28.8219},
          { lat: -25.5224, lng: 28.8518},
          { lat: -25.5018, lng: 28.8699},
          { lat: -25.5247, lng: 28.8741},
          { lat: -25.5271, lng: 28.8582},
          { lat: -25.5691, lng: 28.9101},
          { lat: -25.5245, lng: 28.9152},
          { lat: -25.558, lng: 28.9465},
          { lat: -25.5998, lng: 29.0078},
          { lat: -25.5742, lng: 28.9869},
          { lat: -25.5402, lng: 28.9881},
          { lat: -25.5395, lng: 28.9932},
          { lat: -25.5425, lng: 29.0067},
          { lat: -25.5376, lng: 29.0106},
          { lat: -25.5332, lng: 29.019},
          { lat: -25.5278, lng: 29.0137},
          { lat: -25.5132, lng: 29.036},
          { lat: -25.5213, lng: 29.0422},
          { lat: -25.521, lng: 29.0474},
          { lat: -25.517, lng: 29.0482},
          { lat: -25.5202, lng: 29.0587},
          { lat: -25.5272, lng: 29.0787},
          { lat: -25.4996, lng: 29.0774},
          { lat: -25.4962, lng: 29.08},
          { lat: -25.4946, lng: 29.0914},
          { lat: -25.5306, lng: 29.0984},
          { lat: -25.5377, lng: 29.0869},
          { lat: -25.5486, lng: 29.0684},
          { lat: -25.5523, lng: 29.0343},
          { lat: -25.5634, lng: 29.042},
          { lat: -25.5733, lng: 29.0456},
          { lat: -25.5808, lng: 29.04},
          { lat: -25.6175, lng: 29.0642},
          { lat: -25.6207, lng: 29.0661},
          { lat: -25.6305, lng: 29.0565},
          { lat: -25.6326, lng: 29.0646},
          { lat: -25.6424, lng: 28.9688},
          { lat: -25.6733, lng: 28.9507},
          { lat: -25.7283, lng: 28.9813},
          { lat: -25.7425, lng: 28.9576},
          { lat: -25.7746, lng: 28.9382},
          { lat: -25.8233, lng: 28.9433},
          { lat: -25.8244, lng: 28.9504},
          { lat: -25.8374, lng: 28.9382},
          { lat: -25.8886, lng: 28.8868},
          { lat: -25.8714, lng: 28.8839},
          { lat: -25.8783, lng: 28.889},
          { lat: -25.8803, lng: 28.8795},
          { lat: -25.9252, lng: 28.8825},
          { lat: -25.9698, lng: 28.8833},
          { lat: -25.9714, lng: 28.8852},
          { lat: -25.9714, lng: 28.885},
          { lat: -25.9744, lng: 28.8673},
          { lat: -26.007, lng: 28.8648},
          { lat: -26.0013, lng: 28.8329},
          { lat: -26.0048, lng: 28.7939},
          { lat: -26.0046, lng: 28.7739},
          { lat: -26.0137, lng: 28.7624},
          { lat: -25.9831, lng: 28.7579},
          { lat: -25.9837, lng: 28.7575},
          { lat: -25.9906, lng: 28.7108},
          { lat: -25.9906, lng: 28.6527},
          { lat: -25.9743, lng: 28.6323},
          { lat: -25.9708, lng: 28.591},
          { lat: -26.0025, lng: 28.5867},
          { lat: -26.016, lng: 28.5789},
          { lat: -26.0596, lng: 28.5386},
          { lat: -26.0478, lng: 28.5308},
          { lat: -26.0616, lng: 28.5158},
          { lat: -26.0585, lng: 28.4687},
          { lat: -26.0781, lng: 28.4405},
          { lat: -26.0477, lng: 28.4422},
          { lat: -26.0422, lng: 28.4491},
          { lat: -26.0427, lng: 28.4547},
          { lat: -26.0308, lng: 28.4601},
          { lat: -26.0237, lng: 28.4643},
          { lat: -26.0256, lng: 28.4644},
          { lat: -26.0114, lng: 28.4457},
          { lat: -26.0181, lng: 28.4482},
          { lat: -25.9999, lng: 28.4511},
          { lat: -25.9955, lng: 28.4444},
          { lat: -25.9911, lng: 28.4517},
          { lat: -25.9864, lng: 28.4627},
          { lat: -25.9744, lng: 28.468},
          { lat: -25.9633, lng: 28.457},
          { lat: -25.9567, lng: 28.446},
          { lat: -25.9649, lng: 28.4177},
          { lat: -25.9505, lng: 28.4167},
          { lat: -25.9484, lng: 28.4195},
          { lat: -25.9487, lng: 28.4176},
          { lat: -25.9394, lng: 28.4072},
          { lat: -25.9241, lng: 28.3524},
          { lat: -25.9546, lng: 28.3381},
          { lat: -25.95, lng: 28.3195},
          { lat: -25.9137, lng: 28.2694},
          { lat: -25.9158, lng: 28.2712},
          { lat: -25.925, lng: 28.2691}
        ];
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
