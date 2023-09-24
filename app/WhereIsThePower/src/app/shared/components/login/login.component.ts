import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { Router } from '@angular/router';
import { ToastController } from '@ionic/angular';
import { User } from '../../models/user';
import { AuthService } from '../../../authentication/auth.service';
import { ModalController } from '@ionic/angular';
import { LoadingController } from '@ionic/angular';

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
    firstName: "",
    lastName: "",
  };


  loginForm: FormGroup = this.formBuilder.group({
    email: ['', [Validators.required, Validators.email]],
    password: ['', [Validators.required]],
  });

  constructor(
    private router: Router,
    private formBuilder: FormBuilder,
    private toastController: ToastController,
    private authService: AuthService,
    public modalController: ModalController,
    private loadingController: LoadingController
  ) { }


  ngOnInit() { }

  dismissModal() {
    this.modalController.dismiss();
  }

  async login() {
    if (this.loginForm.valid) {
      this.User.authType = "User";
      this.User.email = this.loginForm.value.email;
      this.User.password = this.loginForm.value.password;
      const loading = await this.presentLoading(); // Show loading spinner

      console.log(this.User)
      this.authService.loginUser(this.User).subscribe(async (response: any) => {
        if (response.token) {
          this.dismissModal();
          this.User.token = response.token;
          this.User.firstName = response.firstName;
          this.User.lastName = response.lastName;
          this.authService.user.next(this.User);
          await this.authService.saveUserData('Token', JSON.stringify(this.User.token));
          //this.sucessToast('Welcome back ' + this.User.firstName)
          //const userData = await this.authService.getUserData();
          //console.log("TOKEN " + userData);
        }
        else {
          this.failToast('Please ensure all details are correct');
        }
        loading.dismiss(); // Dismiss loading spinner when response is received
      });
    } else {
      this.failToast('Please ensure all details are correct');
    }
  }

  async failToast(message: string) {
    const toast = await this.toastController.create({
      message: message,
      color: 'danger',
      duration: 3000,
      position: 'bottom',
    });
    toast.present();
  }

  async sucessToast(message: string) {
    const toast = await this.toastController.create({
      message: message,
      color: 'success',
      duration: 2000,
      position: 'top',
    });
    toast.present();
  }

  private async presentLoading() {
    const loading = await this.loadingController.create({
      message: 'Logging in...',
      spinner: 'crescent', // spinner style
      duration: 20000,
    });
    await loading.present();
    return loading;
  }
}
