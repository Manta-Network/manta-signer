import React, { useState } from 'react';
import { Button, Header, Input } from 'semantic-ui-react';

const Authorize = ({
  summary,
  sendPassword,
  stopPasswordPrompt,
  hideWindow,
}) => {
  const [password, setPassword] = useState('');

  var header;
  var summaryMessage;
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

  const authorize = async () => {
    console.log("[INFO]: Authorize.");
    const shouldRetry = await sendPassword(password);
    setPassword('');
    // FIXME: Clear the input element too.
    if (!shouldRetry) {
      hideWindow();
    } else {
      console.log("RETRY!");
    }
  };

  const decline = async () => {
    console.log("[INFO]: Decline.");
    setPassword('');
    // FIXME: Clear the input element too.
    await stopPasswordPrompt();
    hideWindow();
  };

  return (
    <>
      <Header>{header}</Header>
      <div className="authorize-summary">{summaryMessage}</div>
      <Input
        type="password"
        label="Password"
        onChange={(e) => setPassword(e.target.value)}
      />
      <Button className="button" onClick={authorize}>
        Authorize
      </Button>
      <Button className="button" onClick={decline}>
        Decline
      </Button>
    </>
  );
};

export default Authorize;
