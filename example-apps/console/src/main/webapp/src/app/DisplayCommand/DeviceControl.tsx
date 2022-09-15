import * as React from "react";
import { useContext, useState } from "react";
import {
  AlertVariant,
  Button,
  InputGroup,
  NumberInput,
  Toolbar,
  ToolbarContent,
  ToolbarItem
} from "@patternfly/react-core";
import { setDisplay, useEndpoints } from "@app/backend";
import { useAuth } from "oidc-react";
import { ToastsContext } from "@app/index";

const DeviceControl: React.FunctionComponent = () => {

  const [brightness, setBrightness] = useState(0);
  const endpoints = useEndpoints();
  const auth = useAuth();
  const toast = useContext(ToastsContext);

  const onClick = async () => {
    await setDisplay(endpoints, auth, {
      brightness,
      enabled: brightness > 0
    }).then(() => {
      toast.addAlert?.(AlertVariant.success, "Sent display command", 5000);
    }).catch((err) => {
      console.warn("Failed: ", err);
      toast.addAlert?.(AlertVariant.warning, "Failed", false, (<p>Failed to send command: {err}</p>));
    });

  };

  return (
    <Toolbar>
      <ToolbarContent>
        <ToolbarItem variant="label">Display</ToolbarItem>
        <ToolbarItem>
          <InputGroup>
            <NumberInput
              value={brightness}
              min={0}
              max={255}
              onMinus={() => {
                setBrightness(brightness - 5);
              }}
              onChange={(event) => {
                if (event.target instanceof HTMLInputElement) {
                  let b = Number(event.target.value);
                  if (b > 255) {
                    b = 255;
                  }
                  if (b < 0) {
                    b = 0;
                  }
                  setBrightness(b);
                }
              }}
              onPlus={() => {
                setBrightness(brightness + 5);
              }}
              minusBtnAriaLabel="minus"
              plusBtnAriaLabel="plus"
            />
          </InputGroup>
        </ToolbarItem>
        <ToolbarItem>
          <Button variant="control" onClick={onClick}> Set</Button>
        </ToolbarItem>
      </ToolbarContent>
    </Toolbar>
  )
    ;
};

export { DeviceControl };
