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

/// Data structure for the JSON Web Token (JWT)
export interface ClientToken {
  iss: string; // Issuer
  sub: string; // Subject (whom token refers to)
  aud: string; // Audience (whom the token is intended for)
  iat: number; // Issued at (as UTC timestamp)
  exp: number; // Expiration time (as UTC timestamp)

  // Public and private claims
  name: string;
  email: string;
  permissions: Permission[];
}

export type RefreshToken = ClientToken;
