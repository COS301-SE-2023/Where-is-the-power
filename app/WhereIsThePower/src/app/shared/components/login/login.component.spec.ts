// @ts-nocheck
import { async, ComponentFixture, TestBed } from '@angular/core/testing';
import { Pipe, PipeTransform, Injectable, CUSTOM_ELEMENTS_SCHEMA, NO_ERRORS_SCHEMA, Directive, Input, Output } from '@angular/core';
import { isPlatformBrowser } from '@angular/common';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { By } from '@angular/platform-browser';
import { Observable, of as observableOf, throwError } from 'rxjs';

import { Component } from '@angular/core';
import { LoginComponent } from './login.component';
import { Router } from '@angular/router';
import { FormBuilder } from '@angular/forms';
import { ToastController, ModalController, LoadingController } from '@ionic/angular';
import { AuthService } from '../../../authentication/auth.service';

@Injectable()
class MockRouter {
  navigate() {};
}

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

describe('LoginComponent', () => {
  let fixture;
  let component;

  beforeEach(() => {
    TestBed.configureTestingModule({
      imports: [ FormsModule, ReactiveFormsModule ],
      declarations: [
        LoginComponent,
        TranslatePipe, PhoneNumberPipe, SafeHtmlPipe,
        MyCustomDirective
      ],
      schemas: [ CUSTOM_ELEMENTS_SCHEMA, NO_ERRORS_SCHEMA ],
      providers: [
        { provide: Router, useClass: MockRouter },
        FormBuilder,
        ToastController,
        { provide: AuthService, useClass: MockAuthService },
        ModalController,
        LoadingController
      ]
    }).overrideComponent(LoginComponent, {

    }).compileComponents();
    fixture = TestBed.createComponent(LoginComponent);
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

  it('should run #dismissModal()', async () => {
    component.modalController = component.modalController || {};
    component.modalController.dismiss = jest.fn();
    component.dismissModal();
    // expect(component.modalController.dismiss).toHaveBeenCalled();
  });

  it('should run #login()', async () => {
    component.loginForm = component.loginForm || {};
    component.loginForm.valid = 'valid';
    component.loginForm.value = {
      email: {},
      password: {}
    };
    component.User = component.User || {};
    component.User.authType = 'authType';
    component.User.email = 'email';
    component.User.password = 'password';
    component.User.token = 'token';
    component.User.firstName = 'firstName';
    component.User.lastName = 'lastName';
    component.presentLoading = jest.fn();
    component.authService = component.authService || {};
    component.authService.loginUser = jest.fn().mockReturnValue(observableOf({
      token: {},
      firstName: {},
      lastName: {}
    }));
    component.authService.user = {
      next: function() {}
    };
    component.authService.saveUserData = jest.fn();
    component.dismissModal = jest.fn();
    component.failToast = jest.fn();
    await component.login();
    // expect(component.presentLoading).toHaveBeenCalled();
    // expect(component.authService.loginUser).toHaveBeenCalled();
    // expect(component.authService.saveUserData).toHaveBeenCalled();
    // expect(component.dismissModal).toHaveBeenCalled();
    // expect(component.failToast).toHaveBeenCalled();
  });

  it('should run #failToast()', async () => {
    component.toastController = component.toastController || {};
    component.toastController.create = jest.fn();
    await component.failToast({});
    // expect(component.toastController.create).toHaveBeenCalled();
  });

  it('should run #sucessToast()', async () => {
    component.toastController = component.toastController || {};
    component.toastController.create = jest.fn();
    await component.sucessToast({});
    // expect(component.toastController.create).toHaveBeenCalled();
  });

  it('should run #presentLoading()', async () => {
    component.loadingController = component.loadingController || {};
    component.loadingController.create = jest.fn();
    await component.presentLoading();
    // expect(component.loadingController.create).toHaveBeenCalled();
  });

});