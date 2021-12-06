import React, { useState } from 'react';
import { Button, Input, Header, Form } from 'semantic-ui-react';

const SignIn = ({ sendPassword, endInitialConnectionPhase }) => {
  const [password, setPassword] = useState('');
  const [passwordInvalid, setPasswordInvalid] = useState(null);

  const onClickSignIn = async () => {
    const shouldRetry = await sendPassword(password);
    if (!shouldRetry) {
      setPassword('');
      console.log("[INFO]: End Initial Connection Phase");
      await endInitialConnectionPhase();
    } else {
      console.log("RETRY!");
      setPasswordInvalid(true)
    }
  };

  const onChangePassword = password => {
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
      <Button className="button" onClick={onClickSignIn}>
        Sign in
      </Button>
    </div>
  );
};

export default SignIn;
