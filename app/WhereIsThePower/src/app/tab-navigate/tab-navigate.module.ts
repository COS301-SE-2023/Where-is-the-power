import { IonicModule } from '@ionic/angular';
import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { TabNavigatePage } from './tab-navigate.page';
import { ExploreContainerComponentModule } from '../explore-container/explore-container.module';
import { TabNavigateRoutingModule } from './tab-navigate-routing.module';
import { LocationPickerModule } from '../shared/location-picker/location-picker.module';
import { MapModalModule } from '../shared/map-modal/map-modal.module';
@NgModule({
  imports: [
    IonicModule,
    CommonModule,
    FormsModule,
    ExploreContainerComponentModule,
    TabNavigateRoutingModule,
    LocationPickerModule,
    MapModalModule
  ],
  declarations: [TabNavigatePage]
})
export class TabNavigateModule { }
