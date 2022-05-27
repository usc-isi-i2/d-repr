import * as Cookies from "js-cookie";
import { access } from "fs";
import { runInNewContext } from "vm";
import axios, { AxiosRequestConfig } from "axios";
import { BugError } from "src/misc/Exception";
import { notification } from "antd";

export enum SyncStatus {
  synched = "synched",
  synching = "synching",
  error = "error"
}

export class AppTbl {
  public static default() {
    return new AppTbl(SyncStatus.synched, "", "");
  }

  public synchStatus: SyncStatus;
  private _email: string;
  private _accessToken: string;

  constructor(synchStatus: SyncStatus, email: string, accessToken: string) {
    this.synchStatus = synchStatus;
    this._email = email;
    this._accessToken = accessToken;
  }

  public cloneRef() {
    return new AppTbl(this.synchStatus, this._email, this._accessToken);
  }

  get email(): string {
    return this._email;
  }

  public isLoggedIn(): boolean {
    return this._email !== "";
  }

  // send post request to server
  public post(
    url: string,
    data?: any,
    config?: AxiosRequestConfig,
    dismissNotifyError?: boolean
  ) {
    if (config && config.headers) {
      config.headers.Authorization = this._accessToken;
    } else {
      config = {
        ...config,
        headers: { Authorization: this._accessToken }
      };
    }

    if (dismissNotifyError) {
      return axios.post(url, data, config);
    }

    return axios.post(url, data, config).catch(reason => {
      notification.error({
        message: "Error",
        description: reason.response.data.message || reason.message
      });

      throw reason;
    });
  }

  // send get request to server
  public get(
    url: string,
    config?: AxiosRequestConfig,
    dismissNotifyError?: boolean
  ) {
    if (config && config.headers) {
      config.headers.Authorization = this._accessToken;
    } else {
      config = {
        ...config,
        headers: { Authorization: this._accessToken }
      };
    }

    if (dismissNotifyError) {
      return axios.get(url, config);
    }

    return axios.get(url, config).catch(reason => {
      notification.error({
        message: "Error",
        description: reason.response.data.message || reason.message
      });

      throw reason;
    });
  }

  public head(
    url: string,
    config?: AxiosRequestConfig,
    dismissNotifyError?: boolean
  ) {
    if (config && config.headers) {
      config.headers.Authorization = this._accessToken;
    } else {
      config = {
        ...config,
        headers: { Authorization: this._accessToken }
      };
    }

    if (dismissNotifyError) {
      return axios.head(url, config);
    }

    return axios.head(url, config).catch(reason => {
      notification.error({
        message: "Error",
        description: reason.response.data.message || reason.message
      });

      throw reason;
    });
  }

  public delete(
    url: string,
    config?: AxiosRequestConfig,
    dismissNotifyError?: boolean
  ) {
    if (config && config.headers) {
      config.headers.Authorization = this._accessToken;
    } else {
      config = {
        ...config,
        headers: { Authorization: this._accessToken }
      };
    }

    if (dismissNotifyError) {
      return axios.delete(url, config);
    }

    return axios.delete(url, config).catch(reason => {
      notification.error({
        message: "Error",
        description: reason.response.data.message || reason.message
      });

      throw reason;
    });
  }

  // attempt to re-login
  public attempt2ReLoggedIn(): Promise<AppTbl> {
    const email = Cookies.get("email");
    const accessToken = Cookies.get("accessToken");
    if (email === undefined || accessToken === undefined) {
      return Promise.resolve(this);
    }

    // TODO: need to re-connect with server to verify
    const na = this.cloneRef();
    na._email = email;
    na._accessToken = accessToken;

    return na
      .head(`/has_authority`, undefined, true)
      .then(() => na)
      .catch(() => this);
  }

  // update the current access token
  public setAccessToken(email: string, accessToken: string) {
    if (
      !(
        typeof accessToken === "string" &&
        typeof email === "string" &&
        accessToken.length > 0 &&
        email.length > 0
      )
    ) {
      throw new BugError(
        "AccessToken or Email should not be null or undefined"
      );
    }

    this._accessToken = accessToken;
    this._email = email;
    Cookies.set("accessToken", accessToken);
    Cookies.set("email", email);
  }
}
