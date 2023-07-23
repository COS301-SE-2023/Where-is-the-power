import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { TabsPage } from './tabs.page';

const routes: Routes = [
  {
    path: 'tabs',
    component: TabsPage,
    children: [
      {
        path: 'tab-navigate',
        loadChildren: () => import('../tab-navigate/tab-navigate.module').then(m => m.TabNavigateModule)
      },
      {
        path: 'tab-saved',
        loadChildren: () => import('../tab-saved/tab-saved.module').then(m => m.TabSavedPageModule)
      },
      {
        path: 'tab-schedule',
        loadChildren: () => import('../tab-schedule/tab-schedule.module').then(m => m.TabSchedulePageModule)
      },
      {
        path: 'tab-statistics',
        loadChildren: () => import('../tab-statistics/tab-statistics.module').then(m => m.TabStatisticsPageModule)
      },
      {
        path: 'tab-profile',
        loadChildren: () => import('../tab-profile/tab-profile.module').then(m => m.TabProfilePageModule)
      },
      {
        path: '',
        redirectTo: '/tabs/tab-navigate',
        pathMatch: 'full'
      }
    ]
  },
  {
    path: '',
    redirectTo: '/tabs/tab-navigate',
    pathMatch: 'full'
  }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
})
export class TabsPageRoutingModule { }
