// @ts-nocheck
import { async, ComponentFixture, TestBed } from '@angular/core/testing';
import { Pipe, PipeTransform, Injectable, CUSTOM_ELEMENTS_SCHEMA, NO_ERRORS_SCHEMA, Directive, Input, Output } from '@angular/core';
import { isPlatformBrowser } from '@angular/common';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { By } from '@angular/platform-browser';
import { Observable, of as observableOf, throwError } from 'rxjs';

import { Component, ChangeDetectorRef } from '@angular/core';
import { MapModalComponent } from './map-modal.component';
import { MapSuburbsService } from './map-suburbs.service';
import { UserLocationService } from '../../user-location.service';
import { ModalController, LoadingController } from '@ionic/angular';
import { SavedPlacesService } from '../../tab-saved/saved-places.service';
import { Router } from '@angular/router';
import { ReportService } from '../../report/report.service';

@Injectable()
class MockMapSuburbsService {}

@Injectable()
class MockUserLocationService {}

@Injectable()
class MockSavedPlacesService {}

@Injectable()
class MockRouter {
  navigate() {};
}

@Injectable()
class MockReportService {}

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

describe('MapModalComponent', () => {
  let fixture;
  let component;

  beforeEach(() => {
    TestBed.configureTestingModule({
      imports: [ FormsModule, ReactiveFormsModule ],
      declarations: [
        MapModalComponent,
        TranslatePipe, PhoneNumberPipe, SafeHtmlPipe,
        MyCustomDirective
      ],
      schemas: [ CUSTOM_ELEMENTS_SCHEMA, NO_ERRORS_SCHEMA ],
      providers: [
        { provide: MapSuburbsService, useClass: MockMapSuburbsService },
        { provide: UserLocationService, useClass: MockUserLocationService },
        ModalController,
        ChangeDetectorRef,
        { provide: SavedPlacesService, useClass: MockSavedPlacesService },
        { provide: Router, useClass: MockRouter },
        { provide: ReportService, useClass: MockReportService },
        LoadingController
      ]
    }).overrideComponent(MapModalComponent, {

    }).compileComponents();
    fixture = TestBed.createComponent(MapModalComponent);
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
    component.savedPlacesService = component.savedPlacesService || {};
    component.savedPlacesService.navigateToPlace = observableOf({});
    component.savedPlacesService.selectedPlace = {
      address: {
        substring: function() {
          return 'ngentest';
        },
        indexOf: function() {}
      },
      hasOwnProperty: function() {},
      longitude: {},
      latitude: {},
      center: {
        0: {},
        1: {}
      }
    };
    component.savedPlacesService.navigateToSavedPlace = observableOf({});
    component.map = component.map || {};
    spyOn(component.map, 'flyTo');
    spyOn(component, 'openNavigateModal');
    component.ngOnInit();
    // expect(component.map.flyTo).toHaveBeenCalled();
    // expect(component.openNavigateModal).toHaveBeenCalled();
  });

  it('should run #undefined()', async () => {
    // Error: ERROR this JS code is invalid, "reports.forEach((report)"
    //     at Function.getFuncReturn (C:\Users\tumis\Documents\Where-is-the-power\app\WhereIsThePower\node_modules\ngentest\lib\util.js:325:13)
    //     at C:\Users\tumis\Documents\Where-is-the-power\app\WhereIsThePower\node_modules\ngentest\lib\util.js:413:30
    //     at Array.forEach (<anonymous>)
    //     at Function.getFuncParamObj (C:\Users\tumis\Documents\Where-is-the-power\app\WhereIsThePower\node_modules\ngentest\lib\util.js:396:26)
    //     at Function.getFuncArguments (C:\Users\tumis\Documents\Where-is-the-power\app\WhereIsThePower\node_modules\ngentest\lib\util.js:347:30)
    //     at Function.getFuncReturn (C:\Users\tumis\Documents\Where-is-the-power\app\WhereIsThePower\node_modules\ngentest\lib\util.js:332:34)
    //     at FuncTestGen.setMockData (C:\Users\tumis\Documents\Where-is-the-power\app\WhereIsThePower\node_modules\ngentest\lib\func-test-gen.js:159:31)
    //     at FuncTestGen.setMockData (C:\Users\tumis\Documents\Where-is-the-power\app\WhereIsThePower\node_modules\ngentest\lib\func-test-gen.js:90:12)
    //     at C:\Users\tumis\Documents\Where-is-the-power\app\WhereIsThePower\node_modules\ngentest\lib\func-test-gen.js:80:14
    //     at Array.forEach (<anonymous>)
  });

  it('should run #addMarker()', async () => {
    spyOn(component, 'closePopup');
    component.addMarker({}, {}, 'reportType');
    // expect(component.closePopup).toHaveBeenCalled();
  });

  it('should run #populatePolygons()', async () => {
    component.map = component.map || {};
    spyOn(component.map, 'on');
    spyOn(component.map, 'addSource');
    spyOn(component.map, 'addLayer');
    component.mapSuburbsService = component.mapSuburbsService || {};
    spyOn(component.mapSuburbsService, 'fetchTimeForPolygon').and.returnValue(observableOf({
      success: {},
      result: {
        timesOff: {}
      }
    }));
    spyOn(component.mapSuburbsService, 'getSuburbData');
    component.populatePolygons();
    // expect(component.map.on).toHaveBeenCalled();
    // expect(component.map.addSource).toHaveBeenCalled();
    // expect(component.map.addLayer).toHaveBeenCalled();
    // expect(component.mapSuburbsService.fetchTimeForPolygon).toHaveBeenCalled();
    // expect(component.mapSuburbsService.getSuburbData).toHaveBeenCalled();
  });

  it('should run #onSearchBarFocus()', async () => {
    component.searchBar = component.searchBar || {};
    component.searchBar.value = {
      length: {}
    };
    component.onSearchBarFocus();

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

  it('should run #onBlur()', async () => {

    component.onBlur();

  });

  it('should run #getRoute()', async () => {
    component.instructions = component.instructions || {};
    spyOn(component.instructions, 'push');
    spyOn(component, 'updateBreakpoint');
    spyOn(component, 'emitGetDirections');
    spyOn(component, 'closePopup');
    spyOn(component, 'cancelNavigateModal');
    spyOn(component, 'openModal');
    spyOn(component, 'presentLoading');
    component.mapSuburbsService = component.mapSuburbsService || {};
    spyOn(component.mapSuburbsService, 'fetchOptimalRoute').and.returnValue(observableOf({}));
    component.searchBar = component.searchBar || {};
    component.searchBar.value = 'value';
    component.map = component.map || {};
    spyOn(component.map, 'getLayer');
    spyOn(component.map, 'getSource').and.returnValue({
      setData: function() {}
    });
    spyOn(component.map, 'addLayer');
    spyOn(component.map, 'fitBounds');
    spyOn(component, 'calculateETA');
    await component.getRoute({
      hasOwnProperty: function() {},
      longitude: {},
      latitude: {},
      address: {},
      center: {
        0: {},
        1: {}
      },
      place_name: {}
    });
    // expect(component.instructions.push).toHaveBeenCalled();
    // expect(component.updateBreakpoint).toHaveBeenCalled();
    // expect(component.emitGetDirections).toHaveBeenCalled();
    // expect(component.closePopup).toHaveBeenCalled();
    // expect(component.cancelNavigateModal).toHaveBeenCalled();
    // expect(component.openModal).toHaveBeenCalled();
    // expect(component.presentLoading).toHaveBeenCalled();
    // expect(component.mapSuburbsService.fetchOptimalRoute).toHaveBeenCalled();
    // expect(component.map.getLayer).toHaveBeenCalled();
    // expect(component.map.getSource).toHaveBeenCalled();
    // expect(component.map.addLayer).toHaveBeenCalled();
    // expect(component.map.fitBounds).toHaveBeenCalled();
    // expect(component.calculateETA).toHaveBeenCalled();
  });

  it('should run #presentLoading()', async () => {
    component.loadingController = component.loadingController || {};
    spyOn(component.loadingController, 'create');
    await component.presentLoading();
    // expect(component.loadingController.create).toHaveBeenCalled();
  });

  it('should run #delay()', async () => {

    component.delay({});

  });

  it('should run #onSearchBarClear()', async () => {
    component.myModal = component.myModal || {};
    spyOn(component.myModal, 'dismiss');
    spyOn(component, 'emitCancelDirections');
    spyOn(component, 'updateBreakpoint');
    component.map = component.map || {};
    spyOn(component.map, 'getSource');
    spyOn(component.map, 'removeLayer');
    spyOn(component.map, 'removeSource');
    spyOn(component.map, 'getLayer');
    component.onSearchBarClear();
    // expect(component.myModal.dismiss).toHaveBeenCalled();
    // expect(component.emitCancelDirections).toHaveBeenCalled();
    // expect(component.updateBreakpoint).toHaveBeenCalled();
    // expect(component.map.getSource).toHaveBeenCalled();
    // expect(component.map.removeLayer).toHaveBeenCalled();
    // expect(component.map.removeSource).toHaveBeenCalled();
    // expect(component.map.getLayer).toHaveBeenCalled();
  });

  it('should run #centerOnStartPoint()', async () => {
    component.map = component.map || {};
    spyOn(component.map, 'flyTo');
    spyOn(component, 'closePopup');
    component.centerOnStartPoint();
    // expect(component.map.flyTo).toHaveBeenCalled();
    // expect(component.closePopup).toHaveBeenCalled();
  });

  it('should run #openModal()', async () => {
    component.myModal = component.myModal || {};
    spyOn(component.myModal, 'present');
    component.openModal({});
    // expect(component.myModal.present).toHaveBeenCalled();
  });

  it('should run #openNavigateModal()', async () => {
    component.navigateModal = component.navigateModal || {};
    spyOn(component.navigateModal, 'present');
    spyOn(component, 'updateBreakpoint');
    component.openNavigateModal();
    // expect(component.navigateModal.present).toHaveBeenCalled();
    // expect(component.updateBreakpoint).toHaveBeenCalled();
  });

  it('should run #calculateETA()', async () => {
    component.tripETA = component.tripETA || {};
    spyOn(component.tripETA, 'setHours');
    spyOn(component.tripETA, 'getHours');
    spyOn(component.tripETA, 'setMinutes');
    spyOn(component.tripETA, 'getMinutes');
    component.calculateETA();
    // expect(component.tripETA.setHours).toHaveBeenCalled();
    // expect(component.tripETA.getHours).toHaveBeenCalled();
    // expect(component.tripETA.setMinutes).toHaveBeenCalled();
    // expect(component.tripETA.getMinutes).toHaveBeenCalled();
  });

  it('should run #getIconForInstruction()', async () => {

    component.getIconForInstruction({});

  });

  it('should run #beginTrip()', async () => {
    spyOn(component, 'centerOnStartPoint');
    spyOn(component, 'updateBreakpoint');
    component.userMarker = component.userMarker || {};
    spyOn(component.userMarker, 'remove');
    component.beginTrip();
    // expect(component.centerOnStartPoint).toHaveBeenCalled();
    // expect(component.updateBreakpoint).toHaveBeenCalled();
    // expect(component.userMarker.remove).toHaveBeenCalled();
  });

  it('should run #pin()', async () => {
    spyOn(component, 'centerOnStartPoint');
    component.userMarker = component.userMarker || {};
    spyOn(component.userMarker, 'remove');
    component.pin();
    // expect(component.centerOnStartPoint).toHaveBeenCalled();
    // expect(component.userMarker.remove).toHaveBeenCalled();
  });

  it('should run #onModalDismiss()', async () => {
    spyOn(component, 'onSearchBarClear');
    component.onModalDismiss();
    // expect(component.onSearchBarClear).toHaveBeenCalled();
  });

  it('should run #emitCancelDirections()', async () => {
    component.mapSuburbsService = component.mapSuburbsService || {};
    component.mapSuburbsService.gettingDirections = {
      next: function() {}
    };
    component.map = component.map || {};
    spyOn(component.map, 'resize');
    component.emitCancelDirections();
    // expect(component.map.resize).toHaveBeenCalled();
  });

  it('should run #emitGetDirections()', async () => {
    component.mapSuburbsService = component.mapSuburbsService || {};
    component.mapSuburbsService.gettingDirections = {
      next: function() {}
    };
    spyOn(component, 'delay');
    component.map = component.map || {};
    spyOn(component.map, 'resize');
    await component.emitGetDirections();
    // expect(component.delay).toHaveBeenCalled();
    // expect(component.map.resize).toHaveBeenCalled();
  });

  it('should run #onResize()', async () => {
    spyOn(component, 'updateBreakpoint');
    component.onResize();
    // expect(component.updateBreakpoint).toHaveBeenCalled();
  });

  it('should run #updateBreakpoint()', async () => {
    component.myModal = component.myModal || {};
    spyOn(component.myModal, 'setCurrentBreakpoint');
    component.updateBreakpoint();
    // expect(component.myModal.setCurrentBreakpoint).toHaveBeenCalled();
  });

  it('should run #closePopup()', async () => {
    component.popup = component.popup || {};
    spyOn(component.popup, 'remove');
    component.closePopup();
    // expect(component.popup.remove).toHaveBeenCalled();
  });

  it('should run #savePlace()', async () => {
    component.savedPlacesService = component.savedPlacesService || {};
    component.savedPlacesService.savedPlace = 'savedPlace';
    component.savedPlacesService.savePlace = {
      next: function() {}
    };
    spyOn(component, 'cancelNavigateModal');
    component.savePlace();
    // expect(component.cancelNavigateModal).toHaveBeenCalled();
  });

  it('should run #cancelNavigateModal()', async () => {
    component.navigateModal = component.navigateModal || {};
    spyOn(component.navigateModal, 'dismiss');
    component.cancelNavigateModal();
    // expect(component.navigateModal.dismiss).toHaveBeenCalled();
  });

  it('should run #ngOnDestroy()', async () => {
    component.navigateToPlaceSubscription = component.navigateToPlaceSubscription || {};
    spyOn(component.navigateToPlaceSubscription, 'unsubscribe');
    component.savedPlacesService = component.savedPlacesService || {};
    component.savedPlacesService.navigateToPlace = {
      next: function() {}
    };
    component.MapSubscription = component.MapSubscription || {};
    spyOn(component.MapSubscription, 'unsubscribe');
    component.ngOnDestroy();
    // expect(component.navigateToPlaceSubscription.unsubscribe).toHaveBeenCalled();
    // expect(component.MapSubscription.unsubscribe).toHaveBeenCalled();
  });

  it('should run #goToReport()', async () => {
    component.router = component.router || {};
    spyOn(component.router, 'navigate');
    component.goToReport();
    // expect(component.router.navigate).toHaveBeenCalled();
  });

  it('should run #isPointInsidePolygon()', async () => {

    component.isPointInsidePolygon({
      coordinates: {
        0: {},
        1: {}
      }
    }, {
      coordinates: {
        0: {}
      }
    });

  });

});
// Error: ERROR this JS code is invalid, "reports.forEach((report)"
// Error: ERROR this JS code is invalid, "data.features.map((feature)"