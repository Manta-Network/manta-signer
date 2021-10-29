import React from 'react';
import { Button, Form, Input, Header } from 'semantic-ui-react';

const CreateAccount = () => {
  return (
    <>
      <Header> Create Account </Header>
      <Input type="password" label="Password" />
      <Button className="button">Create Account</Button>
    </>
  );
};

export default CreateAccount;
