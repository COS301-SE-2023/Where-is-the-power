import {
  Component,
  OnInit,
  AfterViewInit,
} from '@angular/core';
import { environment } from 'src/environments/environment';
import * as mapboxgl from 'mapbox-gl';
@Component({
  selector: 'app-map-modal',
  templateUrl: './map-modal.component.html',
  styleUrls: ['./map-modal.component.scss'],
})
export class MapModalComponent implements OnInit, AfterViewInit {
  constructor(
  ) { }

  map: any;
  ngOnInit() {
  }

  ngAfterViewInit() {
    (mapboxgl as any).accessToken = environment.MapboxApiKey;
    this.map = new mapboxgl.Map({
      container: 'map', // container ID
      style: 'mapbox://styles/mapbox/streets-v12', // style URL
      center: [28.261181, -25.771179], // starting position [lng, lat]
      zoom: 12 // starting zoom
    });

    this.map.on('load', () => {
      this.map.resize(); // Trigger map resize after the initial rendering
    });
  }
}
