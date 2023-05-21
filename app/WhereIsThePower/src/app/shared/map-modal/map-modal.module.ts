import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { IonicModule } from '@ionic/angular';
import { MapModalComponent } from './map-modal.component';

@NgModule({
  declarations: [
    MapModalComponent
  ],
  imports: [
    CommonModule,
    IonicModule
  ],
  exports: [
    MapModalComponent
  ]
})
export class MapModalModule { }
