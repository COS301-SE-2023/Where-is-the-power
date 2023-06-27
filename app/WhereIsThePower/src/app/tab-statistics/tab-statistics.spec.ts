import { async, ComponentFixture, TestBed, inject, tick, fakeAsync } from '@angular/core/testing';
import { CUSTOM_ELEMENTS_SCHEMA, Component, DebugElement } from '@angular/core';
import { By }  from '@angular/platform-browser';

import { TabStatisticsPage } from './tab-statistics.page';

describe('TabStaticsPage', () => {
  let tabStaticsPage: TabStatisticsPage;
  let fixture: ComponentFixture<TabStatisticsPage>;
  let de: DebugElement;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ TabStatisticsPage ],
      schemas: [CUSTOM_ELEMENTS_SCHEMA]
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

  it('should call populate Doughnut Graph', () => {
    const doughnutData = {
      labels: ['Uptime', 'Downtime'],
      datasets: [{
        label: 'Loadshedding',
        data: [20, 4], // Uptime vs Downtime
        borderWidth: 0,
        backgroundColor: [
          '#007A4D',
          '#DE3831',
        ],
      }]
    };

    spyOn(tabStaticsPage, 'populateDoughnutChart');
    tabStaticsPage.populateDoughnutChart(doughnutData);
    expect(tabStaticsPage.populateDoughnutChart).toHaveBeenCalledTimes(1);
  });

  it('should call populate Bar Chart', () => {
    const doughnutData = {
      labels: ['Uptime', 'Downtime'],
      datasets: [{
        label: 'Loadshedding',
        data: [20, 4], // Uptime vs Downtime
        borderWidth: 0,
        backgroundColor: [
          '#007A4D',
          '#DE3831',
        ],
      }]
    };

    spyOn(tabStaticsPage, 'populateDoughnutChart')
    tabStaticsPage.populateDoughnutChart(doughnutData);
    expect(tabStaticsPage.populateDoughnutChart).toHaveBeenCalledTimes(1);
  });


  it('should expect doughnut graph to be defined', () => {
    expect(tabStaticsPage.doughnutChart).toBeDefined();
  });

  
  it('should expect bar chart to be defined', () => {
    expect(tabStaticsPage.barChart).toBeDefined();
  });

  it('should clear Bar chart', () => {
    tabStaticsPage.clearBarChart();
    expect(tabStaticsPage.barChart).toBeNull();
  });

  it('should clear Doughnut Graph', () => {
    tabStaticsPage.clearDoughnutChart();
    expect(tabStaticsPage.doughnutChart).toBeNull();
  });

  // Integration test
  it('should call both Bar Chart and Doughnut Graph with clearAllCharts', () => {
    spyOn(tabStaticsPage, 'clearDoughnutChart');
    spyOn(tabStaticsPage, 'clearBarChart');
    tabStaticsPage.clearAllCharts();
    expect(tabStaticsPage.clearDoughnutChart).toHaveBeenCalledTimes(1);
    expect(tabStaticsPage.clearBarChart).toHaveBeenCalledTimes(1);
  });

  it('should set Bar Chart and Doughnut Graph to null with clearAllCharts', () => {
    expect(tabStaticsPage.barChart).toBeDefined();
    expect(tabStaticsPage.doughnutChart).toBeDefined();
    tabStaticsPage.clearAllCharts();
    expect(tabStaticsPage.barChart).toBeNull();
    expect(tabStaticsPage.doughnutChart).toBeNull();
  });

});