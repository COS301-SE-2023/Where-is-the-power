import { Component } from '@angular/core';
import { UserLocationService } from '../user-location.service';
import { HttpClient } from '@angular/common/http';
import { environment } from 'src/environments/environment';
import { FeatureTypes } from './feature-types';
import { empty } from 'rxjs';

@Component({
  selector: 'app-tab-saved',
  templateUrl: 'tab-saved.page.html',
  styleUrls: ['tab-saved.page.scss']
})
export class TabSavedPage {
  latitude: any;
  places: any[] = [];
  featureTypesEnum = FeatureTypes;
  savedPlaces: any[] = [];

  constructor(private userLocationService: UserLocationService, private http: HttpClient) {}

  ngOnInit() {
    this.userLocationService.getUserLocation();
  }

  ionViewDidEnter() {
    this.latitude = this.userLocationService.getLatitude();
  }

  input: string | undefined;

  updateResults() {
    if(this.input !== '') {
      this.http.get('https://api.mapbox.com/search/searchbox/v1/suggest?q='+this.input+'&access_token='+environment.MapboxApiKey+'&session_token&country=za&origin=25,-25').subscribe((data: any) => {
        console.log(data);
        this.places = [];
        
        data.suggestions.forEach((searchResult: any)  => {
          let obj = {
            type: this.getFeatureType(searchResult.feature_type), 
            name: searchResult.name, 
            feature: searchResult.feature_type,
            address: searchResult.full_address,
            id: searchResult.mapbox_id
          };
          this.places.push(obj);
        });
      })
    } else {
      this.places = [];
    }
  }

  addSavedPlace(place: any) {
    this.savedPlaces.push(place);
    console.log(this.savedPlaces);
  }

  removeSavedPlace(place: any) {
    this.savedPlaces = this.savedPlaces.filter((sPlace: any) => {
      if(sPlace.id !== place.id) return sPlace; 
    });
    console.log(this.savedPlaces);
  }

  isPlaceSaved(place: any) {
    return !this.savedPlaces.includes(place);
  }

  getFeatureType(featureType: string) {
    switch(featureType) {
      case 'country': 
        return this.featureTypesEnum.Country;
      case 'region': 
        return this.featureTypesEnum.Region;
      case 'postcode': 
        return this.featureTypesEnum.Postcode;
      case 'district': 
        return this.featureTypesEnum.District;
      case 'place': 
        return this.featureTypesEnum.Place;
      case 'city': 
        return this.featureTypesEnum.City;
      case 'locality': 
        return this.featureTypesEnum.Locality;
      case 'neighbourhood': 
        return this.featureTypesEnum.Neighbourhood;
      case 'street': 
        return this.featureTypesEnum.Street;
      case 'address': 
        return this.featureTypesEnum.Address;    
      default:
        return this.featureTypesEnum.Default;
    }
  }
  
}
