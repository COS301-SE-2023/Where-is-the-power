import { Component } from '@angular/core';
import { MapSuburbsService } from '../shared/map-modal/map-suburbs.service';
@Component({
  selector: 'app-tabs',
  templateUrl: 'tabs.page.html',
  styleUrls: ['tabs.page.scss']
})
export class TabsPage {

  constructor(private MapSuburbsService: MapSuburbsService) { }
  hideBottomTabs = false;

  ngOnInit() {
    this.MapSuburbsService.gettingDirections.subscribe((data: boolean) => {
      this.hideBottomTabs = data;
    });
  }
}
