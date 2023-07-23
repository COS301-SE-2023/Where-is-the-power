import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { Router } from '@angular/router';
import { ToastController } from '@ionic/angular';
import { RegisterUser } from '../../models/register-user';
import { AuthService } from '../../../authentication/auth.service';
import { ModalController } from '@ionic/angular';

@Component({
  selector: 'app-signup',
  templateUrl: './signup.component.html',
  styleUrls: ['./signup.component.scss'],
})
export class SignupComponent implements OnInit {
  newUser: RegisterUser = {
    firstName: "",
    lastName: "",
    email: "",
    password: "",
  }


  signupForm: FormGroup = this.formBuilder.group({
    firstName: ['', [Validators.required]],
    lastName: ['', [Validators.required]],
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

  signup() {
    if (this.signupForm.valid) {
      this.newUser.firstName = this.signupForm.value.firstName;
      this.newUser.lastName = this.signupForm.value.lastName;
      this.newUser.email = this.signupForm.value.email;
      this.newUser.password = this.signupForm.value.password;

      console.log(this.newUser)
      this.authService.signupUser(this.newUser).subscribe(async (response: any) => {
        console.log(response);
        this.dismissModal();
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
