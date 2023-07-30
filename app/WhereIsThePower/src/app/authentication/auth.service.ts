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
  place = {
    "address": "20 11 Street",
    "latitude": 0,
    "longitude": 0,
    "mapboxId": "dXJuOm1ieHJldDpDcERqSFJpdzlKcXlVbVlHd0dvaGtzWlFpSWt2Vy1nbHR0YXBwWTZ0QUNoLWI5WVJmbjJicTg5Wi1tMFN3bjJ0MTJxOFdSemw3WFZpXzVmLVhLSVpTLWY1T2tMRlZfWFVoYVhzb09hOTdRMG1mSXZpS09yTkhtR0J5QVg1NVg4cVBFakUwcnhBaUVYWFYyWkFiZXkzVEg1QzdUbGJubW43dnZaYnFsUUdPSi1BNlFuV1Z4U0poQjROdG9ZLXV5aktUREdGRnNwVDY0bFZFdFBMMW1XX3lRSEZTX3AyN1ZFNF9EOWNrdzdKUkRsUWVqb3J1a1JCX2lmWFJFQVNKT3U1dXdlajk5QUdfb1JXd1ZPd1o3MzZDTVZ4RUhrYjU3Qk9DTU05LXE4OENKWVhyM3pDM1lhRDk2eFRKQXBGbGVFQTJuTkp6dzRhU29leHp1OWh6YlotOGxFYldiNW4zYmNhOTZTUzRoOVpuVEtUVTZhTjlocjFVQ2lGaEJBY3lyY19MVzlxYnZQSDFEYm5Obk50QjRTVkNpQkZqdlFFZW9ZOGNoa0NheGJUWXBXVUZGX0FWNWp3NXA1aDRsdURLTmtNUmFKVTZmdm80VEx0VlNNUkkwU2d2VUFPUmcxZ3ZqWTZNVmFjd05FUktjdVBoSHFIei1JeEh4SWMyVVIyck9CeHRFOXY3ODY1ZFlud0J6RWZNNGo3REdxc3owd3NfZFF0ZkJfN1FPRUR3bTNobnF4akJ6QUxDTkhjWUtJWlRtb2RtcWhVMElGdnktTTU2S1llNmxLSXcyUVJodlBwS2tKaVNQRk1FVkVvTWhPTm45OGpqZHNNZ3pWaldLVVh4dWdIQjN2eWdReHk5NjlMNEdaOFJHQ1BUNjVPaG0yVlg3cVNPZ0NlMXlHZ3pJRUhtUEtXZ0p5anZtT0NobjBRRWRNemdsOVBtY2FiZzF6ZFlmTkV1QThCNDJFYzFZUy1wRjQ4c25UN080bDhERkg4RjR6TFdXVHhDQUtyUVhNTlkwZUNuVmNMQ3BYVXltMW82TG9kUFdmYi1MVE5ieXR4aURQRk0wa3JTR1RWM0NDMmNveUdCVW03OVp4c29RTmZLU3NJTFIwcjVYa0sxdXFodzNPaVV4UlBiU2tJVGNYMXAzWU1XS0djRURDUnd2bThmVkJGaUR0MFJTS28yd2UwVGxlM3Q2OW1nX1oxd3ktRkF5ek5kNkdHa0NYeGYxV184cDhESUdhU3RoeUw2WERQNU9fbXFEOXp6STdsTTllVUw3OE9RcDQ5QV9mc2RPb1l2UTctanpqZkpkR0pjZGV3QTVCQkt3VEMzZDFOTFlySWR5cFR0eWdmWGFBQlVvNFJfRWZielJRWUxnR0lkTDV2QTFUcEsxYmgwdEJqblhTLVRHbHJmcWkwSkpEb0hxRUhxek4xYUo3cndiNlJ4WjQ3Uk5KSG9TNlNobDZ2dmtGVnhHUXRjUTNJZjBvdDFOYzZQU2d0VHZKeUFNV2JlY3BjWDV1TU5hV3hCbFI0QlMwZVRZZHRUUWxjT2c5Vk9hdzNRWS0tSlhXZjRXS1lsU2VMUUJhZ19wMXdpRWVVRjdCeUVYUXFnb3Z2VF9VUnVhaHg3VGZfcGZ0UFcwcGd5NWgwSzVIVDdJYjYtbFVseUFCTU9nOVFHY1VMY1pXUjlSOEtDY3dFVGlNZy0tb3NHbEpYMFlPRll1dmtWZ0dtWnVKcjFUTUpXV2tsOWlENS1qTnNueFFXT0x6aWtwQ1NHeG5OX0ZDZ1dYMTJSWG9ELW1LRnItSkZELTZFbzltenZyc1JTVnZHWDFkNTRMLVU2d2lEU1djSFhRYUwyQlJ1OVNhcEdlY1VBVjdncTZPcGhDVlN6ZUV5QUV4SHBXUXhRTkdnWXZUY3liY19ReVJlenFnb0NCYlFEdkphLUpZbElKOW9BYUJRSDBkcG9RRlVQWGNQM1g0NV9WMWx3QmVsdjkyTHhlX21MWXJZT3NYZHRraUhBLTVEMjMxWVJIQzFwVldRSXc3MzZHTnFxRFlmMjNqQXo4M1pqVGxXem1XQmtlRlhCWnRjNHBPNDk1c1AzNkJQVTdTZktYaGttWWNZTW91NHhrZGlTc29jYmdOUGI2RXVDNHNyM3FRM202UTJLb3pFdEhWallRQnppM1llN20xRHdpT291R092aHlscnM1M1hzNXdNZTBYd1NqVTI5VnhEby03b2lsQm9HWjZtNmpJX2Fyell1UFdxM2dnZUFmd24xQ3YyS2VSbENaenpEaHB3UUJUM0xmVnlrSU0wb0RWc3M1VVJObHBKcnJEWkwwbkk0UVlJamU2b1ducFBjaW5uYTZ5OHJ0bXRrRG5GMVRoaGhVYmdLZzBuRjY4cjRxTFpLc1VHbHlOTmxVczBnV2EtY19qZWQyQVZBQUQwSG84Mk1zSW9WMVpFemp0RmdEbXM3WGRNZ25pX3oxMHNVVjhxOVZkLW00UV9Ra2czSE1kWjVQaVlleUluUE1UbzFxY0Fjek91ZUl3YmNPUWlsbVFCZ3lSd3U0djVHNGNwa3NCRVpBRmRTMVhpV1BiRWJxbUZGR0FXellVTEhFTnZtVHZIZ3dmb1VGUkpJc2FmVEVEdzZFejNSM3N4dXdtajJvajU0V1pFWU9VPQ==",
    "name": "Home"
  }

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

  addSavedPlace() {
    console.log("add saved place");
    console.log(this.place);
    console.log(this.headers);

    return this.httpClient.put(`${this.apiUrl}user/savedPlaces`, this.place, { headers: this.headers })
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
}
