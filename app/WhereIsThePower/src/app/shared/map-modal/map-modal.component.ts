import {
  Component,
  OnInit,
  AfterViewInit,
} from '@angular/core';
import { environment } from 'src/environments/environment';
//import * as mapboxgl from 'mapbox-gl';
//import * as MapboxGeocoder from '@mapbox/mapbox-gl-geocoder';
declare let MapboxDirections: any;
declare let mapboxgl: any;

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

    /*
    const geocoder = new MapboxGeocoder({
      // Initialize the geocoder
      accessToken: environment.MapboxApiKey, // Set the access token
      mapboxgl: mapboxgl, // Set the mapbox-gl instance
      marker: false, // Do not use the default marker style
      placeholder: 'Search for places', // Placeholder text for the search bar

    });
    Add the geocoder to the map
    this.map.addControl(geocoder);
    */

    // Add the Navigation Control
    let exclusionArea: string = 'point(28.278153 -25.781812),point(28.277781 -25.78166),point(28.276252 -25.781039),point(28.274805 -25.780169),point(28.271878 -25.778368),point(28.271868 -25.778362),point(28.271357 -25.780567),point(28.272005 -25.780674),point(28.272028 -25.780909),point(28.272131 -25.781988),point(28.27693 -25.78533),point(28.28062 -25.78286),point(28.27941 -25.78539),point(28.28524 -25.78414)';
    this.navigate(exclusionArea);

    // Populate Map(suburbs) with Polygons
    this.populatePolygons();

  }

  navigate(exclusionArea: string) {
    this.map.addControl(
      new MapboxDirections({
        accessToken: mapboxgl.accessToken,
        unit: 'metric',
        exclude: ['motorway', exclusionArea]
      }),
      'top-left'
    );
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

      //Loadshedding Area Mock data
      // Add a data source containing GeoJSON data.
      this.map.addSource('menlyn', {
        'type': 'geojson',
        'data': {
          'type': 'Feature',
          'geometry': {
            'type': 'Polygon',
            // These coordinates outline Maine.
            'coordinates': [[[28.278153, -25.781812], [28.277781, -25.78166], [28.276252, -25.781039], [28.274805, -25.780169], [28.271878, -25.778368], [28.271868, -25.778362], [28.271357, -25.780567], [28.272005, -25.780674], [28.272028, -25.780909], [28.272131, -25.781988], [28.273088, -25.781879], [28.273176, -25.781889], [28.273249, -25.78257], [28.273317, -25.783255], [28.273336, -25.783449], [28.273329, -25.783493], [28.273279, -25.783745], [28.273191, -25.783733], [28.272402, -25.783829], [28.271852, -25.783882], [28.271883, -25.78414], [28.27194, -25.784489], [28.272039, -25.784601], [28.272161, -25.784687], [28.272318, -25.784763], [28.272524, -25.784863], [28.272837, -25.785091], [28.273073, -25.785357], [28.2732, -25.785413], [28.273372, -25.785439], [28.273481, -25.78545], [28.273512, -25.785341], [28.273574, -25.785166], [28.273687, -25.785057], [28.274178, -25.785052], [28.275, -25.784969], [28.275408, -25.784928], [28.275547, -25.784918], [28.275645, -25.784943], [28.275723, -25.784995], [28.275785, -25.785114], [28.275826, -25.785512], [28.275909, -25.785775], [28.276022, -25.785927], [28.276106, -25.785997], [28.276567, -25.786251], [28.276583, -25.786213], [28.276625, -25.786112], [28.276952, -25.785404], [28.27755, -25.785558], [28.278122, -25.785708], [28.278668, -25.785849], [28.27914, -25.785995], [28.279154, -25.785954], [28.279211, -25.78577], [28.279303, -25.785538], [28.279341, -25.785498], [28.279463, -25.785446], [28.279875, -25.785362], [28.279989, -25.785341], [28.280085, -25.785337], [28.28016, -25.785344], [28.280172, -25.785345], [28.280935, -25.78546], [28.281786, -25.785597], [28.281936, -25.785633], [28.282018, -25.785703], [28.282127, -25.785863], [28.282235, -25.785998], [28.28237, -25.786076], [28.283506, -25.786275], [28.284163, -25.786432], [28.28422, -25.78632], [28.284407, -25.785946], [28.284578, -25.785595], [28.284754, -25.785238], [28.284895, -25.784956], [28.284992, -25.784781], [28.284995, -25.784777], [28.2852, -25.784409], [28.285267, -25.784292], [28.285528, -25.783838], [28.285673, -25.783466], [28.284074, -25.783327], [28.283035, -25.78316], [28.282951, -25.783146], [28.281956, -25.782987], [28.281678, -25.782916], [28.280621, -25.782647], [28.280194, -25.782538], [28.278153, -25.781812]]]
          }
        }
      });

      // Add a new layer to visualize the polygon.
      this.map.addLayer({
        'id': 'menlyn',
        'type': 'fill',
        'source': 'menlyn', // reference the data source
        'layout': {},
        'paint': {
          'fill-color': '#eb3434', // red color fill
          'fill-opacity': 0.4
        }
      });
      // Add a black outline around the polygon.
      this.map.addLayer({
        'id': 'outlineLoadshedding',
        'type': 'line',
        'source': 'menlyn',
        'layout': {},
        'paint': {
          'line-color': '#8a1616',
          'line-width': 0.5
        }
      });
    });
  }
}

