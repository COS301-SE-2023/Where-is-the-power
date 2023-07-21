import { ComponentFixture, TestBed } from '@angular/core/testing';
import { TabProfilePage } from './tab-profile.page';

describe('TabProfilePage', () => {
  let component: TabProfilePage;
  let fixture: ComponentFixture<TabProfilePage>;

  beforeEach(async(() => {
    fixture = TestBed.createComponent(TabProfilePage);
    component = fixture.componentInstance;
    fixture.detectChanges();
  }));

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
