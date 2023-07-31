import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { AuthService } from '../authentication/auth.service';
import { BehaviorSubject } from 'rxjs';
import { Place } from './place';

@Injectable({
  providedIn: 'root'
})
export class SavedPlacesService {

  constructor(
    private httpClient: HttpClient,
    private auth: AuthService
    ) { }

  apiUrl = 'https://witpa.codelog.co.za/api/';
  headers = this.auth.getAuthHeaders;

  place = new BehaviorSubject<Place | null>(null);

}
