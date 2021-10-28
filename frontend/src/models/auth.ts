export interface LoginResult {
  clientToken: string;
  refreshToken: string;
}

export interface UserDetails {
  id: string;
  name?: string;
  email?: string;
}

export enum Permission {
  CanLogin,
  CreateElection,
  Register,
  Vote,
}
