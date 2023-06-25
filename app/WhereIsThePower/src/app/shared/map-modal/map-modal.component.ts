import {
  Component,
  OnInit,
  AfterViewInit,
} from '@angular/core';
import { environment } from 'src/environments/environment';
import * as mapboxgl from 'mapbox-gl';
import * as MapboxGeocoder from '@mapbox/mapbox-gl-geocoder';
@Component({
  selector: 'app-map-modal',
  templateUrl: './map-modal.component.html',
  styleUrls: ['./map-modal.component.scss'],
})
export class MapModalComponent implements OnInit, AfterViewInit {
  constructor() { }

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

    const geocoder = new MapboxGeocoder({
      // Initialize the geocoder
      accessToken: environment.MapboxApiKey, // Set the access token
      mapboxgl: mapboxgl, // Set the mapbox-gl instance
      marker: false, // Do not use the default marker style
      placeholder: 'Search for places', // Placeholder text for the search bar
    });

    // Add the geocoder to the map
    this.map.addControl(geocoder);

    // After the map style has loaded on the page,
    // add a source layer and default styling for a single point
    this.map.on('load', () => {
      this.map.addSource('single-point', {
        'type': 'geojson',
        'data': {
          'type': 'FeatureCollection',
          'features': []
        }
      });

      this.map.addLayer({
        'id': 'point',
        'source': 'single-point',
        'type': 'circle',
        'paint': {
          'circle-radius': 10,
          'circle-color': '#448ee4'
        }
      });

      // Listen for the `result` event from the Geocoder // `result` event is triggered when a user makes a selection
      //  Add a marker at the result's coordinates
      geocoder.on('result', (event: any) => {
        this.map.getSource('single-point').setData(event.result.geometry);
      });
    });

  }
}

