import { DispatchWithoutAction, useEffect, useReducer, useState } from "react";
import { useAuth } from "oidc-react";
import { ToastsContext } from "@app/index";

export interface DeviceClaim {
  deviceId: string | null;
}

interface ServiceInit {
  status: 'init';
}

interface ServiceLoading {
  status: 'loading';
}

interface ServiceLoaded<T> {
  status: 'loaded';
  payload: T;
}

interface ServiceError {
  status: 'error';
  error: Error;
}

export type Service<T> =
  | ServiceInit
  | ServiceLoading
  | ServiceLoaded<T>
  | ServiceError;

const API_BASE = 'http://localhost:8080';
const WS_BASE = 'ws://localhost:8080';

const useGameService = (): [Service<DeviceClaim>, DispatchWithoutAction] => {
  const [result, setResult] = useState<Service<DeviceClaim>>({status: 'loading'});
  const auth = useAuth();
  const [trigger, reload] = useReducer((x) => x + 1, 0);

  useEffect(() => {

    if (auth.userData?.expired) {
      auth.userManager.startSilentRenew()
    }

    fetch( API_BASE + '/api/deviceClaims/v1alpha1', {
      headers: new Headers({
        'Authorization': 'Bearer ' + auth.userData?.access_token
      })
    })
      .then(response => {
        if (!response.ok) {
          throw new Error(`Request failed: ${response.status}: ${response.statusText}`);
        }
        return response;
      })
      .then(response => response.json())
      .then(response => setResult({status: 'loaded', payload: response}))
      .catch(error => setResult({status: 'error', error}))
  }, [auth, trigger]);

  return [result, reload];
}

const claimDevice = async (deviceId: string, accessToken?: string): Promise<Response> => {
  const url = API_BASE + '/api/deviceClaims/v1alpha1?' + new URLSearchParams({
    deviceId
  });

  return await fetch( url, {
    method: "PUT",
    headers: new Headers({
      'Authorization': 'Bearer ' + accessToken
    })
  })
    .then(response => {
      if (!response.ok) {
        throw new Error(`Request failed: ${response.status}: ${response.statusText}`);
      }
      return response;
    });
}

const releaseDevice = async (deviceId: string, accessToken?: string): Promise<Response> => {
  const url = API_BASE + '/api/deviceClaims/v1alpha1?' + new URLSearchParams({
    deviceId
  });

  return await fetch( url, {
    method: "DELETE",
    headers: new Headers({
      'Authorization': 'Bearer ' + accessToken
    })
  })
    .then(response => {
      if (!response.ok) {
        throw new Error(`Request failed: ${response.status}: ${response.statusText}`);
      }
      return response;
    });
}

export {useGameService, claimDevice, releaseDevice, API_BASE, WS_BASE};
