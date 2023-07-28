import {
  Component,
  OnInit,
  AfterViewInit,
} from '@angular/core';
import { environment } from 'src/environments/environment';
import { UserLocationService } from '../../user-location.service';

//import * as mapboxgl from 'mapbox-gl';
//import * as MapboxGeocoder from '@mapbox/mapbox-gl-geocoder';
import { MapSuburbsService } from './map-suburbs.service';
declare let MapboxDirections: any;
declare let mapboxgl: any;
declare let MapboxGeocoder: any;

@Component({
  selector: 'app-map-modal',
  templateUrl: './map-modal.component.html',
  styleUrls: ['./map-modal.component.scss'],
})
export class MapModalComponent implements OnInit, AfterViewInit {
  constructor(private mapSuburbsService: MapSuburbsService, private userLocationService: UserLocationService) { }
  map: any;
  dat: any;
  searchResults: any[] = [];
  start = [];
  latitude: any;
  longitude: any;

  ngOnInit() {
    this.mapSuburbsService.getSuburbData().subscribe((data: any) => {
      console.log(data);
      this.dat = data.mapPolygons;
      //  console.log(this.dat);

    },
      (error: any) => {
        console.log(error);
      }
    );
  }

  async ngAfterViewInit() {
    // get user location
    await this.userLocationService.getUserLocation();
    this.latitude = this.userLocationService.getLatitude();
    this.longitude = this.userLocationService.getLongitude();

    // Render the Map
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

    // Populate Map(suburbs) with Polygons
    this.populatePolygons();

  }

  populatePolygons() {
    this.map.on('load', () => {
      // Add a data source containing GeoJSON data.
      this.map.addSource('polygons', {
        'type': 'geojson',
        'data': 'assets/suburbs.json'
      });
      // console.log('./suburbs.geojson');
      // Add a new layer to visualize the polygon.
      this.map.addLayer({
        'id': 'polygons-layer',
        'type': 'fill',
        'source': 'polygons', // reference the data source
        'layout': {},
        'paint': {
          'fill-color': '#12960e', // blue color fill
          'fill-opacity': 0.4
        }
      });

      // Add a black outline around the polygon.
      this.map.addLayer({
        'id': 'outline',
        'type': 'line',
        'source': 'polygons',
        'layout': {},
        'paint': {
          'line-color': '#1c470c',
          'line-width': 0.5
        }
      });

      // Listen for the click event on the map
      this.map.on('click', 'polygons-layer', (e: any) => {
        const clickedFeature = e.features[0];
        //console.log(e);

        if (clickedFeature) {
          // Handle the click event here, for example, you can log the properties of the clicked feature
          console.log(clickedFeature.properties);
        }
      });
    });
    this.mapSuburbsService.getSuburbData();
  }

  onSearchInput(event: any) {
    const query = event.target.value;

    // Make a request to Mapbox Geocoding API
    fetch(`https://api.mapbox.com/geocoding/v5/mapbox.places/${query}.json?proximity=ip&access_token=${environment.MapboxApiKey}`)
      .then(response => response.json()) // Parsing the response body as JSON
      .then(data => {
        //console.log("DATA " + JSON.stringify(data));
        this.searchResults = data.features.map((feature: any) => {
          const place_name = feature.place_name;
          const firstCommaIndex = place_name.indexOf(',');
          const trimmedPlaceName = place_name.substring(firstCommaIndex + 2);

          // return each feature with an updated place_name property that excludes the text property
          return {
            ...feature,
            place_name: trimmedPlaceName,
          };
        });

        console.log(this.searchResults);
      })
      .catch(error => console.error(error));
  }


  async selectResult(selectedResult: any) {
    // console.log(selectedResult);
    console.log(selectedResult);
    console.log(this.longitude);
    console.log(this.latitude);

    const query = await fetch(`https://api.mapbox.com/directions/v5/mapbox/driving/${this.longitude},${this.latitude};${selectedResult.center[0]},${selectedResult.center[1]}?alternatives=true&geometries=geojson&language=en&overview=full&steps=true&access_token=${environment.MapboxApiKey}`)
    console.log("ROUTE" + JSON.stringify(query));

    const json = await query.json();

    const data = json.routes[0];
    const route = data.geometry.coordinates;
    const geojson = {
      type: 'Feature',
      properties: {},
      geometry: {
        type: 'LineString',
        coordinates: route
      }
    };
    // if the route already exists on the map, we'll reset it using setData
    if (this.map.getSource('route')) {
      this.map.getSource('route').setData(geojson);
    }
    // otherwise, we'll make a new request
    else {
      this.map.addLayer({
        id: 'route',
        type: 'line',
        source: {
          type: 'geojson',
          data: geojson
        },
        layout: {
          'line-join': 'round',
          'line-cap': 'round'
        },
        paint: {
          'line-color': '#3887be',
          'line-width': 5,
          'line-opacity': 0.75
        }
      });
    }
    // add turn instructions here at the end
  }
}

