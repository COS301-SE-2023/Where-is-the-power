import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { TabSavedPage } from './tab-saved.page';

const routes: Routes = [
  {
    path: '',
    component: TabSavedPage,
  }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class TabSavedPageRoutingModule {}
