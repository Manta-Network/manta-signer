import { once } from '@tauri-apps/api/event';
import React, { useState, useEffect } from 'react';
import { Button, Header, Input } from 'semantic-ui-react';

const Authorize = ({
  summary,
  sendPassword,
  stopPasswordPrompt,
  hideWindow,
}) => {
  const [password, setPassword] = useState('');
  const [passwordInvalid, setPasswordInvalid] = useState(false);

  useEffect(() => {
    once("abort_auth", async () => {
      await onClickDecline();
    });
  });

  const onClickAuthorize = async () => {
    console.log("[INFO]: Authorizing.");
    const shouldRetry = await sendPassword(password);
    if (!shouldRetry) {
      setPassword('');
      setPasswordInvalid(false)
      hideWindow();
    } else {
      setPasswordInvalid(true)
    }
  };

  const onClickDecline = async () => {
    console.log("[INFO]: Declining Transaction.");
    setPassword('');
    setPasswordInvalid(false)
    await stopPasswordPrompt();
    hideWindow();
  };

  const onChangePassword = password => {
    setPassword(password)
    setPasswordInvalid(false)
  }

  return (
    <>
      <Header>Authorize</Header>
      <div className="authorize-summary">{summary}</div>
      <Input
        type="password"
        label="Password"
        value={password}
        onChange={(e) => onChangePassword(e.target.value)}
        error={passwordInvalid}
      />
      <Button className="button" onClick={onClickAuthorize}>
        Authorize
      </Button>
      <Button className="button" onClick={onClickDecline}>
        Decline
      </Button>
    </>
  );
};

export default Authorize;
