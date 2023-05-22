import { IonicModule } from '@ionic/angular';
import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { TabSchedulePage } from './tab-schedule.page';
import { LocationPickerModule } from '../shared/location-picker/location-picker.module';

import { TabSchedulePageRoutingModule } from './tab-schedule-routing.module';

@NgModule({
  imports: [
    IonicModule,
    CommonModule,
    FormsModule,
    LocationPickerModule,
    TabSchedulePageRoutingModule
  ],
  declarations: [TabSchedulePage]
})
export class TabSchedulePageModule { }
