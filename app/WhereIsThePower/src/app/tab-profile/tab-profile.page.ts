import { Component, OnInit } from '@angular/core';
import { RegisterUser } from '../shared/models/register-user';
import { AuthService } from '../authentication/auth.service';
@Component({
  selector: 'app-tab-profile',
  templateUrl: './tab-profile.page.html',
  styleUrls: ['./tab-profile.page.scss'],
})
export class TabProfilePage implements OnInit {
  newUser: RegisterUser = {
    firstName: "Jill",
    lastName: "Moore",
    email: "jill@gmail.com",
    password: "Password!123"
  };

  constructor(private authService: AuthService) { }

  ngOnInit() {
    this.RegisterUser();
  }

  RegisterUser() {
    console.log(this.newUser)
    this.authService.signupUser(this.newUser).subscribe((response: any) => {
      console.log(response);
    });
  }
}
