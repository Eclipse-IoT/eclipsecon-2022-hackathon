import * as React from "react";
import { useContext, useState } from "react";
import {
  AlertVariant,
  Button,
  EmptyState,
  EmptyStateBody,
  EmptyStateIcon,
  Form,
  FormGroup,
  FormHelperText,
  Modal,
  ModalVariant,
  PageSection,
  TextInput,
  Title
} from "@patternfly/react-core";
import { claimDevice, releaseDevice, useGameService } from "@app/backend";
import { ExclamationCircleIcon, MicrochipIcon } from "@patternfly/react-icons";
import { EndpointsContext, ToastsContext } from "@app/index";
import { useAuth } from "oidc-react";
import { DeviceState } from "@app/DeviceState/DeviceState";

type validate = "success" | "error";

const Dashboard: React.FunctionComponent = () => {

  const [isModalOpen, setIsModalOpen] = useState(false);
  const handleModalToggle = () => {
    setIsModalOpen(!isModalOpen);
  };

  const toasts = useContext(ToastsContext);
  const [deviceIdValue, setDeviceIdValue] = useState("");
  const auth = useAuth();

  const [service, reload] = useGameService();
  const endpoints = useContext(EndpointsContext);

  const onClaimDevice = async () => {
    try {
      await claimDevice(endpoints, deviceIdValue, auth.userData?.access_token);
      toasts.addAlert?.(AlertVariant.success, "Claimed device", 5000);
    } catch (err) {
      toasts.addAlert?.(AlertVariant.danger, `Failed to claim device: ${err}`);
    }

    handleModalToggle();
    reload();
  };

  const onReleaseDevice = async (deviceId) => {
    try {
      await releaseDevice(endpoints, deviceId, auth.userData?.access_token);
      toasts.addAlert?.(AlertVariant.success, "Released device", 5000);
    } catch (err) {
      toasts.addAlert?.(AlertVariant.danger, `Failed to release device: ${err}`);
    }
    reload();
  };

  const [validated, setValidated] = useState<validate>("error");
  const handleDeviceIdChange = (value: string) => {
    setDeviceIdValue(value);
    if (value !== "") {
      setValidated("success");
    } else {
      setValidated("error");
    }
  };

  if (service.status === "loaded") {
    let claim;
    if (service.payload?.deviceId !== undefined) {
      claim = (<React.Fragment>
        <>Device claimed: {service.payload?.deviceId}</>
        <Button variant="secondary" isDanger onClick={() => onReleaseDevice(service.payload?.deviceId)}>Release</Button>

        <DeviceState></DeviceState>

      </React.Fragment>);
    } else {
      claim = (
        <React.Fragment>
          <EmptyState>
            <EmptyStateIcon icon={MicrochipIcon} />
            <Title headingLevel="h4" size="lg">
              No device claimed
            </Title>
            <EmptyStateBody>
              You do not yet claimed a device.
            </EmptyStateBody>
            <Button variant="primary" onClick={handleModalToggle}>Claim device</Button>
          </EmptyState>

          <Modal
            variant={ModalVariant.small}
            title="Claim device"
            isOpen={isModalOpen}
            onClose={handleModalToggle}
            actions={[
              <Button key="confirm" variant="primary" onClick={onClaimDevice}
                      isDisabled={validated !== "success"}>Claim</Button>,
              <Button key="cancel" variant="link" onClick={handleModalToggle}>Cancel</Button>
            ]}
          >
            <Form id="claim-device-modal" onSubmit={onClaimDevice} method="dialog">
              <FormGroup
                label="Device ID"
                isRequired
                fieldId="claimDeviceId"
                helperText={
                  <FormHelperText icon={<ExclamationCircleIcon />} isHidden={validated !== "error"}>
                    Enter the device ID
                  </FormHelperText>
                }
                helperTextInvalid="Must not be empty"
                helperTextInvalidIcon={<ExclamationCircleIcon />}
                validated={validated}
              >
                <TextInput
                  isRequired
                  id="claimDeviceId"
                  onChange={handleDeviceIdChange}
                  validated={validated}
                  aria-describedby="claimDeviceId-helper"
                >
                </TextInput>
              </FormGroup>
            </Form>
          </Modal>
        </React.Fragment>
      );
    }

    return (<PageSection>
      {claim}
    </PageSection>);

  } else {
    return (<></>);
  }

};

export { Dashboard };
