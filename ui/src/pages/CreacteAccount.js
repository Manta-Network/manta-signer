import React, { useState } from 'react';
import { Button, Input, Header, Container } from 'semantic-ui-react';

const CreateAccount = ({ listen }) => {
  const [password, setPassword] = useState('');
  const [recoveryPhrase, setRecoveryPhrase] = useState('');

  const isValid = (password) => {
    return password.length >= 8;
  };

  const onClickCreateAccount = async () => {
    if (isValid(password)) {
      let recoveryPhrase = await window.__TAURI__.invoke('get_mnemonic', {
        password: password,
      });
      setPassword('');
      setRecoveryPhrase(recoveryPhrase);
    }
  };

  const onClickConfirmRecoveryPhrase = () => {
    setRecoveryPhrase('');
    window.__TAURI__.invoke('end_connect');
    listen();
  };

  return (
    <>
      {recoveryPhrase && (
        <>
          <Header className="recovery-phrase-header">
            {' '}
            Your recovery phrase{' '}
          </Header>
          <div className="recovery-phrase-info">
            This phrase can restore your funds if you lose access to your
            account. Write it down on paper and store it somewhere secure. ⚠️
            Never share your recovery phrase with anyone!
          </div>
          <Container className="recovery-phrase-warning"></Container>
          <div className="recovery-phrase">
            <b>{recoveryPhrase}</b>
          </div>
          <Button className="button" onClick={onClickConfirmRecoveryPhrase}>
            I have written down my recovery phrase
          </Button>
        </>
      )}
      {!recoveryPhrase && (
        <>
          <Header> Create Account </Header>
          <Input
            type="password"
            label="Password"
            onChange={(e) => setPassword(e.target.value)}
          />
          <Button className="button" onClick={onClickCreateAccount}>
            Create Account
          </Button>
        </>
      )}
    </>
  );
};

export default CreateAccount;
