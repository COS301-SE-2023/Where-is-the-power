// @ts-nocheck
import { async, ComponentFixture, TestBed } from '@angular/core/testing';
import { Pipe, PipeTransform, Injectable, CUSTOM_ELEMENTS_SCHEMA, NO_ERRORS_SCHEMA, Directive, Input, Output } from '@angular/core';
import { isPlatformBrowser } from '@angular/common';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { By } from '@angular/platform-browser';
import { Observable, of as observableOf, throwError } from 'rxjs';

import { Component } from '@angular/core';
import { TabNavigatePage } from './tab-navigate.page';
import { UserLocationService } from '../user-location.service';
import { SavedPlacesService } from '../tab-saved/saved-places.service';

@Injectable()
class MockUserLocationService {}

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

describe('TabNavigatePage', () => {
  let fixture;
  let component;

  beforeEach(() => {
    TestBed.configureTestingModule({
      imports: [ FormsModule, ReactiveFormsModule ],
      declarations: [
        TabNavigatePage,
        TranslatePipe, PhoneNumberPipe, SafeHtmlPipe,
        MyCustomDirective
      ],
      schemas: [ CUSTOM_ELEMENTS_SCHEMA, NO_ERRORS_SCHEMA ],
      providers: [
        { provide: UserLocationService, useClass: MockUserLocationService },
        { provide: SavedPlacesService, useClass: MockSavedPlacesService }
      ]
    }).overrideComponent(TabNavigatePage, {

    }).compileComponents();
    fixture = TestBed.createComponent(TabNavigatePage);
    component = fixture.debugElement.componentInstance;
  });

  afterEach(() => {
    component.ngOnDestroy = function() {};
    fixture.destroy();
  });

  it('should run #constructor()', async () => {
    expect(component).toBeTruthy();
  });

  it('should run #ionViewDidEnter()', async () => {
    component.UserLocationService = component.UserLocationService || {};
    spyOn(component.UserLocationService, 'getUserLocation');
    component.UserLocationService.isLocationAvailable = observableOf({});
    component.mapModalComponent = component.mapModalComponent || {};
    component.mapModalComponent = ['mapModalComponent'];
    await component.ionViewDidEnter();
    // expect(component.UserLocationService.getUserLocation).toHaveBeenCalled();
  });

  it('should run #onLocateUser()', async () => {
    component.UserLocationService = component.UserLocationService || {};
    spyOn(component.UserLocationService, 'getUserLocation');
    component.onLocateUser();
    // expect(component.UserLocationService.getUserLocation).toHaveBeenCalled();
  });

  it('should run #ionViewDidLeave()', async () => {
    component.mapModalComponent = component.mapModalComponent || {};
    component.mapModalComponent.searchBar = {
      value: {}
    };
    component.savedPlacesService = component.savedPlacesService || {};
    component.savedPlacesService.navigateToPlace = {
      next: function() {}
    };
    component.savedPlacesService.navigateToSavedPlace = {
      next: function() {}
    };
    component.ionViewDidLeave();

  });

});