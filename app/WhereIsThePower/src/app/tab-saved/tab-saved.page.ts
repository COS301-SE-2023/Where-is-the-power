import { Component, ViewChild } from '@angular/core';
import { UserLocationService } from '../user-location.service';
import { HttpClient } from '@angular/common/http';
import { environment } from 'src/environments/environment';
import { Subscription } from 'rxjs';
import { AuthService } from '../authentication/auth.service';
import { Router } from '@angular/router';
import { SavedPlacesService } from './saved-places.service';
import { Place } from './place';
import { ToastController } from '@ionic/angular';
import { take } from 'rxjs/operators';

@Component({
  selector: 'app-tab-saved',
  templateUrl: 'tab-saved.page.html',
  styleUrls: ['tab-saved.page.scss']
})
export class TabSavedPage {
  latitude: any;
  places: Place[] = [];
  savedPlaces: Place[] = [];
  isLoggedIn: boolean = false;
  showResultsList: boolean = false;
  searchResults: any[] = [];
  queryLength = 0;
  private placesSubscription: Subscription;
  private savePlaceSubscription: Subscription;
  @ViewChild('searchBar', { static: false }) searchBar: any;

  constructor(private router: Router,
    private userLocationService: UserLocationService,
    private http: HttpClient,
    private authService: AuthService,
    private savedPlaceService: SavedPlacesService,
    private toastController: ToastController
  ) {
    this.placesSubscription = new Subscription();
    this.savePlaceSubscription = new Subscription();
  }

  ngOnInit() { }

  gotoProfileRoute() {
    this.router.navigate(['tabs/tab-profile']);
  }

  async ionViewDidEnter() {
    this.isLoggedIn = await this.authService.isUserLoggedIn();

    if (this.isLoggedIn) {
      this.placesSubscription = this.savedPlaceService.getPlaces().subscribe((data: any) => {
        this.places = data.result;
        this.places.sort((a: Place, b: Place) => {
          return a.name.localeCompare(b.name); // Sort alphabetically based on the name property
        });
        console.log("Saved Places: ", this.places);
      });

      this.savePlaceSubscription = this.savedPlaceService.savePlace.subscribe((savePlace: any) => {
        if (savePlace === true) {
          console.log("Save Page savePlace: ", this.savedPlaceService.savedPlace);
          this.router.navigate(['tabs/tab-saved']);
          this.addSavedPlace(this.savedPlaceService.savedPlace);
        }
      });
    }
  }

  ionViewDidLeave() {
    this.isLoggedIn = false;
  }

  goToPlace(result: any) {
    this.savedPlaceService.goToPlace(result);
  }

  goToSavedPlace(result: any) {
    this.savedPlaceService.navigateToSavedPlace.next(true);
    this.savedPlaceService.goToPlace(result);
  }

  savePlace(result: any) {
    this.showResultsList = false;
    console.log("result: ", result);


    // Assign the result to a new object
    let newPlace: Place = {
      "mapboxId": result.id,
      "name": result.text,
      "address": result.place_name,
      "latitude": result.center[1],
      "longitude": result.center[0],
      "category": "average",
      "placeType": "unkown"
    }
    this.goToPlace(newPlace);
  }

  addSavedPlace(place: any) {
    if (!this.isPlaceSaved(place)) {
      this.savedPlaceService.addSavedPlace(place)
        .pipe(take(1)) //subscription will automatically unsubscribe after the first emission
        .subscribe(data => {
          console.log("addSavedPlace: ", data);

          if (this.places.length > 0) {
            this.savedPlaceService.place.next([...this.places, place]);
          } else {
            this.savedPlaceService.place.next([place]);
          }
          // this.sucessToast('Succesfully added place');
        },
          error => {
            console.error("addSavedPlace error: ", error);
          }
        );
    }
  }

  removeSavedPlace(place: any) {
    this.savedPlaces = this.savedPlaces.filter((sPlace: any) => {
      if (sPlace.id !== place.id) return sPlace;
    });
    console.log("removeSavedPlace: ", this.savedPlaces);
  }

  isPlaceSaved(place: any) {
    let isSaved = false;
    this.savedPlaces.forEach((sPlace: any) => {
      if (sPlace.id === place.id) isSaved = true;
    });
    //console.log(isSaved)
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

  onBlur() {
    console.log("Search Bar Blurred");
    setTimeout(() => {
      this.showResultsList = false;
    }, 200); // 200ms delay
  }

  async sucessToast(message: string) {
    const toast = await this.toastController.create({
      message: message,
      color: 'success',
      duration: 500,
      position: 'bottom',
    });
    toast.present();
  }

  // TODO send Boolean to mapmodal
  ngOnDestroy() {
    if (this.placesSubscription) {
      this.placesSubscription.unsubscribe(); // unsubscribe from the observable
    }
  }
}
