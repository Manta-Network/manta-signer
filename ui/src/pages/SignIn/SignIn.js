import React, { useState } from 'react';
import SignInPage from './SignInPage';
import SignInSuccess from './SignInSuccess';

const SignIn = ({
  loginSuccess,
  setLoginSuccess,
  sendSelection,
  getReceivingKeys,
  receivingKey,
  receivingKeyDisplay,
  sendPassword,
  endInitialConnectionPhase,
  startRecover,
  hideWindow,
  loginFailedOccured,
  setLoginFailedOccured
}) => {
  const [password, setPassword] = useState('');
  const [passwordInvalid, setPasswordInvalid] = useState(null);
  const [showCopyNotification, setShowCopyNotification] = useState(false);
  const [loading, setLoading] = useState(false);

  const onClickSignIn = async () => {
    setLoading(true);

    if (!loginFailedOccured) {
      await sendSelection("SignIn");
    }

    const shouldRetry = await sendPassword(password);

    if (!shouldRetry) {

      await getReceivingKeys();
      await endInitialConnectionPhase();
      setPassword('');
      setLoginSuccess(true);
    } else {
      setLoginFailedOccured(true);
      setPasswordInvalid(true);
      console.log("[INFO]: Invalid password, RETRY!");
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
    <SignInPage
      loading={loading}
      password={password}
      passwordInvalid={passwordInvalid}
      onChangePassword={onChangePassword}
      onClickSignIn={onClickSignIn}
      onClickForgotPassword={onClickForgotPassword}
    />
    }
    {loginSuccess &&
      <SignInSuccess
      receivingKeyDisplay={receivingKeyDisplay}
      showCopyNotification={showCopyNotification}
      onClickCopyZkAddress={onClickCopyZkAddress}
      onClickFinishSignIn={onClickFinishSignIn}
      />
    }
  </>);
};

export default SignIn;
