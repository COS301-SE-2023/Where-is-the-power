// @ts-nocheck
import { async, ComponentFixture, TestBed } from '@angular/core/testing';
import { Pipe, PipeTransform, Injectable, CUSTOM_ELEMENTS_SCHEMA, NO_ERRORS_SCHEMA, Directive, Input, Output } from '@angular/core';
import { isPlatformBrowser } from '@angular/common';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { By } from '@angular/platform-browser';
import { Observable, of as observableOf, throwError } from 'rxjs';

import { Component, Renderer2 } from '@angular/core';
import { TabStatisticsPage } from './tab-statistics.page';
import { StatisticsService } from './statistics.service';
import { HttpClient } from '@angular/common/http';
import { UserLocationService } from '../user-location.service';

@Injectable()
class MockStatisticsService {}

@Injectable()
class MockHttpClient {
  post() {};
}

@Injectable()
class MockUserLocationService {}

@Directive({ selector: '[myCustom]' })
class MyCustomDirective {
  @Input() myCustom;
}

@Pipe({name: 'translate'})
class TranslatePipe implements PipeTransform {
  transform(value) { return value; }
}

@Pipe({name: 'phoneNumber'})
class PhoneNumberPipe implements PipeTransform {
  transform(value) { return value; }
}

@Pipe({name: 'safeHtml'})
class SafeHtmlPipe implements PipeTransform {
  transform(value) { return value; }
}

describe('TabStatisticsPage', () => {
  let fixture;
  let component;

  beforeEach(() => {
    TestBed.configureTestingModule({
      imports: [ FormsModule, ReactiveFormsModule ],
      declarations: [
        TabStatisticsPage,
        TranslatePipe, PhoneNumberPipe, SafeHtmlPipe,
        MyCustomDirective
      ],
      schemas: [ CUSTOM_ELEMENTS_SCHEMA, NO_ERRORS_SCHEMA ],
      providers: [
        { provide: StatisticsService, useClass: MockStatisticsService },
        { provide: HttpClient, useClass: MockHttpClient },
        Renderer2,
        { provide: UserLocationService, useClass: MockUserLocationService }
      ]
    }).overrideComponent(TabStatisticsPage, {

    }).compileComponents();
    fixture = TestBed.createComponent(TabStatisticsPage);
    component = fixture.debugElement.componentInstance;
  });

  afterEach(() => {
    component.ngOnDestroy = function() {};
    fixture.destroy();
  });

  it('should run #constructor()', async () => {
    expect(component).toBeTruthy();
  });

  it('should run #ngOnInit()', async () => {
    component.http = component.http || {};
    component.http.get = jest.fn().mockReturnValue(observableOf({}));
    await component.ngOnInit();
    // expect(component.http.get).toHaveBeenCalled();
  });

  it('should run #ionViewWillEnter()', async () => {
    component.userLocationService = component.userLocationService || {};
    component.userLocationService.getUserLocation = jest.fn();
    component.userLocationService.isLocationAvailable = observableOf({});
    component.userLocationService.getArea = jest.fn();
    component.selectSuburb = jest.fn();
    await component.ionViewWillEnter();
    // expect(component.userLocationService.getUserLocation).toHaveBeenCalled();
    // expect(component.userLocationService.getArea).toHaveBeenCalled();
    // expect(component.selectSuburb).toHaveBeenCalled();
  });

  it('should run #processDoughnutChart()', async () => {
    component.populateDoughnutChart = jest.fn();
    component.processDoughnutChart({
      result: {
        perDayTimes: {
          today: {
            on: {},
            off: {}
          }
        }
      }
    });
    // expect(component.populateDoughnutChart).toHaveBeenCalled();
  });

  it('should run #populateDoughnutChart()', async () => {
    component.doughnutChart = component.doughnutChart || {};
    component.doughnutChart.clear = jest.fn();
    component.doughnutChart.destroy = jest.fn();
    component.populateDoughnutChart({});
    // expect(component.doughnutChart.clear).toHaveBeenCalled();
    // expect(component.doughnutChart.destroy).toHaveBeenCalled();
  });

  it('should run #processBarChart()', async () => {
    component.populateBarChart = jest.fn();
    component.processBarChart({
      result: {
        perDayTimes: {
          day: {
            on: {},
            off: {}
          }
        }
      }
    });
    // expect(component.populateBarChart).toHaveBeenCalled();
  });

  it('should run #populateBarChart()', async () => {
    component.barChart = component.barChart || {};
    component.barChart.clear = jest.fn();
    component.barChart.destroy = jest.fn();
    component.barChartRef = component.barChartRef || {};
    component.barChartRef.nativeElement = 'nativeElement';
    component.populateBarChart({});
    // expect(component.barChart.clear).toHaveBeenCalled();
    // expect(component.barChart.destroy).toHaveBeenCalled();
  });

  it('should run #clearDoughnutChart()', async () => {

    component.clearDoughnutChart();

  });

  it('should run #clearBarChart()', async () => {

    component.clearBarChart();

  });

  it('should run #clearAllCharts()', async () => {
    component.clearBarChart = jest.fn();
    component.clearDoughnutChart = jest.fn();
    component.clearAllCharts();
    // expect(component.clearBarChart).toHaveBeenCalled();
    // expect(component.clearDoughnutChart).toHaveBeenCalled();
  });

  it('should run #onSearch()', async () => {
    component.searchTerm = component.searchTerm || {};
    component.searchTerm.toLowerCase = jest.fn();
    component.searchItems = component.searchItems || {};
    component.searchItems = ['searchItems'];
    component.onSearch({});
    // expect(component.searchTerm.toLowerCase).toHaveBeenCalled();
  });

  it('should run #onBlur()', async () => {

    component.onBlur();

  });

  it('should run #selectSuburb()', async () => {
    component.statisticsService = component.statisticsService || {};
    component.statisticsService.getSuburbData = jest.fn().mockReturnValue(observableOf({
      result: {}
    }));
    component.processDoughnutChart = jest.fn();
    component.processBarChart = jest.fn();
    component.selectSuburb({
      name: {},
      id: {}
    });
    // expect(component.statisticsService.getSuburbData).toHaveBeenCalled();
    // expect(component.processDoughnutChart).toHaveBeenCalled();
    // expect(component.processBarChart).toHaveBeenCalled();
  });

  it('should run #ngOnDestroy()', async () => {
    component.isLocationAvailableSubscription = component.isLocationAvailableSubscription || {};
    component.isLocationAvailableSubscription.unsubscribe = jest.fn();
    component.suburbDataSubscription = component.suburbDataSubscription || {};
    component.suburbDataSubscription.unsubscribe = jest.fn();
    component.listSuburbsSubscription = component.listSuburbsSubscription || {};
    component.listSuburbsSubscription.unsubscribe = jest.fn();
    component.ngOnDestroy();
    // expect(component.isLocationAvailableSubscription.unsubscribe).toHaveBeenCalled();
    // expect(component.suburbDataSubscription.unsubscribe).toHaveBeenCalled();
    // expect(component.listSuburbsSubscription.unsubscribe).toHaveBeenCalled();
  });

});