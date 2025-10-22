export interface Connection {
  id: string;
  name: string;
  host: string;
  port: number;
  username: string;
  password: string;
  private_key_path: string;
  auth_method: AuthMethod;
  created_at?: string;
  updated_at?: string;
}

export interface ConnectionForm {
  name: string;
  host: string;
  port: number;
  username: string;
  password: string;
  private_key_path: string;
  auth_method: AuthMethod;
}

export interface ConnectionListItem {
  id: string;
  name: string;
  host: string;
  port: number;
  connected: boolean;
  active: boolean;
}



export type AuthMethod = "Password" | "PrivateKey" | "Both";
