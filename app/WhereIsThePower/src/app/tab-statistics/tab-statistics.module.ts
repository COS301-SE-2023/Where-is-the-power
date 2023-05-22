import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';

import { IonicModule } from '@ionic/angular';

import { TabStatisticsPageRoutingModule } from './tab-statistics-routing.module';

import { TabStatisticsPage } from './tab-statistics.page';

@NgModule({
  imports: [
    CommonModule,
    FormsModule,
    IonicModule,
    TabStatisticsPageRoutingModule
  ],
  declarations: [TabStatisticsPage]
})
export class TabStatisticsPageModule { }
