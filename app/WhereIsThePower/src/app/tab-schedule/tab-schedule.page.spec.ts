import { ComponentFixture, TestBed } from '@angular/core/testing';
import { IonicModule } from '@ionic/angular';

import { ExploreContainerComponentModule } from '../explore-container/explore-container.module';

import { TabSchedulePage } from './tab-schedule.page';

describe('TabSchedulePage', () => {
  let component: TabSchedulePage;
  let fixture: ComponentFixture<TabSchedulePage>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [TabSchedulePage],
      imports: [IonicModule.forRoot(), ExploreContainerComponentModule]
    }).compileComponents();

    fixture = TestBed.createComponent(TabSchedulePage);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
