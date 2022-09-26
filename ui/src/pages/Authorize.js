import { once } from '@tauri-apps/api/event';
import React, { useState, useEffect } from 'react';
import { Button, Input } from 'semantic-ui-react';

// Hardcoding current network for now until multi network support
const CURRENT_NETWORK = "Dolphin";

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
      <div className='authTransactionHeader'>
        <h1 className='mainheadline'>Authorize Transaction</h1>
      </div>
      <div className='transactionContainer'>
        <div className='transactionDetail'>
          <h5 className='transactionDescription'>Send</h5>
          <div>
            <h5 className='transactionValue'>{summary.sendAmount + " " + summary.currency}</h5>
          </div>
        </div>
        <div className='transactionDetail'>
          <h5 className='transactionDescription'>To</h5>
          <h5 className='transactionValue'>{summary.toAddress}</h5>
        </div>
        <div className='transactionDetailPadded'>
          <h5 className='transactionDescription'>Network</h5>
          <h5 className='transactionValue'>{CURRENT_NETWORK}</h5>
        </div>
      </div>
      <Input
        className='input ui password'
        type="password"
        placeholder="Enter Password"
        value={password}
        onChange={(e) => onChangePassword(e.target.value)}
        error={passwordInvalid}
      />
      <div className='authButtonsContainer'>
        <Button className="button ui cancel" onClick={onClickDecline}>
          Cancel
        </Button>
        <Button className="button ui first thin" onClick={onClickAuthorize}>
          Authorize
        </Button>
      </div>
    </>
  );
};

export default Authorize;
