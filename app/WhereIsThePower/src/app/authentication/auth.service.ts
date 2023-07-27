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
  place = {
    "address": "20 11 Street",
    "latitude": 0,
    "longitude": 0,
    "mapboxId": "pk.eyJ1IjoidTE4MDA0ODc0IiwiYSI6ImNsajMzdWh5ZzAwcHAzZXMxc3lveXJmNDgifQ.7P_tuuiC4M_Q1_H5ZF1rTA",
    "name": "Home"
  }

  headers = {
    "Authorization": "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdXRoVHlwZSI6IlVzZXIiLCJlbWFpbCI6IkRhcmNpZUBnbWFpbC5jb20iLCJleHAiOjE2OTA0NzMxMjJ9.PaM16wcLPjmLLpT6O7-6dZSXoBTPl86GZ6ufTaEYp50cL2Vkb2myKhRAG8VSzP7W2GUi6g_nHZcl7zZuqexrMmfiN4hzpw_DuaVe5DPn5myEWlJgkw12_aONJOuJ2L9a8JmE9C9m5IGjS-Ek0l3hQkuHTN90mdo1hsdNhjIUkmg8pfDvHErkQ1C9yPHSaH3wsDt7GZkyG0uqQDU-uYJYmFRnlf1dE8yACB_9j1hSSk8Lc1cY9oxjPXTg7VxQAFBVbTF-A19hUgFITq139YLEzQ3s39ARpc4EAKfY3WrAlxTvy-5b4X50xu9gOAulgPmKZNsYSltRvNNo-KWU62wiwQTFlOWKqK74DrpCN-YA0YyHHFMaGWdGn-fqnezZ8_cA7Tp_5-ac6WpVfellhLe8sQSqo6o28eLrnvwaxLEX1wB4eTubIsm-0mzycq49mtuieLSk5vKi4dp44fqka2_zrkXhYeNo3Ja2Uk8-uT1FWhMhRmClNEDXQoeNsIg-zJRjq-oL8Ujfwuza_84zXT6IjVE8mIqW88Oq6Aqa4UH7SbWGrbN5ZwqYjCiu96Eb-mfeZo3qFDEoJbtb7E2dbZeoRidtbwgMoP5aL-e_UdJ685PDlEOvi2esKjhJdB5cjAAhGjHPUeLsQhqSE6N81Aml1o_CJFV5xrWQrhBFYRsJdPo"
  }


  constructor(private httpClient: HttpClient) { }

  signupUser(registerUser: RegisterUser) {
    return this.httpClient.post(`${this.apiUrl}user`, registerUser)
  }

  loginUser(user: User) {
    this.isLoggedin = true;
    return this.httpClient.post(`${this.apiUrl}auth`, user)
  }

  addSavedPlace() {
    console.log("add saved place")
    return this.httpClient.put(`${this.apiUrl}user/add_saved_place`, this.place, { headers: this.headers })
  }

  async saveUserData(key: string, value: any) {
    Preferences.set({ key: key, value: value });
  }

  async signOutUser() {
    this.isLoggedin = false;
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
}
