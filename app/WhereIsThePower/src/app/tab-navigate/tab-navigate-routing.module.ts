import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { TabNavigatePage } from './tab-navigate.page';

const routes: Routes = [
  {
    path: '',
    component: TabNavigatePage,
  }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class TabNavigateRoutingModule {}
