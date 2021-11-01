import React, { useState } from 'react';
import { Button, Header, Input, Container } from 'semantic-ui-react';

const AuthorizeTx = ({ txSummary, disconnect, listen, loadPassword }) => {
  let summaryMessage = '';

  if (txSummary && typeof txSummary !== 'string') {
    const { amount, currency_symbol, kind } = txSummary;
    const kindMessage = typeof kind === 'string' ? kind : 'Transfer';
    const recipient =
      typeof kind === 'string'
        ? 'your public wallet'
        : kind.PrivateTransfer.recipient;
    summaryMessage = `${kindMessage} ${amount} ${currency_symbol} to: ${recipient}`;
  }

  const [password, setPassword] = useState('');

  const onClickAuthorize = async () => {
    await loadPassword(password);
    listen();
  };

  const onClickDecline = async () => {
    await disconnect();
  };

  return (
    <>
      <Header> Authorize Transaction </Header>
      <Container className="tx-summary">{summaryMessage}</Container>
      <Input
        type="password"
        label="Password"
        onChange={(e) => setPassword(e.target.value)}
      />
      <Button className="button" onClick={onClickAuthorize}>
        Authorize
      </Button>
      <Button className="button" onClick={onClickDecline}>
        Decline
      </Button>
    </>
  );
};

export default AuthorizeTx;
