import React, { useState } from 'react';
import { Button, Input, Form, Icon } from 'semantic-ui-react';
import mantaLogo from "../icons/manta.png";
import dolphinLogo from "../icons/Square150x150Logo.png";
import calamariLogo from "../icons/calamari.png";
import newAccount from "../icons/new_account.png";
import "../App.css";
import HyperLinkButton from '../components/HyperLinkButton';

const SignIn = ({
  sendSelection,
  getReceivingKeys,
  receivingKey,
  receivingKeyDisplay,
  sendPassword,
  endInitialConnectionPhase,
  startRecover,
  hideWindow
}) => {
  const [password, setPassword] = useState('');
  const [passwordInvalid, setPasswordInvalid] = useState(null);
  const [loginSuccess, setLoginSuccess] = useState(false);
  const [showCopyNotification, setShowCopyNotification] = useState(false);
  const [loading, setLoading] = useState(false);

  const onClickSignIn = async () => {
    setLoading(true);
    await sendSelection("SignIn");
    const shouldRetry = await sendPassword(password);

    if (!shouldRetry) {

      await getReceivingKeys();
      await endInitialConnectionPhase();
      setPassword('');
      setLoginSuccess(true);
    } else {
      console.log("[INFO]: Invalid password, RETRY!");
      setPasswordInvalid(true);
    }
    setLoading(false);
  };

  const onChangePassword = password => {
    setPassword(password)
    setPasswordInvalid(false)
  }

  const onClickCopyZkAddress = () => {
    navigator.clipboard.writeText(receivingKey);

    if (!showCopyNotification) {
      setShowCopyNotification(true);
      setTimeout(function () { setShowCopyNotification(false) }, 2000);
    }

  }

  const onClickForgotPassword = async () => {
    startRecover();
  }

  const onClickFinishSignIn = async () => {
    await hideWindow();
  }

  return (<>
    {!loginSuccess &&
      <div>
        <div className='main-logo-container login'>
          <img className="main-logo" alt="Manta Logo" src={mantaLogo} />
        </div>

        <div>
          <h1 className='main-headline'>Welcome Back!</h1>
          <p className='sub-text'>Let's connect you to the Web3 privacy hub.</p>
        </div>
        <Form.Field>
          <Input
            className='input ui password tall'
            type="password"
            placeholder="Password"
            value={password}
            onChange={(e) => onChangePassword(e.target.value)}
            error={passwordInvalid}
          />
        </Form.Field>
        {
          loading ?
            <Button disabled={true} className="button ui first">Loading</Button>
            :
            <Button className="button ui first" onClick={onClickSignIn}>
              Unlock
            </Button>
        }
        <HyperLinkButton
          text={"Forgot Password?"}
          onclick={onClickForgotPassword}
        />

        <div className='learn-about-manta-small'>
          <p>Learn more about &nbsp;
            <a href='https://www.manta.network/' target="_blank" rel="noreferrer">
              Manta
            </a>
          </p>
        </div>
      </div>
    }
    {loginSuccess &&
      <div>
        <div className='finish-logo-container'>
          <img className="main-logo" alt="Manta Logo" src={newAccount} />
        </div>

        <div>
          <h1 className='main-headline'>Your zkAddress</h1>
        </div>
        <div className='zk-address-container'>
          <p className='sub-text'>{receivingKeyDisplay}</p>
          {showCopyNotification ?
            <Button onClick={onClickCopyZkAddress} className='button ui copy'>
              <Icon name="checkmark" className='specific' />
            </Button> :
            <Button onClick={onClickCopyZkAddress} className='button ui copy'>
              <Icon name="copy outline" className='specific' />
            </Button>
          }
        </div>
        <div className='supported-networks-container'>
          <div className='supported-networks-child'>
            <div className='supported-networks-header'>
              <h4>Supported Networks</h4>
            </div>

            <table className='network-table'>
              <tbody>
                <tr>
                  <th><img className='mini-dolphin-logo' alt="Dolphin Logo" src={dolphinLogo} /></th>
                  <th><p className='network-text'>Dolphin Network</p></th>
                  <th></th>
                </tr>
                <tr>
                  <th><img className='mini-calamari-logo' alt="Calamari Logo" src={calamariLogo} /></th>
                  <th><p className='network-text'>&nbsp;Calamari Network&nbsp;&nbsp;</p></th>
                  <th><a href='https://calamari.network/' target="_blank" rel="noreferrer">(soon)</a></th>
                </tr>
                <tr>
                  <th><img className='mini-manta-logo' alt="Manta Logo" src={mantaLogo} /></th>
                  <th><p className='network-text'>Manta Network</p></th>
                  <th><a href='https://calamari.network/' target="_blank" rel="noreferrer">(soon)</a></th>
                </tr>
              </tbody>
            </table>
          </div>

        </div>
        <Button className="button ui first" onClick={onClickFinishSignIn}>
          Start
        </Button>
      </div>
    }
  </>);
};

export default SignIn;
