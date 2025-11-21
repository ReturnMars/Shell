export enum LinkStatus {
  INFO = "info",
  SUCCESS = "success",
  CONNECTING = "connecting",
}
export interface LinkItem {
  id: string | number;
  name: string;
  ip: string;
  status: LinkStatus;
}
