import { IonicModule } from '@ionic/angular';
import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { TabSavedPage } from './tab-saved.page';
import { ExploreContainerComponentModule } from '../explore-container/explore-container.module';
import { MapModalModule } from '../shared/map-modal/map-modal.module';

import { TabSavedPageRoutingModule } from './tab-saved-routing.module';

@NgModule({
  imports: [
    IonicModule,
    CommonModule,
    FormsModule,
    ExploreContainerComponentModule,
    TabSavedPageRoutingModule,
    MapModalModule
  ],
  declarations: [TabSavedPage]
})
export class TabSavedPageModule {}


// onSearchInput(event: any) {
//   if (event.target.value.length > 0) {
//     this.showResultsList = true;
//     const query = event.target.value;

//     // The bounding box for South Africa
//     const MIN_LONGITUDE = 16.344976;
//     const MIN_LATITUDE = -34.819166;
//     const MAX_LONGITUDE = 32.830120;
//     const MAX_LATITUDE = -22.126612;

//     // Define the bounding box coordinates for South Africa (limit search results to SA only)
//     const bbox = `${MIN_LONGITUDE},${MIN_LATITUDE},${MAX_LONGITUDE},${MAX_LATITUDE}`;

//     // Make a request to Mapbox Geocoding API
//     fetch(`https://api.mapbox.com/geocoding/v5/mapbox.places/${query}.json?proximity=ip&bbox=${bbox}&access_token=${environment.MapboxApiKey}`)
//       .then(response => response.json()) // Parsing the response body as JSON
//       .then(data => {
//         //console.log("DATA " + JSON.stringify(data));
//         this.searchResults = data.features.map((feature: any) => {
//           const place_name = feature.place_name;
//           const firstCommaIndex = place_name.indexOf(',');
//           const trimmedPlaceName = place_name.substring(firstCommaIndex + 2);
//           // return each feature with an updated place_name property that excludes the text property
//           return {
//             ...feature,
//           };
//         });
//         console.log(this.searchResults);
//       })
//       .catch(error => console.error(error));
//   }
// }