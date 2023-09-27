// @ts-nocheck
import { async, ComponentFixture, TestBed } from '@angular/core/testing';
import { Pipe, PipeTransform, Injectable, CUSTOM_ELEMENTS_SCHEMA, NO_ERRORS_SCHEMA, Directive, Input, Output } from '@angular/core';
import { isPlatformBrowser } from '@angular/common';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { By } from '@angular/platform-browser';
import { Observable, of as observableOf, throwError } from 'rxjs';

import { Component } from '@angular/core';
import { ReportPage } from './report.page';
import { ReportService } from './report.service';
import { Router } from '@angular/router';
import { AuthService } from '../authentication/auth.service';

@Injectable()
class MockReportService {}

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

describe('ReportPage', () => {
  let fixture;
  let component;

  beforeEach(() => {
    TestBed.configureTestingModule({
      imports: [ FormsModule, ReactiveFormsModule ],
      declarations: [
        ReportPage,
        TranslatePipe, PhoneNumberPipe, SafeHtmlPipe,
        MyCustomDirective
      ],
      schemas: [ CUSTOM_ELEMENTS_SCHEMA, NO_ERRORS_SCHEMA ],
      providers: [
        { provide: ReportService, useClass: MockReportService },
        { provide: Router, useClass: MockRouter },
        { provide: AuthService, useClass: MockAuthService }
      ]
    }).overrideComponent(ReportPage, {

    }).compileComponents();
    fixture = TestBed.createComponent(ReportPage);
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

  it('should run #ionViewWillEnter()', async () => {
    component.authService = component.authService || {};
    component.authService.isUserLoggedIn = jest.fn();
    await component.ionViewWillEnter();
    // expect(component.authService.isUserLoggedIn).toHaveBeenCalled();
  });

  it('should run #report()', async () => {
    component.reportService = component.reportService || {};
    component.reportService.reportIssue = jest.fn().mockReturnValue(observableOf({}));
    component.router = component.router || {};
    component.router.navigate = jest.fn();
    component.report({});
    // expect(component.reportService.reportIssue).toHaveBeenCalled();
    // expect(component.router.navigate).toHaveBeenCalled();
  });

  it('should run #ngOnDestroy()', async () => {
    component.createReportSubscription = component.createReportSubscription || {};
    component.createReportSubscription.unsubscribe = jest.fn();
    component.ngOnDestroy();
    // expect(component.createReportSubscription.unsubscribe).toHaveBeenCalled();
  });

});