import { ComponentFixture, TestBed } from '@angular/core/testing';
import { IonicModule } from '@ionic/angular';

import { ExploreContainerComponentModule } from '../explore-container/explore-container.module';

import { TabNavigatePage } from './tab-navigate.page';

describe('TabNavigatePage', () => {
  let component: TabNavigatePage;
  let fixture: ComponentFixture<TabNavigatePage>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [TabNavigatePage],
      imports: [IonicModule.forRoot(), ExploreContainerComponentModule]
    }).compileComponents();

    fixture = TestBed.createComponent(TabNavigatePage);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
