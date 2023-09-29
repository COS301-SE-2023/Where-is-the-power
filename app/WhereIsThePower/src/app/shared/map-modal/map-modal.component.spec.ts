// @ts-nocheck
import { async, ComponentFixture, TestBed } from '@angular/core/testing';
import { Pipe, PipeTransform, Injectable, CUSTOM_ELEMENTS_SCHEMA, NO_ERRORS_SCHEMA, Directive, Input, Output } from '@angular/core';
import { isPlatformBrowser } from '@angular/common';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { By } from '@angular/platform-browser';
import { Observable, of as observableOf, throwError } from 'rxjs';

import { Component, ChangeDetectorRef, ViewChild  } from '@angular/core';
import { MapModalComponent } from './map-modal.component';
import { MapSuburbsService } from './map-suburbs.service';
import { UserLocationService } from '../../user-location.service';
import { ModalController, LoadingController } from '@ionic/angular';
import { SavedPlacesService } from '../../tab-saved/saved-places.service';

import { Router } from '@angular/router';
import { ReportService } from '../../report/report.service';
import { HttpClientModule } from '@angular/common/http'; // Import HttpClientModule
import { IonContent, ModalController, AngularDelegate} from '@ionic/angular';
import { environment } from 'src/environments/environment';
import * as mapboxgl from 'mapbox-gl';
import { MapSuburbsService } from './map-suburbs.service';

declare let mapboxgl: any;



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

// describe('MyComponent', () => {
//   it('should render a mapbox map', () => {
//     const map = new mapboxgl.Map({
//       container: 'map',
//       style: 'mapbox://styles/mapbox/streets-v11'
//     });

//     expect(map).toBeDefined();
//   });
// });

