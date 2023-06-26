import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';

import { TabStatisticsPage } from './tab-statistics.page';

const routes: Routes = [
  {
    path: '',
    component: TabStatisticsPage
  }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule],
})
export class TabStatisticsPageRoutingModule {}
