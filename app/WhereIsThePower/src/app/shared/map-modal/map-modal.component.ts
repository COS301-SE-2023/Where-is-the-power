import {
  Component,
  OnInit,
  AfterViewInit,
  ViewChild,
  ChangeDetectorRef,
  HostListener
} from '@angular/core';
import { environment } from 'src/environments/environment';
import { UserLocationService } from '../../user-location.service';
import { IonContent, ModalController } from '@ionic/angular';

//import * as mapboxgl from 'mapbox-gl';
//import * as MapboxGeocoder from '@mapbox/mapbox-gl-geocoder';
import { MapSuburbsService } from './map-suburbs.service';
import { EventEmitter, Output } from '@angular/core';
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

  constructor(
    private mapSuburbsService: MapSuburbsService,
    private userLocationService: UserLocationService,
    private modalCtrl: ModalController,
    private changeDetectorRef: ChangeDetectorRef
  ) { }
  map: any;
  dat: any;
  searchResults: any[] = [];
  start = [];
  latitude: any;
  longitude: any;
  showResultsList: boolean = false;
  instructions: string[] = [];
  tripDuration: number = 0;
  tripDistance: number = 0;
  startTrip: boolean = false; // Only displayed when "Begin trip" button is clicked
  gettingRoute: boolean = false;
  mapLoaded: boolean = false; // Check if map rendered
  public currentBreakpoint = 0.2;
  screenWidth: number = 0;
  screenHeight: number = 0;

  ngOnInit() { }

  ngAfterViewInit() {
    this.mapSuburbsService.getSuburbData().subscribe(async (data: any) => {
      console.log(data.result.mapPolygons[0]);
      this.dat = data.result.mapPolygons[0];
      // Render the Map
      (mapboxgl as any).accessToken = environment.MapboxApiKey;
      this.map = await new mapboxgl.Map({
        container: 'map', // container ID
        style: 'mapbox://styles/mapbox/streets-v12', // style URL
        center: [28.2, -25.754995], // starting position [lng, lat]
        zoom: 11 // starting zoom
      });
      this.mapLoaded = true; // map has finished rendering

      // get user location
      this.latitude = this.userLocationService.getLatitude();
      this.longitude = this.userLocationService.getLongitude();

      this.map.on('load', () => {
        this.map.resize(); // Trigger map resize after the initial rendering
      });

      // Populate Map(suburbs) with Polygons
      this.populatePolygons();

    },
      (error: any) => {
        console.log(error);
      }
    );

  }

  populatePolygons() {
    this.map.on('load', () => {
      // Add a data source containing GeoJSON data.
      this.map.addSource('polygons', {
        'type': 'geojson',
        'data': this.dat
      });
      // console.log('./suburbs.geojson');
      // Add a new layer to visualize the polygon.
      this.map.addLayer({
        'id': 'polygons-layer',
        'type': 'fill',
        'source': 'polygons', // reference the data source
        'layout': {},
        'paint': {
          'fill-color': [
            'match',
            ['get', 'PowerStatus'], // Property to evaluate
            'on', '#12960e',       // Fill color when 'Powerstatus' is 'on'
            'off', '#ff0000',      // Fill color when 'Powerstatus' is 'off'
            /* Add more cases if needed */
            '#cccccc'              // Default fill color when 'Powerstatus' doesn't match any case
          ],
          'fill-opacity': 0.5
        }
      });

      this.map.addLayer({
        'id': 'lines-layer',
        'type': 'line',
        'source': 'polygons', // reference the data source containing line features
        'layout': {
          'line-join': 'round',
          'line-cap': 'round'
        },
        'paint': {
          'line-color': '#1c470c', // Set the color of the lines
          'line-width': 0.5 // Set the width of the lines in pixels
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

  onSearchBarFocus() {
    // Show the list when the search bar gets focused on
    if (this.searchBar.value.length > 0)
      this.showResultsList = true;
  }

  onSearchInput(event: any) {
    if (event.target.value.length > 0) {
      this.showResultsList = true;
      const query = event.target.value;

      // The bounding box for South Africa
      const MIN_LONGITUDE = 16.344976;
      const MIN_LATITUDE = -34.819166;
      const MAX_LONGITUDE = 32.830120;
      const MAX_LATITUDE = -22.126612;

      // Define the bounding box coordinates for South Africa (limit search results to SA only)
      const bbox = `${MIN_LONGITUDE},${MIN_LATITUDE},${MAX_LONGITUDE},${MAX_LATITUDE}`;

      // Make a request to Mapbox Geocoding API
      fetch(`https://api.mapbox.com/geocoding/v5/mapbox.places/${query}.json?proximity=ip&bbox=${bbox}&access_token=${environment.MapboxApiKey}`)
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
            };
          });
          //console.log("searchResults: ",this.searchResults);
        })
        .catch(error => console.error(error));
    }
  }


  async getRoute(selectedResult: any) {
    this.updateBreakpoint();

    this.emitGetDirections();
    this.gettingRoute = true;
    this.searchBar.value = `${selectedResult.place_name}`;

    this.showResultsList = false;
    let coords: any;
    console.log(selectedResult);

    let query: any;
    if (Array.isArray(selectedResult)) {
      query = await fetch(`https://api.mapbox.com/directions/v5/mapbox/driving/${this.longitude},${this.latitude};${selectedResult[0]},${selectedResult[1]}?alternatives=true&geometries=geojson&language=en&overview=full&steps=true&access_token=${environment.MapboxApiKey}`)
    }
    else {
      query = await fetch(`https://api.mapbox.com/directions/v5/mapbox/driving/${this.longitude},${this.latitude};${selectedResult.center[0]},${selectedResult.center[1]}?alternatives=true&geometries=geojson&language=en&steps=true&access_token=${environment.MapboxApiKey}`)
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
          'circle-radius': 12,
          'circle-color': '#1a9107' // Green color for the start point
        }
      });
      // Center the map on the start point (user's current location)
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
          'circle-radius': 12,
          'circle-color': '#f30'
        }
      });
    }

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
    // get the sidebar and add the instructions
    const steps = data.legs[0].steps;
    for (const step of steps) {
      this.instructions.push(step.maneuver.instruction);
    }

    this.tripDuration = Math.floor(data.duration / 60);
    this.tripDistance = Math.floor(data.distance / 1000);

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
          'line-opacity': 1
        }
      });
    }

    // Calculate the bounding box OF THE ROUTE
    let minLng = Infinity;
    let maxLng = -Infinity;
    let minLat = Infinity;
    let maxLat = -Infinity;

    for (const coord of route) {
      minLng = Math.min(minLng, coord[0]);
      maxLng = Math.max(maxLng, coord[0]);
      minLat = Math.min(minLat, coord[1]);
      maxLat = Math.max(maxLat, coord[1]);
    }

    const boundingBox = [
      [minLng, minLat],
      [maxLng, maxLat]
    ];

    this.map.fitBounds(boundingBox, {
      padding: 100, // Adjust padding as needed
      maxZoom: 12 // Adjust the maximum zoom level as needed
    });
  }

  delay(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
  }

  onSearchBarClear() {
    this.myModal.dismiss();
    this.startTrip = false;
    this.showResultsList = false;
    this.tripDuration = 0;
    this.tripDistance = 0;
    this.gettingRoute = false;
    this.emitCancelDirections();
    this.updateBreakpoint();

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

  centerOnStartPoint() {
    this.map.flyTo({
      center: [this.longitude, this.latitude], // Center on user position
      zoom: 15, // Adjust the zoom level
      speed: 1.2, // Adjust the speed of the animation
    });
  }
  @ViewChild('myModal') myModal: any; // Reference to the ion-modal element
  modalResult: any; // To store the selected result data

  openModal(result: any) {
    // if (!this.myModal) {
    this.modalResult = result;
    this.myModal.present();
    //  }
  }

  getIconForInstruction(instruction: string) {
    // Regular expressions to match keywords related to arrows
    const arrowKeywords = [
      { keyword: /(north|toward|straight|south|continue)/i, icon: 'assets/arrow_upwards.svg' },
      { keyword: /(west|left)/i, icon: 'assets/turn_left.svg' },
      { keyword: /(east|right)/i, icon: 'assets/turn_right.svg' },
      { keyword: /(back| u-turn)/i, icon: 'assets/u_turn.svg' },
      { keyword: /(roundabout)/i, icon: 'assets/roundabout.svg' },
      { keyword: /(exit | ramp)/i, icon: 'assets/exit.svg' }
    ];

    // Search for arrow keywords in the instruction text
    for (const arrow of arrowKeywords) {
      if (arrow.keyword.test(instruction)) {
        return arrow.icon;
      }
    }

    // If no arrow keyword is found, return a default icon
    return 'information-circle-outline';
  }

  beginTrip() {
    this.startTrip = true;
    this.centerOnStartPoint();
    this.updateBreakpoint();


    if (this.userMarker) {
      this.userMarker.remove();
    }
  }

  userMarker: any;

  pin() {
    this.centerOnStartPoint();

    // Check if the userMarker already exists
    if (this.userMarker) {
      this.userMarker.remove();
    }

    // Create a new marker at the user's location
    this.userMarker = new mapboxgl.Marker({ color: '#4287f5' }) // Customize the pin color if desired
      .setLngLat([this.longitude, this.latitude]) // Set the marker's position to the user's location
      .addTo(this.map); // Add the marker to the map
  }

  onModalDismiss() {
    this.onSearchBarClear();
  }

  emitCancelDirections() {
    this.mapSuburbsService.gettingDirections.next(false);
    this.map.resize();
  }

  async emitGetDirections() {
    this.mapSuburbsService.gettingDirections.next(true);
    await this.delay(500);
    this.map.resize();
  }

  @HostListener('window:resize')
  onResize() {
    this.updateBreakpoint();
  }

  updateBreakpoint() {
    this.screenWidth = window.innerWidth;
    this.screenHeight = window.innerHeight;
    const isIphone = /iPhone/i.test(navigator.userAgent); // iphone screen sizing is different

    if (this.startTrip == false) {
      if (this.screenHeight > 840 && !isIphone) {
        this.currentBreakpoint = 0.2;
      }
      else if ((this.screenHeight > 770 && this.screenHeight <= 840) || isIphone && this.screenHeight > 770) {
        this.currentBreakpoint = 0.22;
      }
      else if (this.screenHeight > 735 && this.screenHeight <= 870 && !isIphone) {
        this.currentBreakpoint = 0.23;
      }
      else if (this.screenHeight > 700 && this.screenHeight <= 770 && !isIphone) {
        this.currentBreakpoint = 0.24;
      }
      else if (this.screenHeight <= 700 || isIphone) {
        this.currentBreakpoint = 0.28;
      }
    }
    else {
      if (this.screenHeight > 800) {
        this.currentBreakpoint = 0.1;
      }
      else if (this.screenHeight > 700 && this.screenHeight <= 800) {
        this.currentBreakpoint = 0.12;
      }
      else if (this.screenHeight <= 700) {
        this.currentBreakpoint = 0.14;
      }
    }
    if (this.myModal) // Check if myModal is defined before calling setCurrentBreakpoint
      this.myModal.setCurrentBreakpoint(this.currentBreakpoint);
  }
}

