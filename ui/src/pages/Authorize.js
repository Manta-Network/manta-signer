import React, { useState } from 'react';
import { Button, Header, Input } from 'semantic-ui-react';

const Authorize = ({
  summary,
  sendPassword,
  stopPasswordPrompt,
  hideWindow,
}) => {
  const [password, setPassword] = useState('');
  const [passwordInvalid, setPasswordInvalid] = useState(false)

  let header;
  let summaryMessage;
  switch (summary.type) {
    case "Reclaim":
      header = "Authorize Transaction";
      summaryMessage = `Withdraw ${summary.amount} ${summary.currency_symbol} to your public wallet`;
      break;
    case "PrivateTransfer":
      header = "Authorize Transaction";
      summaryMessage = `Transfer ${summary.amount} ${summary.currency_symbol} to: ${summary.recipient}`;
      break;
    default:
      header = "Login";
      summaryMessage = ""
      break;
  }

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
      <Header>{header}</Header>
      <div className="authorize-summary">{summaryMessage}</div>
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
