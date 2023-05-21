import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { TabSchedulePage } from './tab-schedule.page';

const routes: Routes = [
  {
    path: '',
    component: TabSchedulePage,
  }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule],
})
export class TabSchedulePageRoutingModule { }
