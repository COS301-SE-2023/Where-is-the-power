import { TestBed, inject, async } from '@angular/core/testing';
import { HttpClientTestingModule, HttpTestingController } from '@angular/common/http/testing';
import { AuthService } from './auth.service';
import { RegisterUser } from '../shared/models/register-user';
import { User } from '../shared/models/user';

describe('AuthService', () => {
  let authService: AuthService;
  let httpMock: HttpTestingController;

  beforeEach(() => {
    TestBed.configureTestingModule({
      imports: [HttpClientTestingModule],
      providers: [AuthService],
    });

    authService = TestBed.inject(AuthService);
    httpMock = TestBed.inject(HttpTestingController);
  });

  it('should be created', () => {
    expect(authService).toBeTruthy();
  });

  it('should sign up a user', () => {
    const registerUser: RegisterUser = {
        firstName: 'andy',
        lastName: 'vis',
        email: 'andy@gmail.com',
        password: 'Helloworld123',
    };

    authService.signupUser(registerUser).subscribe((response) => {
    });

    const req = httpMock.expectOne(`${authService.apiUrl}user`);
    expect(req.request.method).toBe('POST');
    req.flush({  });
  });

  it('should log in a user', () => {
    const user: User = {
        authType: '',
        email: '',
        password: '',
        firstName: '',
        lastName: ''
    };

    authService.loginUser(user).subscribe((response) => {
    });

    const req = httpMock.expectOne(`${authService.apiUrl}auth`);
    expect(req.request.method).toBe('POST');
    req.flush({ token: 'sample_token' }); 
  });

  it('should save user data', async(() => {
    authService.saveUserData('Token', 'sample_token').then(() => {
    });
  }));

  it('should sign out a user', async(() => {
    authService.signOutUser();
  }));

  it('should get user data', async(() => {
    authService.getUserData().then((userData) => {
    });
  }));

  it('should check if a user is logged in', async(() => {
    authService.isUserLoggedIn().then((loggedIn) => {
    });
  }));

  it('should get authentication headers', () => {
    const headers = authService.getAuthHeaders();
  });

  afterEach(() => {
    httpMock.verify();
  });
});
