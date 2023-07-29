import {
  Component,
  OnInit,
  AfterViewInit,
  ViewChild,
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
  @ViewChild('searchBar', { static: false }) searchBar: any;
  constructor(private mapSuburbsService: MapSuburbsService, private userLocationService: UserLocationService) { }
  map: any;
  dat: any;
  searchResults: any[] = [];
  start = [];
  latitude: any;
  longitude: any;
  showResultsList: boolean = false;

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
    // Render the Map
    (mapboxgl as any).accessToken = environment.MapboxApiKey;
    this.map = new mapboxgl.Map({
      container: 'map', // container ID
      style: 'mapbox://styles/mapbox/streets-v12', // style URL
      center: [28.261181, -25.771179], // starting position [lng, lat]
      zoom: 12 // starting zoom
    });

    // get user location
    this.latitude = this.userLocationService.getLatitude();
    this.longitude = this.userLocationService.getLongitude();

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
    this.showResultsList = true;
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


  async getRoute(selectedResult: any) {
    console.log(this.searchBar);
    this.searchBar.value = `${selectedResult.text}, ${selectedResult.place_name}`;

    this.showResultsList = false;
    let coords: any;

    // console.log(selectedResult);
    console.log(selectedResult);
    console.log(this.longitude);
    console.log(this.latitude);
    let query: any;
    if (Array.isArray(selectedResult)) {
      query = await fetch(`https://api.mapbox.com/directions/v5/mapbox/driving/${this.longitude},${this.latitude};${selectedResult[0]},${selectedResult[1]}?alternatives=true&geometries=geojson&language=en&overview=full&steps=true&access_token=${environment.MapboxApiKey}`)
    }
    else {
      query = await fetch(`https://api.mapbox.com/directions/v5/mapbox/driving/${this.longitude},${this.latitude};${selectedResult.center[0]},${selectedResult.center[1]}?alternatives=true&geometries=geojson&language=en&overview=full&steps=true&access_token=${environment.MapboxApiKey}`)
      coords = [selectedResult.center[0], selectedResult.center[1]];
    }
    console.log(coords);
    // Add a marker for the start point

    const start = {
      type: 'FeatureCollection',
      features: [
        {
          type: 'Feature',
          properties: {},
          geometry: {
            type: 'Point',
            coordinates: [this.longitude, this.latitude]
          }
        }
      ]
    };
    if (this.map.getLayer('start')) {
      this.map.getSource('start').setData(start);
    } else {
      this.map.addLayer({
        id: 'start',
        type: 'circle',
        source: {
          type: 'geojson',
          data: {
            type: 'FeatureCollection',
            features: [
              {
                type: 'Feature',
                properties: {},
                geometry: {
                  type: 'Point',
                  coordinates: [this.longitude, this.latitude]
                }
              }
            ]
          }
        },
        paint: {
          'circle-radius': 10,
          'circle-color': '#1a9107' // Green color for the start point
        }
      });
    }

    const end = {
      type: 'FeatureCollection',
      features: [
        {
          type: 'Feature',
          properties: {},
          geometry: {
            type: 'Point',
            coordinates: coords
          }
        }
      ]
    };
    if (this.map.getLayer('end')) {
      this.map.getSource('end').setData(end);
    } else {
      this.map.addLayer({
        id: 'end',
        type: 'circle',
        source: {
          type: 'geojson',
          data: {
            type: 'FeatureCollection',
            features: [
              {
                type: 'Feature',
                properties: {},
                geometry: {
                  type: 'Point',
                  coordinates: coords
                }
              }
            ]
          }
        },
        paint: {
          'circle-radius': 10,
          'circle-color': '#f30'
        }
      });
    }

    console.log("ROUTE" + JSON.stringify(query));

    const json = await query.json();

    const data = json.routes[0]; // Pick 1st route in list of route recommendations
    const route = data.geometry.coordinates; // list of coordinates forming route

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
          'line-width': 10,
          'line-opacity': 0.75
        }
      });
    }
    // add turn instructions here at the end
  }

  onSearchBarClear() {
    if (this.map.getSource('route')) {
      this.map.removeLayer('route');
      this.map.removeSource('route');
    }

    // Remove the start point marker layer (if it exists)
    if (this.map.getLayer('start')) {
      this.map.removeLayer('start');
      this.map.removeSource('start');
    }

    // Remove the end point marker layer (if it exists)
    if (this.map.getLayer('end')) {
      this.map.removeLayer('end');
      this.map.removeSource('end');
    }
  }
}

