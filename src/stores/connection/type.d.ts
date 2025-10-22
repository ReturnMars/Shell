// 连接相关类型定义
export interface ConnectionConfig {
  id: string
  name: string
  host: string
  port: number
  username: string
  password?: string | null
  private_key_path?: string | null
  auth_method: AuthMethod
  created_at: string
  updated_at: string
  connected: boolean
  active: boolean
}

export interface ConnectionForm {
  name: string
  host: string
  port: number
  username: string
  password: string
  private_key_path: string
  auth_method: AuthMethod
}

export interface ConnectionListItem {
  id: string
  name: string
  host: string
  port: number
  connected: boolean
  active: boolean
}

export type AuthMethod = "Password" | "PrivateKey" | "Both"

// 连接状态枚举
export enum ConnectionStatus {
  Disconnected = "Disconnected",
  Connecting = "Connecting", 
  Connected = "Connected",
  Error = "Error"
}

// 连接操作结果
export interface ConnectionResult {
  success: boolean
  message: string
  connectionId?: string
}
