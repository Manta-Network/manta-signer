import { once } from '@tauri-apps/api/event';
import React, { useState, useEffect } from 'react';
import { Button, Input } from 'semantic-ui-react';

const Authorize = ({
  cancelSign,
  summary,
  sendPassword,
  stopPasswordPrompt,
  hideWindow,
}) => {
  const [password, setPassword] = useState('');
  const [passwordInvalid, setPasswordInvalid] = useState(false);

  useEffect(() => {
    once("abort_auth", async () => {
      console.log("[INFO]: Authorization window aborting to cancel function");
      await onClickDecline();
    });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

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
    await cancelSign();
    hideWindow();
  };

  const onChangePassword = password => {
    setPassword(password)
    setPasswordInvalid(false)
  }

  return (
    <>
      {!summary.isTransactionDataRequest ? 
      <div className='auth-transaction-header'>
        <h1 className='main-headline'>Authorize Transaction Data Request</h1>
      </div> :
      <div>
        <div className='auth-transaction-header'>
          <h1 className='main-headline'>Authorize Transaction</h1>
        </div>
        <div className='transaction-container'>
          <div className='transaction-detail'>
            <h5 className='transaction-description'>Send</h5>
            <div>
              <h5 className='transaction-value'>{summary.sendAmount + " " + summary.currency}</h5>
            </div>
          </div>
          <div className='transaction-detail'>
            <h5 className='transaction-description'>To</h5>
            <h5 className='transaction-value'>{summary.toAddress}</h5>
          </div>
          <div className='transaction-detail-padded'>
            <h5 className='transaction-description'>Network</h5>
            <h5 className='transaction-value'>{summary.network}</h5>
          </div>
        </div>
      </div>
      }
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
