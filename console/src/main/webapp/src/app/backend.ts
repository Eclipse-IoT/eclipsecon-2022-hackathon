import { DispatchWithoutAction, useContext, useEffect, useReducer, useState } from "react";
import { useAuth } from "oidc-react";
import { EndpointsContext } from "@app/index";
import { AuthContextProps } from "oidc-react/build/src/AuthContextInterface";

export interface DeviceClaim {
  id: string | null;
  deviceId: string | null;
  password: string | null;
}

interface ServiceInit {
  status: "init";
}

interface ServiceLoading {
  status: "loading";
}

interface ServiceLoaded<T> {
  status: "loaded";
  payload: T;
}

interface ServiceError {
  status: "error";
  error: Error;
}

export type Service<T> =
  | ServiceInit
  | ServiceLoading
  | ServiceLoaded<T>
  | ServiceError;

interface EndpointsInner {
  authServerUrl: string;
  api?: string;
  ws?: string;
  simulatorUrl?: string,
}

export class Endpoints {

  private inner: EndpointsInner;

  constructor(inner?: EndpointsInner) {
    if (inner) {
      this.inner = inner;
    } else {
      this.inner = { authServerUrl: "" };
    }
  }

  get authServerUrl(): string {
    return this.inner.authServerUrl;
  }

  get apiBase(): string {
    if (this.inner.api) {
      return this.inner.api;
    } else {
      const url = new URL(document.URL);
      url.pathname = "";
      url.search = "";
      url.hash = "";
      return url.toString();
    }
  }

  get wsBase(): string {
    if (this.inner.ws) {
      return this.inner.ws;
    } else {
      const url = new URL(this.apiBase);
      let protocol;
      if (url.protocol === "http:") {
        protocol = "ws";
      } else {
        protocol = "wss";
      }
      return protocol + "://" + url.host;
    }
  }

  private url(base: string, path?: string): string {
    let result = base;
    while (result.endsWith("/")) {
      result = result.slice(0, -1);
    }
    if (path !== undefined) {
      if (!path.startsWith("/")) {
        result += "/";
      }
      result += path;
    }
    return result;
  }

  api(path?: string): string {
    return this.url(this.apiBase, path);
  }

  ws(path?: string): string {
    return this.url(this.wsBase, path);
  }

  get simulatorUrl(): string | undefined {
    return this.inner.simulatorUrl;
  }
}

const useEndpoints = (): Service<Endpoints> => {

  const [endpoints, setEndpoints] = useState<Service<Endpoints>>({ status: "loading" });

  useEffect(() => {
    console.log("Fetching backend information");
    fetch("/.well-known/eclipsecon-2022/endpoints", {
      cache: "no-cache"
    })
      .then(response => {
        if (!response.ok) {
          throw new Error(`Failed to retrieve endpoint information: ${response.status} - ${response.statusText}`);
        }
        return response;
      })
      .then(response => response.json())
      .then(payload => {
        console.log("Loaded endpoints: ", payload);
        setEndpoints({ status: "loaded", payload: new Endpoints({ ...payload }) });
      })
      .catch(error => {
        console.error("Failed to load backend information", error);
        setEndpoints({ status: "error", error });
      });

  }, []);

  return endpoints;
};

const useGameService = (): [Service<DeviceClaim>, DispatchWithoutAction] => {
  const [result, setResult] = useState<Service<DeviceClaim>>({ status: "loading" });
  const auth = useAuth();
  const [trigger, reload] = useReducer((x) => x + 1, 0);

  const endpoints = useContext<Endpoints>(EndpointsContext);

  useEffect(() => {

    const url = endpoints?.api("/api/deviceClaims/v1alpha1");

    fetch(url, {
      headers: new Headers({
        "Authorization": "Bearer " + auth.userData?.access_token
      })
    })
      .then(response => {
        if (!response.ok) {
          throw new Error(`Request failed: ${response.status}: ${response.statusText}`);
        }
        return response;
      })
      .then(response => response.json())
      .then(payload => setResult({ status: "loaded", payload }))
      .catch(error => setResult({ status: "error", error }));
  }, [auth, trigger, endpoints]);

  return [result, reload];
};

interface DisplaySettings {
  brightness: number;
  enabled: boolean;
}

const setDisplay = async (endpoints: Service<Endpoints>, auth: AuthContextProps, display: DisplaySettings): Promise<Response> => {

  if (endpoints.status !== "loaded") {
    return Promise.reject("Missing endpoints");
  }

  const url = endpoints.payload.api("/api/commands/v1alpha1/display");

  return await fetch(url, {
    method: "POST",
    headers: new Headers({
      "Authorization": "Bearer " + auth.userData?.access_token,
      "Content-Type": "application/json"
    }),
    body: JSON.stringify(display)
  }).then(response => {
    if (!response.ok) {
      throw new Error(`Request failed: ${response.status}: ${response.statusText}`);
    }
    return response;
  });

};

const claimDevice = async (endpoints: Endpoints, deviceId: string, accessToken?: string): Promise<Response> => {

  const url = endpoints.api("/api/deviceClaims/v1alpha1?" + new URLSearchParams({
    deviceId
  }));

  return await fetch(url, {
    method: "PUT",
    headers: new Headers({
      "Authorization": "Bearer " + accessToken
    })
  })
    .then(response => {
      if (!response.ok) {
        throw new Error(`Request failed: ${response.status}: ${response.statusText}`);
      }
      return response;
    });
};

const releaseDevice = async (endpoints: Endpoints, deviceId: string, accessToken?: string): Promise<Response> => {
  const url = endpoints.api("/api/deviceClaims/v1alpha1?" + new URLSearchParams({
    deviceId
  }));

  return await fetch(url, {
    method: "DELETE",
    headers: new Headers({
      "Authorization": "Bearer " + accessToken
    })
  })
    .then(response => {
      if (!response.ok) {
        throw new Error(`Request failed: ${response.status}: ${response.statusText}`);
      }
      return response;
    });
};

const createSimulator = async (endpoints: Endpoints, accessToken?: string): Promise<Response> => {
  const url = endpoints.api("/api/deviceClaims/v1alpha1/simulator");

  return await fetch(url, {
    method: "PUT",
    headers: new Headers({
      "Authorization": "Bearer " + accessToken
    })
  })
    .then(response => {
      if (!response.ok) {
        throw new Error(`Request failed: ${response.status}: ${response.statusText}`);
      }
      return response;
    });
};

export { useEndpoints, useGameService, claimDevice, releaseDevice, setDisplay, createSimulator };
