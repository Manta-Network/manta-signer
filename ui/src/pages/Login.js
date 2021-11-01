import React, { useState } from 'react';
import { Button, Input, Header } from 'semantic-ui-react';

const Login = ({ listen, loadPassword }) => {
  const [password, setPassword] = useState('');

  const onClickSignIn = async () => {
    await loadPassword(password);
    setPassword('');
    listen();
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

export default Login;
