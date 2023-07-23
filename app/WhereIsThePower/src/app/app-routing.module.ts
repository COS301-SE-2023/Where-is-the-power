import { NgModule } from '@angular/core';
import { PreloadAllModules, RouterModule, Routes } from '@angular/router';

const routes: Routes = [
  {
    path: '',
    loadChildren: () => import('./tabs/tabs.module').then(m => m.TabsPageModule)
  },
  {
    path: 'tab-statistics',
    loadChildren: () => import('./tab-statistics/tab-statistics.module').then(m => m.TabStatisticsPageModule)
  },  {
    path: 'tab-profile',
    loadChildren: () => import('./tab-profile/tab-profile.module').then( m => m.TabProfilePageModule)
  }

];
@NgModule({
  imports: [
    RouterModule.forRoot(routes, { preloadingStrategy: PreloadAllModules })
  ],
  exports: [RouterModule]
})
export class AppRoutingModule {}
