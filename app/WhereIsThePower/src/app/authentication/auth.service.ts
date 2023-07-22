import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { RegisterUser } from '../shared/models/register-user';
import { User } from '../shared/models/user';
@Injectable({
  providedIn: 'root'
})
export class AuthService {
  apiUrl = 'http://witpa.codelog.co.za/api/'

  constructor(private httpClient: HttpClient) { }

  signupUser(registerUser: RegisterUser) {
    return this.httpClient.post(`${this.apiUrl}user`, registerUser)
  }

  loginUser(user: User) {
    return this.httpClient.post(`${this.apiUrl}auth`, user)
  }
}
