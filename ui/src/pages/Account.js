import React, { useState, useEffect } from 'react';
import { Button, Header } from 'semantic-ui-react';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faCopy, faCheck } from '@fortawesome/free-solid-svg-icons';

const Account = ({receivingKeys, hideWindow}) => {
  const receivingKey = receivingKeys[0];
  const receivingKeyDisplay = receivingKey ?
    `${receivingKey.slice(0, 10)}...${receivingKey.slice(-10)}` :
    '';

  const onClickClose = () => {
    hideWindow()
  }

  return (
    <>
      <Header>Account</Header>
      <div className='account-address-container'>
        <CopyIcon receivingKey={receivingKey} />
        <div className='account-address' title={receivingKey} >{receivingKeyDisplay}</div>
      </div>
      <Button className="button" onClick={onClickClose}>
        Close
      </Button>
    </>
  );
};

const CopyIcon = ({receivingKey}) => {
  const [addressCopied, setAddressCopied] = useState(false);

  const copyToClipboard = (e) => {
    navigator.clipboard.writeText(receivingKey);
    setAddressCopied(true);
    e.stopPropagation();
    return false;
  };

  useEffect(() => {
    const timer = setTimeout(
      () => addressCopied && setAddressCopied(false),
      1500
    );
    return () => clearTimeout(timer);
  }, [addressCopied]);

  return (
    <div>
      {addressCopied ? (
        <FontAwesomeIcon icon={faCheck} />
      ) : (
        <FontAwesomeIcon
          className='account-address-copy-icon'
          icon={faCopy}
          onMouseDown={(e) => copyToClipboard(e)}
        />
      )}
    </div>
  )
}

export default Account;
