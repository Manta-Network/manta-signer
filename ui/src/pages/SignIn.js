import React, { useState } from 'react';
import { Button, Input, Header } from 'semantic-ui-react';

const SignIn = ({ loadPasswordToSignerServer, endInitialConnectionPhase }) => {
  const [password, setPassword] = useState('');

  const onClickSignIn = async () => {
    await loadPasswordToSignerServer(password);
    setPassword('');
    await endInitialConnectionPhase();
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
