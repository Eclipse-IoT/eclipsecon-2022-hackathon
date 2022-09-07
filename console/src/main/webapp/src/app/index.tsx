import * as React from "react";
import "@patternfly/react-core/dist/styles/base.css";
import { BrowserRouter as Router } from "react-router-dom";
import { AppLayout } from "@app/AppLayout/AppLayout";
import { AppRoutes } from "@app/routes";
import "@app/app.css";
import { AuthProvider, useAuth } from "oidc-react";
import { Login } from "@app/Login/Login";
import { Alert, AlertActionCloseButton, AlertGroup, AlertProps, AlertVariant } from "@patternfly/react-core";
import { useState } from "react";
import { Endpoints, useEndpoints } from "@app/backend";

const Content: React.FunctionComponent = () => {

  const auth = useAuth();

  if (auth && auth.userData) {
    return (
      <Router>
        <AppLayout>
          <AppRoutes />
        </AppLayout>
      </Router>
    );
  } else if (!auth.isLoading) {
    return (
      <Login />
    );
  } else {
    return (<></>);
  }

};

export interface Toasts {
  addAlert: (variant: AlertVariant, title: string, timeout?: number | boolean) => void;
  removeAlert: (key: React.Key) => void;
}

export const ToastsContext = React.createContext<Partial<Toasts>>({});
export const EndpointsContext = React.createContext<Endpoints>(new Endpoints());

const App: React.FunctionComponent = () => {

  const [alerts, setAlerts] = useState<Partial<AlertProps>[]>([]);

  const addAlert = (variant: AlertVariant, title: string, timeout?: number | boolean) => {
    setAlerts(prevState => [...prevState, {
      title, variant, timeout, key: new Date().getTime()
    }]);
  };

  const removeAlert = (key?: React.Key) => {
    setAlerts(prevState => [...prevState.filter(alert => alert.key !== key)]);
  };

  const endpoints = useEndpoints();

  let content;
  if (endpoints.status === "loaded") {

    const authConfig = {
      onSignIn: async () => {
        window.location.search = "";
        window.location.hash = "";
      },
      authority: endpoints.payload.authServerUrl,
      clientId: "frontend",
      redirectUri: document.location.toString(),
      automaticSilentRenew: true,
      autoSignIn: false
    };

    content = <EndpointsContext.Provider value={endpoints.payload}>
      <AuthProvider {...authConfig}>
        <Content />
      </AuthProvider>
    </EndpointsContext.Provider>;
  } else {
    content = <div>Loading…</div>;
  }

  return (
    <React.Fragment>
      <AlertGroup isToast isLiveRegion>
        {alerts.map(({ key, variant, title, timeout }) => {

          let actionClose;
          if (!timeout) {
            actionClose = (<AlertActionCloseButton
              title="Close"
              onClose={() => removeAlert(key)}
            />);
          }

          return (<Alert
            variant={variant}
            title={title}
            timeout={timeout}
            actionClose={actionClose}
            onTimeout={() => removeAlert(key)}
            key={key}
          />);
        })}
      </AlertGroup>

      <ToastsContext.Provider value={{
        addAlert, removeAlert
      }}>
        {content}
      </ToastsContext.Provider>

    </React.Fragment>
  );

};

export default App;
