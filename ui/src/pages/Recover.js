import { useEffect, useState } from "react"
import { Button, Input, Label, Header } from 'semantic-ui-react';

const Recover = (props) => {

  const [mnemonics, setMnemonics] = useState('');
  const [validity, setValidity] = useState(false);
  const VALID_LENGTH = 24;

  const onClickStartRecover = async () => {

  }

  const onClickCancel = async () => {
    props.cancelRecover();
  }

  const validateMnemonics = () => {
    let mnemonics_length = mnemonics.split(" ").length;

    if (mnemonics_length == VALID_LENGTH && (!validity)) {
      setValidity(true);
    } else if ((mnemonics_length != VALID_LENGTH) && (validity)) {
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