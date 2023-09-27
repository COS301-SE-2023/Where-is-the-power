// @ts-nocheck
import { async, ComponentFixture, TestBed } from '@angular/core/testing';
import { Pipe, PipeTransform, Injectable, CUSTOM_ELEMENTS_SCHEMA, NO_ERRORS_SCHEMA, Directive, Input, Output } from '@angular/core';
import { isPlatformBrowser } from '@angular/common';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { By } from '@angular/platform-browser';
import { Observable, of as observableOf, throwError } from 'rxjs';

import { Component } from '@angular/core';
import { TabProfilePage } from './tab-profile.page';
import { AuthService } from '../authentication/auth.service';
import { ModalController } from '@ionic/angular';

@Injectable()
class MockAuthService {}

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

describe('TabProfilePage', () => {
  let fixture;
  let component;

  beforeEach(() => {
    TestBed.configureTestingModule({
      imports: [ FormsModule, ReactiveFormsModule ],
      declarations: [
        TabProfilePage,
        TranslatePipe, PhoneNumberPipe, SafeHtmlPipe,
        MyCustomDirective
      ],
      schemas: [ CUSTOM_ELEMENTS_SCHEMA, NO_ERRORS_SCHEMA ],
      providers: [
        { provide: AuthService, useClass: MockAuthService },
        ModalController
      ]
    }).overrideComponent(TabProfilePage, {

    }).compileComponents();
    fixture = TestBed.createComponent(TabProfilePage);
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
    component.authService = component.authService || {};
    component.authService.user = observableOf({
      firstName: {
        charAt: function() {}
      }
    });
    component.authService.isLoggedin = 'isLoggedin';
    spyOn(component, 'getInitialDataURL');
    component.ngOnInit();
    // expect(component.getInitialDataURL).toHaveBeenCalled();
  });

  it('should run #showSignupComponent()', async () => {
    component.modalController = component.modalController || {};
    spyOn(component.modalController, 'create');
    await component.showSignupComponent();
    // expect(component.modalController.create).toHaveBeenCalled();
  });

  it('should run #showLoginComponent()', async () => {
    component.modalController = component.modalController || {};
    spyOn(component.modalController, 'create');
    await component.showLoginComponent();
    // expect(component.modalController.create).toHaveBeenCalled();
  });

  it('should run #logout()', async () => {
    component.authService = component.authService || {};
    spyOn(component.authService, 'signOutUser');
    spyOn(component, 'toggleTheme');
    await component.logout();
    // expect(component.authService.signOutUser).toHaveBeenCalled();
    // expect(component.toggleTheme).toHaveBeenCalled();
  });

  it('should run #ngOnDestroy()', async () => {
    component.userSubscription = component.userSubscription || {};
    spyOn(component.userSubscription, 'unsubscribe');
    component.ngOnDestroy();
    // expect(component.userSubscription.unsubscribe).toHaveBeenCalled();
  });

  it('should run #getInitialDataURL()', async () => {

    component.getInitialDataURL({});

  });

  it('should run #toggleTheme()', async () => {

    component.toggleTheme({});

  });

});