import { IonicModule } from '@ionic/angular';
import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { TabSchedulePage } from './tab-schedule.page';
import { TabSchedulePageRoutingModule } from './tab-schedule-routing.module';

@NgModule({
  imports: [
    IonicModule,
    CommonModule,
    FormsModule,
    TabSchedulePageRoutingModule
  ],
  declarations: [TabSchedulePage]
})
export class TabSchedulePageModule { }
