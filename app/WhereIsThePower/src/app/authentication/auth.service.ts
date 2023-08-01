import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { RegisterUser } from '../shared/models/register-user';
import { User } from '../shared/models/user';
import { Preferences } from '@capacitor/preferences';
import { BehaviorSubject, Observable } from 'rxjs';
import { tap } from 'rxjs/operators';
import { HttpHeaders } from '@angular/common/http';

@Injectable({
  providedIn: 'root'
})
export class AuthService {
  static saveData(arg0: string, token: string | undefined) {
    throw new Error('Method not implemented.');
  }
  apiUrl = 'https://witpa.codelog.co.za/api/'
  token = null;
  isLoggedin: boolean = false;
  public user = new BehaviorSubject<User | null>(null);

  headers = new HttpHeaders();
  constructor(private httpClient: HttpClient) { }

  signupUser(registerUser: RegisterUser) {
    return this.httpClient.post(`${this.apiUrl}user`, registerUser)
  }

  loginUser(user: User) {
    this.isLoggedin = true;
    return this.httpClient.post(`${this.apiUrl}auth`, user).pipe(
      tap((response: any) => {
        if (response.token) {
          this.token = response.token;
          console.log("RES" + response.token);
          this.headers = this.headers.set('Authorization', `Bearer ${this.token}`);

          // console.log("HTTP" + this.headers.get('Authorization'));
          // this.saveUserData('Token', token);
        }
      }
      ));
  }

  async saveUserData(key: string, value: any) {
    Preferences.set({ key: key, value: value });
  }

  async signOutUser() {
    this.isLoggedin = false;
    // this.token = null;
    this.headers = this.headers.set('Authorization', '');
    Preferences.remove({ key: 'Token' });
  }

  async getUserData() {
    const ret = await Preferences.get({ key: 'Token' });
    if (ret.value) {
      return JSON.parse(ret.value);
    }
    return null;
  }

  async isUserLoggedIn() {
    return this.isLoggedin;
  }

  getAuthHeaders() {
    return this.headers;
  }
}
