import * as React from "react";
import { useState, useEffect, useRef, useContext } from "react";
import { Endpoints, useEndpoints } from "@app/backend";
import { useAuth } from "oidc-react";
import { EndpointsContext } from "@app/index";

enum ConnectionState {
  Disconnected = "Disconnected",
  Connected = "Connected",
}

interface State {
  connectionState: ConnectionState,
  lastMessage?: LastMessage,
}

interface LastMessage {
  state?: string,
}

interface ConnectionOptions {
  onOpen?: (event: Event) => void;
  onClose?: (event: CloseEvent) => void;
  onError?: (event: Event) => void;
  onMessage?: (event: MessageEvent) => void;
}

class Connection {
  private readonly url: string;
  private ws: WebSocket;
  private closed: boolean;
  private timer?: number;
  private opts?: ConnectionOptions;
  private currentToken?: string;

  constructor(url, opts) {
    this.url = url;
    this.closed = false;
    this.ws = this.connect();
    this.opts = opts;
  }

  connect() {
    const ws = new WebSocket(this.url);
    ws.onopen = (event) => {
      if (this.currentToken !== undefined) {
        this.sendToken();
      }
      this.opts?.onOpen?.(event);
    };
    ws.onclose = (event) => {
      this.reconnect();
      this.opts?.onClose?.(event);
    };
    ws.onerror = (event) => {
      this.reconnect();
      this.opts?.onError?.(event);
    };
    ws.onmessage = (event) => {
      this.opts?.onMessage?.(event);
    };
    return ws;
  }

  close() {
    this.closed = true;
    this.ws.close();
  }

  performReconnect() {
    if (this.closed) {
      return;
    }

    this.ws = this.connect();
  }

  reconnect() {
    if (this.closed || this.timer !== undefined) {
      return;
    }

    this.timer = window.setTimeout(() => {
      this.timer = undefined;
      this.performReconnect();
    }, 5000);
  }

  accessToken(token?: string) {
    this.currentToken = token;
    if (this.ws.readyState === WebSocket.OPEN) {
      this.sendToken();
    }
  }

  private sendToken() {
    this.ws.send(JSON.stringify({ token: this.currentToken }));
  }
}

const DeviceState: React.FunctionComponent = () => {

  const [state, setState] = useState<State>({
    connectionState: ConnectionState.Disconnected
  });

  const auth = useAuth();

  const ws = useRef<Connection>();

  const endpoints = useContext(EndpointsContext);

  useEffect(() => {

    const url = endpoints.ws("/api/events/v1alpha1");

    console.log("WebSocket: ", url);

    ws.current = new Connection(url, {
      onOpen: () => {
        setState({
          connectionState: ConnectionState.Connected
        });
      },
      onError: () => {
        setState({
          connectionState: ConnectionState.Disconnected
        });
      },
      onClose: () => {
        setState({
          connectionState: ConnectionState.Disconnected
        });
      },
      onMessage: (event) => {
        if (event.type === "message") {
          const lastMessage: LastMessage = JSON.parse(event.data);
          setState({
            connectionState: ConnectionState.Connected,
            lastMessage
          });
        }
      }
    });
    const w = ws.current;
    return () => w.close();
  }, [endpoints]);

  useEffect(() => {
    ws.current?.accessToken(auth.userData?.access_token);
  }, [ws, auth]);

  return <div>
    {state.connectionState} / <code>
    <pre>{JSON.stringify(state.lastMessage, null, 2)}</pre>
  </code>
  </div>;
};

export { DeviceState };

