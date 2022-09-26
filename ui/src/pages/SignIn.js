import React, { useState } from 'react';
import { Button, Input, Header, Form, Label } from 'semantic-ui-react';
import mantaLogo from "../icons/manta.png";
import dolphinLogo from "../icons/Square150x150Logo.png";
import calamariLogo from "../icons/calamari.png";
import newAccount from "../icons/new_account.png";
import "../App.css";
const SignIn = (props) => {
  const [password, setPassword] = useState('');
  const [passwordInvalid, setPasswordInvalid] = useState(null);
  const [badPasswordTried, setBadPasswordTried] = useState(false);
  const [loginSuccess, setLoginSuccess] = useState(false);


  const onClickSignIn = async () => {
    const shouldRetry = await props.sendPassword(password);
    if (!shouldRetry) {
      setPassword('');
      setLoginSuccess(true);
    } else {
      console.log("RETRY!");
      setPasswordInvalid(true);
      setBadPasswordTried(true);
    }
  };

  const onChangePassword = password => {
    setBadPasswordTried(false);
    setPassword(password)
    setPasswordInvalid(false)
  }

  const onClickForgotPassword = async () => {
    props.startRecover();
  }

  const onClickFinishSignIn = async () => {
    console.log("[INFO]: End Initial Connection Phase");
    await props.endInitialConnectionPhase();
  }

  return (<>
    {!loginSuccess &&
      <div>
        <div className='mainlogocontainer login'>
          <img className="mainlogo" src={mantaLogo} />
        </div>

        <div>
          <h1 className='mainheadline'>Welcome Back!</h1>
          <p className='subtext'>Connect to ZK world!</p>
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
        <div className='bottomButtonContainer'>
          <a onClick={onClickForgotPassword}>Forgot Password?</a>
        </div>

        <div className='learnAboutMantaSmall'>
          <p>Learn more about &nbsp;
            <a href='https://www.manta.network/' target="_blank">
              Manta
            </a>
          </p>
        </div>
      </div>
    }
    {loginSuccess &&
      <div>
        <div className='finishLogoContainer'>
          <img className="mainlogo" src={newAccount} />
        </div>

        <div>
          <h1 className='mainheadline'>Your zkAddress</h1>
          <p className='subtext'>***Address Placeholder***</p>
        </div>
        <div className='supportedNetworksContainer'>
          <div className='supportedNetworksChild'>
            <div className='supportedNetworksHeader'>
              <h4>Supported Networks</h4>
            </div>
            <img className='miniLogo' src={calamariLogo} />
            <a className='soonTag'>(soon)</a>
            <img className='miniLogo' src={dolphinLogo} />
            <img className='miniMantaLogo' src={mantaLogo} />
            <a className='soonTag'>(soon)</a>
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
