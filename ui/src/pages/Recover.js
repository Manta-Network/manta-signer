import { useEffect, useState } from "react"
import { Button, Input, Label, TextArea, Form } from 'semantic-ui-react';
import "../fonts/ibm-plex/css/styles.css";

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

    <Form>
      <TextArea
        onChange={(e) => {
          setMnemonics(e.target.value);
        }}
        placeholder="Input Mnemonics"
        className="textarea ui scaled"
      />
    </Form>

    <div>
      {validity ?
        <Button primary className="button ui first" onClick={onClickStartRecover}>Start</Button> :
        <Button disabled primary className="button ui first" onClick={onClickStartRecover}>Start</Button>}
      <Button className="button ui two" onClick={onClickCancel}>Cancel</Button>
    </div>
    {(!validity && mnemonics.length != 0) ? <Label color='red'>Invalid Seed Phrase!</Label> : <br></br>}
  </>);
}

export default Recover;