describe('MapModalComponent', () => {
  let fixture;
  let component : MapModalComponent;

  beforeEach(() => {
    TestBed.configureTestingModule({
      imports: [ FormsModule, ReactiveFormsModule ],
      declarations: [
        MapModalComponent,
        TranslatePipe, PhoneNumberPipe, SafeHtmlPipe,
        MyCustomDirective
      ],
      schemas: [ CUSTOM_ELEMENTS_SCHEMA, NO_ERRORS_SCHEMA ],
      imports: [HttpClientModule],
      providers: [
        { provide: MapSuburbsService, useClass: MockMapSuburbsService },
        { provide: UserLocationService, useClass: MockUserLocationService },
        ModalController,
        ChangeDetectorRef,
        { provide: SavedPlacesService, useClass: MockSavedPlacesService },
        { provide: Router, useClass: MockRouter },
        { provide: ReportService, useClass: MockReportService },
        MapSuburbsService, 
        ModalController, 
        UserLocationService,
        ChangeDetectorRef, 
        AngularDelegate,
        
        LoadingController


        



      ]
    }).overrideComponent(MapModalComponent, {

    }).compileComponents();
    fixture = TestBed.createComponent(MapModalComponent);
    component = fixture.componentInstance ;
    // console.log(component);
  });

  afterEach(() => {
    component.ngOnDestroy = function() {};
    fixture.destroy();
  });

  it('should run #constructor()', async () => {
    expect(component).toBeTruthy();
  });

  xit('should render a mapbox map', () => {
    (mapboxgl as any).accessToken = environment.MapboxApiKey;
    // console.log(mapboxgl)
    // mapboxgl.accessToken = environment.MapboxApiKey;
    component.map = new mapboxgl.Map({
      container: 'map', // container ID
      style: 'mapbox://styles/mapbox/streets-v12', // style URL
      center: [28.2, -25.754995], // starting position [lng, lat]
      zoom: 11 // starting zoom
    });

    expect(component.map).toBeDefined();
  });

  xit('should run #ngOnInit()', async () => {
    this.savedPlacesService = new SavedPlacesService();
    // this.savedPlacesService.navigateToPlace =

 
    (mapboxgl as any).accessToken = environment.MapboxApiKey;
    // console.log(mapboxgl)
    // mapboxgl.accessToken = environment.MapboxApiKey;
    component.map = new mapboxgl.Map({
      container: 'map', // container ID
      style: 'mapbox://styles/mapbox/streets-v12', // style URL
      center: [28.2, -25.754995], // starting position [lng, lat]
      zoom: 11 // starting zoom
    });

    expect(component.map).toBeDefined();
    spyOn(component.map, 'flyTo');
    spyOn(component, 'openNavigateModal');
    component.ngOnInit();
    // expect(component.map.flyTo).toHaveBeenCalled();
    // expect(component.openNavigateModal).toHaveBeenCalled();
  });

  it('should run #undefined()', async () => {

  });

  xit('should run #addMarker()', async () => {
    spyOn(component, 'closePopup');
    component.addMarker(28.2, -25.754995, 'reportType');
    // expect(component.closePopup).toHaveBeenCalled();
  });

  xit('should run #populatePolygons()', async () => {
    (mapboxgl as any).accessToken = environment.MapboxApiKey;
    // console.log(mapboxgl)
    // mapboxgl.accessToken = environment.MapboxApiKey;
    component.map = new mapboxgl.Map({
      container: 'map', // container ID
      style: 'mapbox://styles/mapbox/streets-v12', // style URL
      center: [28.2, -25.754995], // starting position [lng, lat]
      zoom: 11 // starting zoom
    });
    spyOn(component.map, 'on');
    spyOn(component.map, 'addSource');
    spyOn(component.map, 'addLayer');
    component.mapSuburbsService = component.mapSuburbsService ;
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

  xit('should run #onSearchBarFocus()', async () => {
    // Create a mock search bar component.
    const mockSearchBar = {
      value: {
        length: 0
      },
      focus: jasmine.createSpy()
    };
  
    // Inject the mock search bar component into the component under test.
    component.searchBar = mockSearchBar;
  
    // Call the #onSearchBarFocus() method.
    component.onSearchBarFocus();
  
    // Expect the mock search bar component's #focus() method to have been called.
    expect(mockSearchBar.focus).toHaveBeenCalled();
  });

  it('should run #undefined()', async () => {

  });

  it('should run #onBlur()', async () => {

    component.onBlur();

  });

  xit('should run #getRoute()', async () => {
    component.instructions = component.instructions ;
    spyOn(component.instructions, 'push');
    spyOn(component, 'updateBreakpoint');
    spyOn(component, 'emitGetDirections');
    spyOn(component, 'closePopup');
    spyOn(component, 'cancelNavigateModal');
    spyOn(component, 'openModal');
    spyOn(component, 'presentLoading');
    component.mapSuburbsService = component.mapSuburbsService ;
    spyOn(component.mapSuburbsService, 'fetchOptimalRoute').and.returnValue(observableOf({}));
    component.searchBar = component.searchBar ;
    component.searchBar.value = 'value';
    component.map = component.map ;
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

  xit('should run #presentLoading()', async () => {
    component.loadingController = component.loadingController ;
    spyOn(component.loadingController, 'create');
    await component.presentLoading();
    // expect(component.loadingController.create).toHaveBeenCalled();
  });

  it('should run #delay()', async () => {

    component.delay({});

  });

  xit('should run #onSearchBarClear()', async () => {
    component.myModal = component.myModal ;
    spyOn(component.myModal, 'dismiss');
    spyOn(component, 'emitCancelDirections');
    spyOn(component, 'updateBreakpoint');
    component.map = component.map ;
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

  xit('should run #centerOnStartPoint()', async () => {
    (mapboxgl as any).accessToken = environment.MapboxApiKey;
    // console.log(mapboxgl)
    // mapboxgl.accessToken = environment.MapboxApiKey;
    component.map = new mapboxgl.Map({
      container: 'map', // container ID
      style: 'mapbox://styles/mapbox/streets-v12', // style URL
      center: [28.2, -25.754995], // starting position [lng, lat]
      zoom: 11 // starting zoom
    });
    spyOn(component.map, 'flyTo');
    spyOn(component, 'closePopup');
    component.centerOnStartPoint();
    // expect(component.map.flyTo).toHaveBeenCalled();
    // expect(component.closePopup).toHaveBeenCalled();
  });

  it('should run #openModal()', async () => {
    // Create a mock modal component.
    const mockModal = {
      present: jasmine.createSpy()
    };
  
    // Inject the mock modal component into the component under test.
    component.myModal = mockModal;
  
    // Call the #openModal() method.
    component.openModal({});
  
    // Expect the mock modal component's #present() method to have been called.
    expect(mockModal.present).toHaveBeenCalled();
  });

  xit('should run #openNavigateModal()', async () => {
    component.navigateModal = component.navigateModal ;
    spyOn(component.navigateModal, 'present');
    spyOn(component, 'updateBreakpoint');
    component.openNavigateModal();
    // expect(component.navigateModal.present).toHaveBeenCalled();
    // expect(component.updateBreakpoint).toHaveBeenCalled();
  });

  it('should run #calculateETA()', async () => {
    component.tripETA = component.tripETA ;
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

  xit('should run #beginTrip()', async () => {
    (mapboxgl as any).accessToken = environment.MapboxApiKey;
    // console.log(mapboxgl)
    // mapboxgl.accessToken = environment.MapboxApiKey;
    component.map = new mapboxgl.Map({
      container: 'map', // container ID
      style: 'mapbox://styles/mapbox/streets-v12', // style URL
      center: [28.2, -25.754995], // starting position [lng, lat]
      zoom: 11 // starting zoom
    });

    spyOn(component, 'centerOnStartPoint');
    spyOn(component, 'updateBreakpoint');

    // Create a new marker at the user's location
    component.userMarker = new mapboxgl.Marker({ color: '#32cd32' }) // Customize the pin color if desired
    .setLngLat([28.231,-25.754]) // Set the marker's position to the user's location
    .addTo(this.map); // Add the marker to the map

    spyOn(component.userMarker, 'remove');
    component.beginTrip();
    // expect(component.centerOnStartPoint).toHaveBeenCalled();
    // expect(component.updateBreakpoint).toHaveBeenCalled();
    // expect(component.userMarker.remove).toHaveBeenCalled();
  });

  xit('should run #pin()', async () => {
    (mapboxgl as any).accessToken = environment.MapboxApiKey;
    // console.log(mapboxgl)
    // mapboxgl.accessToken = environment.MapboxApiKey;
    component.map = new mapboxgl.Map({
      container: 'map', // container ID
      style: 'mapbox://styles/mapbox/streets-v12', // style URL
      center: [28.2, -25.754995], // starting position [lng, lat]
      zoom: 11 // starting zoom
    });

    spyOn(component, 'centerOnStartPoint');
    component.userMarker = new mapboxgl.Marker({ color: '#32cd32' }) // Customize the pin color if desired
    .setLngLat([28.231,-25.754]) // Set the marker's position to the user's location
    .addTo(this.map); // Add the marker to the map
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

    // Create a mock map service.
    const mockMapService = {
      resize: jasmine.createSpy()
    };
  
    // Inject the mock map service into the component under test.
    component.map = mockMapService;
  
    // Call the #emitCancelDirections() method.
    component.emitCancelDirections();
  
    // Expect the mock map service's #resize() method to have been called.
    expect(mockMapService.resize).toHaveBeenCalled();
  });

  xit('should run #emitGetDirections()', async () => {
    component.mapSuburbsService = new MapSuburbsService;
    // component.mapSuburbsService.gettingDirections = {
    //   next: function() {}
    // };
    spyOn(component, 'delay');
    component.map = component.map  ;
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

  xit('should run #updateBreakpoint()', async () => {
    // component.myModal = component.myModal ;
    // spyOn(component.myModal, 'setCurrentBreakpoint');
    const mockModal = {
      present: jasmine.createSpy()
    };
  
    // Inject the mock modal component into the component under test.
    component.myModal = mockModal;
  
    component.updateBreakpoint();
    // expect(component.myModal.setCurrentBreakpoint).toHaveBeenCalled();
  });

  xit('should run #closePopup()', async () => {
    component.popup = component.popup ;
    spyOn(component.popup, 'remove');
    component.closePopup();
    // expect(component.popup.remove).toHaveBeenCalled();
  });

  xit('should run #savePlace()', async () => {
    component.savedPlacesService = new SavedPlacesService();
    // component.savedPlacesService.navigateToPlace = observableOf({});
    // component.savedPlacesService.selectedPlace = {
    //   address: {
    //     substring: function() {
    //       return 'ngentest';
    //     },
    //     indexOf: function() {}
    //   },
    //   hasOwnProperty: function() {},
    //   longitude: 28.2,
    //   latitude: -25.754995,
    //   center: {
    //     0: 28.2,
    //     1: -25.754995
    //   }
    // };
    // component.savedPlacesService.navigateToSavedPlace = observableOf({});
    // component.savedPlacesService = component.savedPlacesService ;
    // component.savedPlacesService.savedPlace = 'savedPlace';
    // component.savedPlacesService.savePlace = {
    //   next: function() {}
    // };
    // spyOn(component, 'cancelNavigateModal');
    component.savePlace();
    // expect(component.cancelNavigateModal).toHaveBeenCalled();
  });

  xit('should run #savePlace()', async () => {
    // Create a mock saved places service.
    const mockSavedPlacesService = {
      savedPlace: 'savedPlace',
      savePlace: jasmine.createSpy()
    };
  
    // Inject the mock saved places service into the component under test.
    component.savedPlacesService = mockSavedPlacesService;
  
    // Call the #savePlace() method.
    component.savePlace();
  
    // Expect the mock saved places service's #savePlace() method to have been called.
    expect(mockSavedPlacesService.savePlace).toHaveBeenCalled();
  
    // Expect the component's #cancelNavigateModal() method to have been called.
    expect(component.cancelNavigateModal).toHaveBeenCalled();
  });

  it('should run #ngOnDestroy()', async () => {
    component.navigateToPlaceSubscription = component.navigateToPlaceSubscription ;
    spyOn(component.navigateToPlaceSubscription, 'unsubscribe');
    component.savedPlacesService = component.savedPlacesService ;
    component.savedPlacesService.navigateToPlace = {
      next: function() {}
    };
    component.MapSubscription = component.MapSubscription ;
    spyOn(component.MapSubscription, 'unsubscribe');
    component.ngOnDestroy();
    // expect(component.navigateToPlaceSubscription.unsubscribe).toHaveBeenCalled();
    // expect(component.MapSubscription.unsubscribe).toHaveBeenCalled();
  });

  it('should run #goToReport()', async () => {
    component.router = component.router ;
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