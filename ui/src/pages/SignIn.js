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
  startRecover
}) => {
  const [password, setPassword] = useState('');
  const [passwordInvalid, setPasswordInvalid] = useState(null);
  const [loginSuccess, setLoginSuccess] = useState(false);
  const [showCopyNotification, setShowCopyNotification] = useState(false);

  const onClickSignIn = async () => {
    await sendSelection("SignIn");
    const shouldRetry = await sendPassword(password);

    if (!shouldRetry) {

      await getReceivingKeys();
      setPassword('');
      setLoginSuccess(true);
      console.log(receivingKey);
    } else {
      console.log("RETRY!");
      setPasswordInvalid(true);
    }
  };

  const onChangePassword = password => {
    setPassword(password)
    setPasswordInvalid(false)
  }

  const onClickCopyZkAddress = () => {
    navigator.clipboard.writeText(receivingKey);

    if (!showCopyNotification) {
      setShowCopyNotification(true);
      setTimeout(function () {setShowCopyNotification(false)},2000);
    }

  }

  const onClickForgotPassword = async () => {
    startRecover();
  }

  const onClickFinishSignIn = async () => {
    console.log("[INFO]: End Initial Connection Phase");
    await endInitialConnectionPhase();
  }

  return (<>
    {!loginSuccess &&
      <div>
        <div className='mainlogocontainer login'>
          <img className="mainlogo" alt="Manta Logo" src={mantaLogo} />
        </div>

        <div>
          <h1 className='mainheadline'>Welcome Back!</h1>
          <p className='subtext'>Let`s connect you to the Web3 privacy hub.</p>
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
        <Button className="button ui first" onClick={onClickSignIn}>
          Unlock
        </Button>
        <HyperLinkButton
          text={"Forgot Password?"}
          onclick={onClickForgotPassword}
        />

        <div className='learnAboutMantaSmall'>
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
        <div className='finishLogoContainer'>
          <img className="mainlogo" alt="Manta Logo" src={newAccount} />
        </div>

        <div>
          <h1 className='mainheadline'>Your zkAddress</h1>
        </div>
        <div className='zkAddressContainer'>
          <p className='subtext'>{receivingKeyDisplay}</p>
          <Button onClick={onClickCopyZkAddress} className='button ui copy'>
            <Icon name="copy outline" className='specific' />
          </Button>
        </div>
        {showCopyNotification ? <p className='subtext copy'>&nbsp;Copied!</p> : <br/>}
        <div className='supportedNetworksContainer'>
          <div className='supportedNetworksChild'>
            <div className='supportedNetworksHeader'>
              <h4>Supported Networks</h4>
            </div>

              <table className='networkTable'>
                <tr>
                  <th><img className='miniDolphinLogo' alt="Dolphin Logo" src={dolphinLogo} /></th>
                  <th><p className='networkText'>Dolphin Network</p></th>
                  <th></th>
                </tr>
                <tr>
                  <th><img className='miniCalamariLogo' alt="Calamari Logo" src={calamariLogo} /></th>
                  <th><p className='networkText'>&nbsp;Calamari Network&nbsp;&nbsp;</p></th>
                  <th><a href='https://calamari.network/' target="_blank" rel="noreferrer">(soon)</a></th>
                </tr>
                <tr>
                  <th><img className='miniMantaLogo' alt="Manta Logo" src={mantaLogo} /></th>
                  <th><p className='networkText'>Manta Network</p></th>
                  <th><a href='https://calamari.network/' target="_blank" rel="noreferrer">(soon)</a></th>
                </tr>
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
