import { Component, ViewChild } from '@angular/core';
import { UserLocationService } from '../user-location.service';
import { HttpClient } from '@angular/common/http';
import { environment } from 'src/environments/environment';
import { empty } from 'rxjs';
import { AuthService } from '../authentication/auth.service';
import { Router } from '@angular/router';
import { SavedPlacesService } from './saved-places.service';

@Component({
  selector: 'app-tab-saved',
  templateUrl: 'tab-saved.page.html',
  styleUrls: ['tab-saved.page.scss']
})
export class TabSavedPage {
  latitude: any;
  places: any[] = [];
  savedPlaces: any[] = [];
  isLoggedIn: boolean = false;
  showResultsList: boolean = false;
  searchResults: any[] = [];
  queryLength = 0;

  @ViewChild('searchBar', { static: false }) searchBar: any;

  constructor(private router: Router,
    private userLocationService: UserLocationService,
    private http: HttpClient,
    private authService: AuthService,
    private savedPlaceService: SavedPlacesService) { }

  ngOnInit() {
    this.userLocationService.getUserLocation();
  }

  gotoProfileRoute() {
    this.router.navigate(['tabs/tab-profile']);
  }

  async ionViewDidEnter() {
    this.latitude = this.userLocationService.getLatitude();
    this.isLoggedIn = await this.authService.isUserLoggedIn();
    console.log(this.isLoggedIn)
  }

  ionViewDidLeave() {
    this.isLoggedIn = false;
  }

  input: string | undefined;

  updateResults() {
    if (this.input !== '') {
      this.http.get('https://api.mapbox.com/search/searchbox/v1/suggest?q=' + this.input + '&access_token=' + environment.MapboxApiKey + '&session_token&country=za&origin=25,-25').subscribe((data: any) => {
        console.log(data);
        this.places = [];

        data.suggestions.forEach((searchResult: any) => {
          let obj = {
            type: this.getFeatureType(searchResult.feature_type),
            name: searchResult.name,
            feature: searchResult.feature_type,
            address: searchResult.full_address,
            id: searchResult.mapbox_id
          };
          this.places.push(obj);

          console.log(this.places);
        });
      })
    } else {
      this.places = [];
    }
  }

  addSavedPlace(place: any) {
    this.savedPlaces.push(place);
    this.places = this.places.filter((sPlace: any) => {
      if (sPlace.id !== place.id) return sPlace;
    });
    console.log(this.savedPlaces);
    this.authService.addSavedPlace().subscribe((data: any) => {
      console.log(data);
    });
  }

  removeSavedPlace(place: any) {
    this.savedPlaces = this.savedPlaces.filter((sPlace: any) => {
      if (sPlace.id !== place.id) return sPlace;
    });
    console.log(this.savedPlaces);
  }

  isPlaceSaved(place: any) {
    let isSaved = false;
    this.savedPlaces.forEach((sPlace: any) => {
      if (sPlace.id === place.id) isSaved = true;
    });
    console.log(isSaved)
    return isSaved;
  }

  isAddress(feature: string) {
    if (feature === 'country' ||
      feature === 'region' ||
      feature === 'postcode' ||
      feature === 'district' ||
      feature === 'place' ||
      feature === 'locality' ||
      feature === 'neighbourhood' ||
      feature === 'address') return true;
    return false;
  }

  c() {
    console.log()
  }


//   export enum FeatureTypes {
//     Country = "globe-outline",
//     Region = "map-outline",
//     Postcode = "map-outline", 
//     District = "map-outline",
//     Place = "business-outline",
//     Neighbourhood = "location-outline",
//     Locality = "location-outline",
//     Address = "home-outline",
//     City = "business-outline",
//     Street = "car-outline",
//     Default = "ellipse-outline"
// }

  getFeatureType(instruction: string) {
    // Regular expressions to match keywords related to arrows
    const featureKeywords = [
      { keyword: /(Country)/i, icon: 'globe-outline' },
      { keyword: /(Region|District)/i, icon: 'map-outline' },
      { keyword: /(Place|City)/i, icon: 'business-outline' },
      { keyword: /(Neighbourhood|Locality|Postcode)/i, icon: 'location-outline' },
      { keyword: /(Address)/i, icon: 'home-outline' },
      { keyword: /(Street)/i, icon: 'car-outline' }
    ];

    // Search for arrow keywords in the instruction text
    for (const feature of featureKeywords) {
      if (feature.keyword.test(instruction)) {
        return feature.icon;
      }
    }

    // If no arrow keyword is found, return a default icon
    return 'ellipse-outline';
  }

  onSearchInput(event: any) {
    if (event.target.value.length > 0) {
      this.showResultsList = true;
      const query = event.target.value;
      this.queryLength = query.length;
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
          console.log(this.searchResults);
        })
        .catch(error => console.error(error));
    }
  }

  onSearchBarFocus() {
    // Show the list when the search bar gets focused on
    if (this.searchBar.value.length > 0)
      this.showResultsList = true;
  }

  onSearchBarClear() {
    this.showResultsList = false;
  }


  // TODO send Boolean to mapmodal

}
