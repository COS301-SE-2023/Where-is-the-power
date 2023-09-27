@@ -1, 99 + 0, 0 @@
// Import necessary modules and dependencies for testing
import { ComponentFixture, TestBed, fakeAsync, tick } from '@angular/core/testing';
import { ReportPage } from './report.page';
import { ReportService } from './report.service';
import { AuthService } from '../authentication/auth.service';
import { Router } from '@angular/router';
import { of } from 'rxjs';

// Create mock services
const mockReportService = {
  reportIssue: (reportType: string) => {
    // Simulate a successful response with a status and a message
    return of({ status: 'success', message: ' Report submitted successfully' });
  },
};


const mockAuthService = {
  isUserLoggedIn: () => Promise.resolve(true), // Change to false for testing non-logged-in state
};

const mockRouter = {
  navigate: jasmine.createSpy('navigate'),
};

describe('ReportPage', () => {
  let component: ReportPage;
  let fixture: ComponentFixture<ReportPage>;
  let authService: AuthService;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ReportPage],
      providers: [
        { provide: ReportService, useValue: mockReportService },
        { provide: AuthService, useValue: mockAuthService },
        { provide: Router, useValue: mockRouter },
      ],
    }).compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(ReportPage);
    component = fixture.componentInstance;
    authService = TestBed.inject(AuthService); // Inject AuthService

    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should set "isLoggedIn" to true when the user is logged in', async () => {
    await component.ionViewWillEnter();
    expect(component.isLoggedIn).toBe(true);
  });

  it('should navigate to "/tabs/tab-navigate" after reporting an issue', () => {
    const reportType = 'SubstationBlew';
    component.report(reportType);
    expect(mockRouter.navigate).toHaveBeenCalledWith(['/tabs/tab-navigate']);
  });

  it('should navigate to "/tabs/tab-profile" when "Go to Profile" button is clicked for non-logged-in users', () => {
    authService.isUserLoggedIn = () => Promise.resolve(false); // Mock non-logged-in state
    fixture.detectChanges();
    const navigateSpy = spyOn(component['router'], 'navigate'); // Access private router property
    const button = fixture.nativeElement.querySelector('ion-button');
    button.click();
    fixture.detectChanges();
    expect(navigateSpy).toHaveBeenCalledWith(['/tabs/tab-profile']);
  });

  it('should display the "Reporting is only available to registered users." message for non-logged-in users', () => {
    authService.isUserLoggedIn = () => Promise.resolve(false); // Mock non-logged-in state
    fixture.detectChanges();
    const message = fixture.nativeElement.querySelector('h3');
    expect(message.textContent).toContain('Reporting is only available to registered users.');
  });

  it('should display report options for logged-in users', () => {
    authService.isUserLoggedIn = () => Promise.resolve(true); // Mock logged-in state
    fixture.detectChanges();
    const reportOptions = fixture.nativeElement.querySelectorAll('ion-col');
    expect(reportOptions.length).toBeGreaterThan(0);
  });

  it('should report "SubstationBlew" when the corresponding card is clicked', fakeAsync(() => {
    authService.isUserLoggedIn = () => Promise.resolve(true); // Mock logged-in state
    fixture.detectChanges();
    const reportType = 'SubstationBlew';
    const reportSpy = spyOn(mockReportService, 'reportIssue').and.returnValue(of({}));
    const card = fixture.nativeElement.querySelector('ion-card');
    card.click();
    tick();
    expect(reportSpy).toHaveBeenCalledWith(reportType);
  }));
});
