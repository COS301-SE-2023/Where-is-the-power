import { ComponentFixture, TestBed } from '@angular/core/testing';
import { IonicModule } from '@ionic/angular';
import { TabStatisticsPage } from './tab-statistics.page';

import { ExploreContainerComponentModule } from '../explore-container/explore-container.module';

describe('TabStatisticsPage', () => {
  let component: TabStatisticsPage;
  let fixture: ComponentFixture<TabStatisticsPage>;

  beforeEach(async() => {
    await TestBed.configureTestingModule({
      declarations: [TabStatisticsPage],
      imports: [IonicModule.forRoot(), ExploreContainerComponentModule]
    }).compileComponents();

    fixture = TestBed.createComponent(TabStatisticsPage);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
