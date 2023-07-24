export interface UserInterface {
  authType: string;
  email: string;
  password: string;
  firstName: string;
  lastName: string;
  token?: string;
}

export class User implements UserInterface {
  constructor(
    public authType: string,
    public email: string,
    public password: string,
    public firstName: string,
    public lastName: string,
    public token?: string,
  ) { }
}
