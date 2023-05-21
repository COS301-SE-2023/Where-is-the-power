import { IonicModule } from '@ionic/angular';
import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { TabSavedPage } from './tab-saved.page';
import { ExploreContainerComponentModule } from '../explore-container/explore-container.module';

import { TabSavedPageRoutingModule } from './tab-saved-routing.module';

@NgModule({
  imports: [
    IonicModule,
    CommonModule,
    FormsModule,
    ExploreContainerComponentModule,
    TabSavedPageRoutingModule
  ],
  declarations: [TabSavedPage]
})
export class TabSavedPageModule {}
