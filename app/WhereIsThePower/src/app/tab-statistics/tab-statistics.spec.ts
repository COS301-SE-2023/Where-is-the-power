import { async, ComponentFixture, TestBed, inject, tick, fakeAsync } from '@angular/core/testing';
import { Component, DebugElement } from '@angular/core';
import { By }  from '@angular/platform-browser';

import { TabStatisticsPage } from './tab-statistics.page';

describe('TabStaticsPage', () => {
  let tabStaticsPage: TabStatisticsPage;
  let fixture: ComponentFixture<TabStatisticsPage>;
  let de: DebugElement;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ TabStatisticsPage ],
    }).compileComponents();
  }));

  beforeEach(() => {
      fixture = TestBed.createComponent(TabStatisticsPage);
      tabStaticsPage = fixture.componentInstance;
      de = fixture.debugElement;

      fixture.detectChanges();
  });

  it('should create', () => {
    expect(Component).toBeTruthy();
  });

  it('should create Doughnut Graph', () => {
    expect(de.query(By.css('#doughnutChart'))).toBeTruthy();
  });

  it('should create Bar Chart', () => {
    expect(de.query(By.css('#barChart'))).toBeTruthy();
  });

  

});