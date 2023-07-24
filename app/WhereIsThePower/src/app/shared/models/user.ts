export interface UserInterface {
  authType: string;
  email: string;
  password: string;
  token?: string;
}

export class User implements UserInterface {
  constructor(
    public authType: string,
    public email: string,
    public password: string,
    public token?: string,
  ) { }
}
