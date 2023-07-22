import { Component, OnInit } from '@angular/core';
import { RegisterUser } from '../shared/models/register-user';
import { AuthService } from '../authentication/auth.service';
import { User } from '../shared/models/user';
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

  User: User = {
    authType: "User",
    email: "jill@gmail.com",
    password: "Password!123"
  };

  constructor(private authService: AuthService) { }

  ngOnInit() {
    this.loginUser();
  }

  RegisterUser() {
    console.log(this.newUser)
    this.authService.signupUser(this.newUser).subscribe((response: any) => {
      console.log(response);
    });
  }

  loginUser() {
    console.log(this.User)
    this.authService.loginUser(this.User).subscribe((response: any) => {
      console.log(response);
    });
  }
}
