import {ActionGroup, Button, Form, LoginForm, LoginPage} from "@patternfly/react-core";
import * as React from "react";

import lg from '@patternfly/patternfly/assets/images/pfbg_1200.jpg';
import sm from '@patternfly/patternfly/assets/images/pfbg_768.jpg';
import sm2x from '@patternfly/patternfly/assets/images/pfbg_768@2x.jpg';
import xs from '@patternfly/patternfly/assets/images/pfbg_576.jpg';
import xs2x from '@patternfly/patternfly/assets/images/pfbg_576@2x.jpg';
import {useAuth} from "oidc-react";

const images = {lg, sm, sm2x, xs, xs2x};

const Login: React.FunctionComponent = () => {
  const auth = useAuth();

  return (
    <LoginPage
      loginTitle="Log in to your account"
      loginSubtitle="Log in to the application using single sign-on (SSO)"
      backgroundImgSrc={images}
      backgroundImgAlt="Images"
    >
      <Form>
        <ActionGroup>
          <Button isBlock onClick={() => auth.signIn()}>Log in using SSO</Button>
        </ActionGroup>
      </Form>
    </LoginPage>
  );
}

export {Login};
