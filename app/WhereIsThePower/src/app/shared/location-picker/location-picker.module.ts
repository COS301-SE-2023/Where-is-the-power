import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { IonicModule } from '@ionic/angular';
import { LocationPickerComponent } from './location-picker.component';
import { HttpClientModule } from '@angular/common/http';
@NgModule({
  declarations: [
    LocationPickerComponent,
  ],
  imports: [
    CommonModule,
    IonicModule,
    HttpClientModule
  ],
  exports: [
    LocationPickerComponent
  ]
})
export class LocationPickerModule { }
