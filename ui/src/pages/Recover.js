import { useEffect, useState } from "react"
import { Button, Input, Label, Header } from 'semantic-ui-react';

const bip39 = require('bip39');

const Recover = (props) => {

  const [mnemonics, setMnemonics] = useState('');
  const [validity, setValidity] = useState(false);

  const onClickStartRecover = async () => {

  }

  const onClickCancel = async () => {
    props.cancelRecover();
  }

  const validateMnemonics = () => {
    let is_valid = bip39.validateMnemonic(mnemonics);

    if (is_valid && (!validity)) {
      setValidity(true);
    } else if (!is_valid && (validity)) {
      setValidity(false);
    }
  }

  useEffect(() => {
    validateMnemonics();
  }, [mnemonics])

  return (<>
    <Header>
      Input Mnemonics.
    </Header>

    <Input
      placeholder="Input Mnemonics"
      onChange={(e) => {
        setMnemonics(e.target.value);
      }}
    />

    <div>
      {validity ?
        <Button primary className="button" onClick={onClickStartRecover}>Start</Button> :
        <Button disabled primary className="button" onClick={onClickStartRecover}>Start</Button>}
      <Button secondary className="button" onClick={onClickCancel}>Cancel</Button>
    </div>
    {(!validity && mnemonics.length != 0) ? <Label color='red'>Invalid Seed Phrase!</Label> : null}
  </>);
}

export default Recover;