import React, { useState } from 'react';
import { Button, Input, Header, Form, Label } from 'semantic-ui-react';

const SignIn = ({ sendPassword, endInitialConnectionPhase }) => {
  const [password, setPassword] = useState('');
  const [passwordInvalid, setPasswordInvalid] = useState(null);
  const [badPasswordTried, setBadPasswordTried] = useState(false);

  const onClickSignIn = async () => {
    const shouldRetry = await sendPassword(password);
    if (!shouldRetry) {
      setPassword('');
      console.log("[INFO]: End Initial Connection Phase");
      await endInitialConnectionPhase();
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

  return (
    <div>
      <Header> Sign in </Header>
      <Form.Field>
        <Input
          type="password"
          placeholder="Password"
          value={password}
          onChange={(e) => onChangePassword(e.target.value)}
          error={passwordInvalid}
        />
      </Form.Field>
      { badPasswordTried && <><Label basic color='red' pointing>Wrong Password</Label><br/></> }
      <Button className="button" onClick={onClickSignIn}>
        Sign in
      </Button>
    </div>
  );
};

export default SignIn;
