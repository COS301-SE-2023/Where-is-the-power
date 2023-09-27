// @ts-nocheck
import { async, ComponentFixture, TestBed } from '@angular/core/testing';
import { Pipe, PipeTransform, Injectable, CUSTOM_ELEMENTS_SCHEMA, NO_ERRORS_SCHEMA, Directive, Input, Output } from '@angular/core';
import { isPlatformBrowser } from '@angular/common';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { By } from '@angular/platform-browser';
import { Observable, of as observableOf, throwError } from 'rxjs';

import { Component } from '@angular/core';
import { TabSavedPage } from './tab-saved.page';
import { Router } from '@angular/router';
import { UserLocationService } from '../user-location.service';
import { HttpClient } from '@angular/common/http';
import { AuthService } from '../authentication/auth.service';
import { SavedPlacesService } from './saved-places.service';
import { ToastController } from '@ionic/angular';

@Injectable()
class MockRouter {
  navigate() {};
}

@Injectable()
class MockUserLocationService {}

@Injectable()
class MockHttpClient {
  post() {};
}

@Injectable()
class MockAuthService {}

@Injectable()
class MockSavedPlacesService {}

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

describe('TabSavedPage', () => {
  let fixture;
  let component;

  beforeEach(() => {
    TestBed.configureTestingModule({
      imports: [ FormsModule, ReactiveFormsModule ],
      declarations: [
        TabSavedPage,
        TranslatePipe, PhoneNumberPipe, SafeHtmlPipe,
        MyCustomDirective
      ],
      schemas: [ CUSTOM_ELEMENTS_SCHEMA, NO_ERRORS_SCHEMA ],
      providers: [
        { provide: Router, useClass: MockRouter },
        { provide: UserLocationService, useClass: MockUserLocationService },
        { provide: HttpClient, useClass: MockHttpClient },
        { provide: AuthService, useClass: MockAuthService },
        { provide: SavedPlacesService, useClass: MockSavedPlacesService },
        ToastController
      ]
    }).overrideComponent(TabSavedPage, {

    }).compileComponents();
    fixture = TestBed.createComponent(TabSavedPage);
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

    component.ngOnInit();

  });

  it('should run #gotoProfileRoute()', async () => {
    component.router = component.router || {};
    component.router.navigate = jest.fn();
    component.gotoProfileRoute();
    // expect(component.router.navigate).toHaveBeenCalled();
  });

  it('should run #ionViewDidEnter()', async () => {
    component.authService = component.authService || {};
    component.authService.isUserLoggedIn = jest.fn();
    component.savedPlaceService = component.savedPlaceService || {};
    component.savedPlaceService.getPlaces = jest.fn().mockReturnValue(observableOf({
      result: {}
    }));
    component.savedPlaceService.savePlace = observableOf({});
    component.savedPlaceService.savedPlace = 'savedPlace';
    component.router = component.router || {};
    component.router.navigate = jest.fn();
    component.addSavedPlace = jest.fn();
    await component.ionViewDidEnter();
    // expect(component.authService.isUserLoggedIn).toHaveBeenCalled();
    // expect(component.savedPlaceService.getPlaces).toHaveBeenCalled();
    // expect(component.router.navigate).toHaveBeenCalled();
    // expect(component.addSavedPlace).toHaveBeenCalled();
  });

  it('should run #ionViewDidLeave()', async () => {

    component.ionViewDidLeave();

  });

  it('should run #goToPlace()', async () => {
    component.savedPlaceService = component.savedPlaceService || {};
    component.savedPlaceService.goToPlace = jest.fn();
    component.goToPlace({});
    // expect(component.savedPlaceService.goToPlace).toHaveBeenCalled();
  });

  it('should run #goToSavedPlace()', async () => {
    component.savedPlaceService = component.savedPlaceService || {};
    component.savedPlaceService.navigateToSavedPlace = {
      next: function() {}
    };
    component.savedPlaceService.goToPlace = jest.fn();
    component.goToSavedPlace({});
    // expect(component.savedPlaceService.goToPlace).toHaveBeenCalled();
  });

  it('should run #savePlace()', async () => {
    component.goToPlace = jest.fn();
    component.savePlace({
      id: {},
      text: {},
      place_name: {},
      center: {
        0: {},
        1: {}
      }
    });
    // expect(component.goToPlace).toHaveBeenCalled();
  });

  it('should run #addSavedPlace()', async () => {
    component.isPlaceSaved = jest.fn();
    component.savedPlaceService = component.savedPlaceService || {};
    component.savedPlaceService.addSavedPlace = jest.fn().mockReturnValue(observableOf({}));
    component.savedPlaceService.place = {
      next: function() {}
    };
    component.places = component.places || {};
    component.addSavedPlace({});
    // expect(component.isPlaceSaved).toHaveBeenCalled();
    // expect(component.savedPlaceService.addSavedPlace).toHaveBeenCalled();
  });

  it('should run #deleteSavedPlace()', async () => {
    component.savedPlaceService = component.savedPlaceService || {};
    component.savedPlaceService.deleteSavedPlace = jest.fn().mockReturnValue(observableOf({}));
    component.places = component.places || {};
    component.places = ['places'];
    component.deleteSavedPlace({
      mapboxId: {}
    });
    // expect(component.savedPlaceService.deleteSavedPlace).toHaveBeenCalled();
  });

  it('should run #isPlaceSaved()', async () => {
    component.savedPlaces = component.savedPlaces || {};
    component.savedPlaces = ['savedPlaces'];
    component.isPlaceSaved({
      id: {}
    });

  });

  it('should run #isAddress()', async () => {

    component.isAddress({});

  });

  it('should run #getFeatureType()', async () => {

    component.getFeatureType({});

  });

  it('should run #undefined()', async () => {
    // Error: ERROR this JS code is invalid, "data.features.map((feature)"
    //     at Function.getFuncReturn (C:\Users\tumis\Documents\Where-is-the-power\app\WhereIsThePower\node_modules\ngentest\lib\util.js:325:13)
    //     at C:\Users\tumis\Documents\Where-is-the-power\app\WhereIsThePower\node_modules\ngentest\lib\util.js:413:30
    //     at Array.forEach (<anonymous>)
    //     at Function.getFuncParamObj (C:\Users\tumis\Documents\Where-is-the-power\app\WhereIsThePower\node_modules\ngentest\lib\util.js:396:26)
    //     at Function.getFuncArguments (C:\Users\tumis\Documents\Where-is-the-power\app\WhereIsThePower\node_modules\ngentest\lib\util.js:347:30)
    //     at Function.getFuncReturn (C:\Users\tumis\Documents\Where-is-the-power\app\WhereIsThePower\node_modules\ngentest\lib\util.js:332:34)
    //     at FuncTestGen.setMockData (C:\Users\tumis\Documents\Where-is-the-power\app\WhereIsThePower\node_modules\ngentest\lib\func-test-gen.js:159:31)
    //     at FuncTestGen.setMockData (C:\Users\tumis\Documents\Where-is-the-power\app\WhereIsThePower\node_modules\ngentest\lib\func-test-gen.js:172:12)
    //     at FuncTestGen.setMockData (C:\Users\tumis\Documents\Where-is-the-power\app\WhereIsThePower\node_modules\ngentest\lib\func-test-gen.js:163:12)
    //     at FuncTestGen.setMockData (C:\Users\tumis\Documents\Where-is-the-power\app\WhereIsThePower\node_modules\ngentest\lib\func-test-gen.js:90:12)
  });

  it('should run #onSearchBarFocus()', async () => {
    component.searchBar = component.searchBar || {};
    component.searchBar.value = {
      length: {}
    };
    component.onSearchBarFocus();

  });

  it('should run #onSearchBarClear()', async () => {

    component.onSearchBarClear();

  });

  it('should run #onBlur()', async () => {

    component.onBlur();

  });

  it('should run #sucessToast()', async () => {
    component.toastController = component.toastController || {};
    component.toastController.create = jest.fn();
    await component.sucessToast({});
    // expect(component.toastController.create).toHaveBeenCalled();
  });

  it('should run #ngOnDestroy()', async () => {
    component.placesSubscription = component.placesSubscription || {};
    component.placesSubscription.unsubscribe = jest.fn();
    component.ngOnDestroy();
    // expect(component.placesSubscription.unsubscribe).toHaveBeenCalled();
  });

});Error: ERROR this JS code is invalid, "data.features.map((feature)"