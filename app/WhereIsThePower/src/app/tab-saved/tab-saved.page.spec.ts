import { ComponentFixture, TestBed } from '@angular/core/testing';
import { IonicModule } from '@ionic/angular';

import { ExploreContainerComponentModule } from '../explore-container/explore-container.module';

import { TabSavedPage } from './tab-saved.page';

describe('TabSavedPage', () => {
  let component: TabSavedPage;
  let fixture: ComponentFixture<TabSavedPage>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [TabSavedPage],
      imports: [IonicModule.forRoot(), ExploreContainerComponentModule]
    }).compileComponents();

    fixture = TestBed.createComponent(TabSavedPage);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
