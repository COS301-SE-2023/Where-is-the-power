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
