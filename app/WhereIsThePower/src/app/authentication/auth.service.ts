import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { RegisterUser } from '../shared/models/register-user';
import { User } from '../shared/models/user';
import { Preferences } from '@capacitor/preferences';
import { BehaviorSubject } from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class AuthService {
  static saveData(arg0: string, token: string | undefined) {
    throw new Error('Method not implemented.');
  }
  apiUrl = 'https://witpa.codelog.co.za/api/'

  isLoggedin: boolean = false;
  public user = new BehaviorSubject<User | null>(null);

  constructor(private httpClient: HttpClient) { }

  signupUser(registerUser: RegisterUser) {
    return this.httpClient.post(`${this.apiUrl}user`, registerUser)
  }

  loginUser(user: User) {
    this.isLoggedin = true;
    this.user.next(user);
    return this.httpClient.post(`${this.apiUrl}auth`, user)
  }

  async saveUserData(key: string, value: any) {
    Preferences.set({ key: key, value: value });
  }

  async getUserData() {
    const ret = await Preferences.get({ key: 'Token' });
    if (ret.value) {
      return JSON.parse(ret.value);
    }
    return null;
  }

  async isUserLoggedIn() {
    if (await this.getUserData() === null) return false;
    return true;
  }
}
