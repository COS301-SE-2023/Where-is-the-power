import { Component, OnInit } from '@angular/core';
import { ModalController } from '@ionic/angular';
import { RegisterUser } from '../shared/models/register-user';
import { AuthService } from '../authentication/auth.service';
import { User } from '../shared/models/user';
import { LoginComponent } from '../shared/components/login/login.component';
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

  constructor(private authService: AuthService, private modalController: ModalController) { }


  ngOnInit() {

  }

  RegisterUser() {
    console.log(this.newUser)
    this.authService.signupUser(this.newUser).subscribe((response: any) => {
      console.log(response);
    });
  }

  async showLoginComponent() {
    const modal = await this.modalController.create({
      component: LoginComponent,
      // You can pass data to the login component using componentProps if needed
      // componentProps: { data: yourData },
    });
    return await modal.present();
  }

  closeModal() {
    this.modalController.dismiss();
  }

}
