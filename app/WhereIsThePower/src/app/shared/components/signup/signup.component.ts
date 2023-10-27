import { Component, OnInit } from '@angular/core';
import { AbstractControl, FormBuilder, FormGroup, Validators } from '@angular/forms';
import { Router } from '@angular/router';
import { ToastController } from '@ionic/angular';
import { RegisterUser } from '../../models/register-user';
import { AuthService } from '../../../authentication/auth.service';
import { ModalController } from '@ionic/angular';
import { User } from '../../models/user';
import { LoadingController } from '@ionic/angular';

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
    confirmPassword: ['', Validators.required],
  });


  constructor(
    private router: Router,
    private formBuilder: FormBuilder,
    private toastController: ToastController,
    private authService: AuthService,
    public modalController: ModalController,
    private loadingController: LoadingController
  ) {
    this.signupForm.addValidators(
      this.comparePasswordValidator(
        this.signupForm.controls['password'],
        this.signupForm.controls['confirmPassword']
      )
    );
   }


    


  ngOnInit() { }

  dismissModal() {
    this.modalController.dismiss();
  }


  comparePasswordValidator(passwordOne: AbstractControl, passwordTwo: AbstractControl) {
      return () => {
      if (passwordOne.value !== passwordTwo.value) {
        console.log("Passwords do not match")
        return { match_error: 'matchingErr' };
      }
        
      return null;
    };
  }

  signup() {
    if(this.signupForm.value.password !== this.signupForm.value.confirmPassword) {
      this.failToast('Passwords do not match');
      return;
    }

    if (this.signupForm.valid) {
      this.newUser.firstName = this.signupForm.value.firstName;
      this.newUser.lastName = this.signupForm.value.lastName;
      this.newUser.email = this.signupForm.value.email;
      this.newUser.password = this.signupForm.value.password;

      console.log(this.newUser)
      this.authService.signupUser(this.newUser).subscribe(async (response: any) => {
        console.log(response);
        let createNewUser = new User("User", this.newUser.email, this.newUser.password, this.newUser.firstName, this.newUser.lastName);
        const loading = await this.presentLoading(); // Show loading spinner

        console.log(createNewUser);
        this.authService.loginUser(createNewUser).subscribe(async (response: any) => {
          createNewUser.token = response.token;
          await this.authService.saveUserData('Token', JSON.stringify(createNewUser.token));
          loading.dismiss(); // Dismiss loading spinner when response is received

          //console.log("RES" + response);
          this.authService.user.next(createNewUser);
          this.dismissModal();
        });
      });
      this.sucessToast('Welcome to WITP, we hope you enjoy your stay');
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
      duration: 3000,
      position: 'bottom',
    });
    toast.present();
  }

  private async presentLoading() {
    const loading = await this.loadingController.create({
      message: 'Signing up...',
      spinner: 'crescent', // spinner style
      duration: 20000,
    });
    await loading.present();
    return loading;
  }
}

