import React, { useState } from 'react';
import { Button, Input, Header } from 'semantic-ui-react';

const SignIn = ({ sendPassword, endInitialConnectionPhase }) => {
  const [password, setPassword] = useState('');

  const onClickSignIn = async () => {
    console.log("[INFO]: Sign In.");
    const shouldRetry = await sendPassword(password);
    setPassword('');
    // FIXME: Clear the input element too.
    if (!shouldRetry) {
      console.log("[INFO]: End Initial Connection Phase");
      await endInitialConnectionPhase();
    } else {
      console.log("RETRY!");
    }
  };

  return (
    <>
      <Header> Sign in </Header>
      <Input
        type="password"
        label="Password"
        onChange={(e) => setPassword(e.target.value)}
        value={password}
      />
      <Button className="button" onClick={onClickSignIn}>
        Sign in
      </Button>
    </>
  );
};

export default SignIn;
