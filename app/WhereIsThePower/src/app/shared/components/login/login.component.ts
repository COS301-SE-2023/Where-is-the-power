import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { Router } from '@angular/router';
import { ToastController } from '@ionic/angular';
import { User } from '../../models/user';
import { AuthService } from '../../../authentication/auth.service';
import { ModalController } from '@ionic/angular';

@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.scss'],
})
export class LoginComponent implements OnInit {

  User: User = {
    authType: "",
    email: "",
    password: "",
  };


  loginForm: FormGroup = this.formBuilder.group({
    email: ['', [Validators.required, Validators.email]],
    password: ['', [Validators.required, Validators.minLength(8)]],
  });

  constructor(
    private router: Router,
    private formBuilder: FormBuilder,
    private toastController: ToastController,
    private authService: AuthService,
    public modalController: ModalController
  ) { }


  ngOnInit() { }

  dismissModal() {
    this.modalController.dismiss();
  }

  login() {
    if (this.loginForm.valid) {
      this.User.authType = "User";
      this.User.email = this.loginForm.value.email;
      this.User.password = this.loginForm.value.password;

      console.log(this.User)
      this.authService.loginUser(this.User).subscribe(async (response: any) => {
        console.log(response);
        this.dismissModal();
        this.User.token = response.token;
        await this.authService.saveUserData('Token', JSON.stringify(this.User.token));

        //const userData = await this.authService.getUserData();
        //console.log("TOKEN " + userData);
      });
    } else {
      this.presentToast('Please enter a valid email and password.');
    }
  }

  async presentToast(message: string) {
    const toast = await this.toastController.create({
      message: message,
      duration: 3000,
      position: 'bottom',
    });
    toast.present();
  }
}
