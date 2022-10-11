import React, { useState } from 'react';
import { Button, Input, Form, Icon } from 'semantic-ui-react';
import mantaLogo from "../icons/manta.png";
import dolphinLogo from "../icons/Square150x150Logo.png";
import calamariLogo from "../icons/calamari.png";
import newAccount from "../icons/new_account.png";
import "../App.css";
import HyperLinkButton from '../components/HyperLinkButton';

const SignIn = (props) => {
  const [password, setPassword] = useState('');
  const [passwordInvalid, setPasswordInvalid] = useState(null);
  const [loginSuccess, setLoginSuccess] = useState(false);

  const onClickSignIn = async () => {
    const shouldRetry = await props.sendPassword(password);

    if (!shouldRetry) {
      await props.getReceivingKeys();
    }

    if (!shouldRetry) {
      setPassword('');
      setLoginSuccess(true);
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
    navigator.clipboard.writeText(props.receivingKey);
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
          <p className='subtext'>{props.receivingKeyDisplay}</p>
          <Button onClick={onClickCopyZkAddress} className='button ui copy'>
            <Icon name="copy outline" className='specific' />
          </Button>
        </div>
        <div className='supportedNetworksContainer'>
          <div className='supportedNetworksChild'>
            <div className='supportedNetworksHeader'>
              <h4>Supported Networks</h4>
            </div>
            <img className='miniLogo' alt="Calamari Logo" src={calamariLogo} />
            <a className='soonTag' href='https://calamari.network/' target="_blank" rel="noreferrer">(soon)</a>
            <img className='miniLogo' alt="Dolphin Logo" src={dolphinLogo} />
            <img className='miniMantaLogo' alt="Manta Logo" src={mantaLogo} />
            <a className='soonTag' href='https://www.manta.network/' target="_blank" rel="noreferrer">(soon)</a>
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
