// @ts-nocheck
import { async, ComponentFixture, TestBed } from '@angular/core/testing';
import { Pipe, PipeTransform, Injectable, CUSTOM_ELEMENTS_SCHEMA, NO_ERRORS_SCHEMA, Directive, Input, Output } from '@angular/core';
import { isPlatformBrowser } from '@angular/common';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { By } from '@angular/platform-browser';
import { Observable, of as observableOf, throwError } from 'rxjs';

import { Component } from '@angular/core';
import { TabSchedulePage } from './tab-schedule.page';
import { UserLocationService } from '../user-location.service';
import { ScheduleService } from './schedule.service';
import { HttpClient } from '@angular/common/http';

@Injectable()
class MockUserLocationService {}

@Injectable()
class MockScheduleService {}

@Injectable()
class MockHttpClient {
  post() {};
}

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

describe('TabSchedulePage', () => {
  let fixture;
  let component;

  beforeEach(() => {
    TestBed.configureTestingModule({
      imports: [ FormsModule, ReactiveFormsModule ],
      declarations: [
        TabSchedulePage,
        TranslatePipe, PhoneNumberPipe, SafeHtmlPipe,
        MyCustomDirective
      ],
      schemas: [ CUSTOM_ELEMENTS_SCHEMA, NO_ERRORS_SCHEMA ],
      providers: [
        { provide: UserLocationService, useClass: MockUserLocationService },
        { provide: ScheduleService, useClass: MockScheduleService },
        { provide: HttpClient, useClass: MockHttpClient }
      ]
    }).overrideComponent(TabSchedulePage, {

    }).compileComponents();
    fixture = TestBed.createComponent(TabSchedulePage);
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
    spyOn(component.http, 'get').and.returnValue(observableOf({}));
    component.scheduleService = component.scheduleService || {};
    spyOn(component.scheduleService, 'getLoadSheddingStage').and.returnValue(observableOf({
      result: {}
    }));
    spyOn(component, 'setChipColor');
    await component.ngOnInit();
    // expect(component.http.get).toHaveBeenCalled();
    // expect(component.scheduleService.getLoadSheddingStage).toHaveBeenCalled();
    // expect(component.setChipColor).toHaveBeenCalled();
  });

  it('should run #ionViewWillEnter()', async () => {
    component.userLocationService = component.userLocationService || {};
    spyOn(component.userLocationService, 'getUserLocation');
    component.userLocationService.isLocationAvailable = observableOf({});
    spyOn(component.userLocationService, 'getArea');
    spyOn(component, 'selectSuburb');
    await component.ionViewWillEnter();
    // expect(component.userLocationService.getUserLocation).toHaveBeenCalled();
    // expect(component.userLocationService.getArea).toHaveBeenCalled();
    // expect(component.selectSuburb).toHaveBeenCalled();
  });

  it('should run #onSearch()', async () => {
    component.searchTerm = component.searchTerm || {};
    spyOn(component.searchTerm, 'toLowerCase');
    component.searchItems = component.searchItems || {};
    component.searchItems = ['searchItems'];
    component.onSearch({});
    // expect(component.searchTerm.toLowerCase).toHaveBeenCalled();
  });

  it('should run #onBlur()', async () => {

    component.onBlur();

  });

  it('should run #undefined()', async () => {
    // Error: ERROR this JS code is invalid, "data.result.timesOff.forEach((timeOff)"
    //     at Function.getFuncReturn (C:\Users\tumis\Documents\Where-is-the-power\app\WhereIsThePower\node_modules\ngentest\lib\util.js:325:13)
    //     at C:\Users\tumis\Documents\Where-is-the-power\app\WhereIsThePower\node_modules\ngentest\lib\util.js:413:30
    //     at Array.forEach (<anonymous>)
    //     at Function.getFuncParamObj (C:\Users\tumis\Documents\Where-is-the-power\app\WhereIsThePower\node_modules\ngentest\lib\util.js:396:26)
    //     at Function.getFuncArguments (C:\Users\tumis\Documents\Where-is-the-power\app\WhereIsThePower\node_modules\ngentest\lib\util.js:347:30)
    //     at Function.getFuncReturn (C:\Users\tumis\Documents\Where-is-the-power\app\WhereIsThePower\node_modules\ngentest\lib\util.js:332:34)
    //     at FuncTestGen.setMockData (C:\Users\tumis\Documents\Where-is-the-power\app\WhereIsThePower\node_modules\ngentest\lib\func-test-gen.js:159:31)
    //     at FuncTestGen.setMockData (C:\Users\tumis\Documents\Where-is-the-power\app\WhereIsThePower\node_modules\ngentest\lib\func-test-gen.js:154:14)
    //     at FuncTestGen.setMockData (C:\Users\tumis\Documents\Where-is-the-power\app\WhereIsThePower\node_modules\ngentest\lib\func-test-gen.js:90:12)
    //     at C:\Users\tumis\Documents\Where-is-the-power\app\WhereIsThePower\node_modules\ngentest\lib\index.js:188:17
  });

  it('should run #convertToDateTime()', async () => {

    component.convertToDateTime({});

  });

  it('should run #formatTime()', async () => {

    component.formatTime({});

  });

  it('should run #setChipColor()', async () => {

    component.setChipColor({});

  });

  it('should run #ngOnDestroy()', async () => {
    component.suburbDataSubscription = component.suburbDataSubscription || {};
    spyOn(component.suburbDataSubscription, 'unsubscribe');
    component.listSuburbsSubscription = component.listSuburbsSubscription || {};
    spyOn(component.listSuburbsSubscription, 'unsubscribe');
    component.ngOnDestroy();
    // expect(component.suburbDataSubscription.unsubscribe).toHaveBeenCalled();
    // expect(component.listSuburbsSubscription.unsubscribe).toHaveBeenCalled();
  });

});
// Error: ERROR this JS code is invalid, "data.result.timesOff.forEach((timeOff)"