import { useEffect, useState } from "react"
import { Button, Input, Label, TextArea, Form, Header } from 'semantic-ui-react';
import "../App.css";

const bip39 = require('bip39');

const Recover = (props) => {

  const [mnemonics, setMnemonics] = useState('');
  const [validity, setValidity] = useState(false);
  const [newPass, setNewPass] = useState('');
  const [newPassValidity, setNewPassValidity] = useState(false)

  const [showNewPassPage, setShowNewPassPage] = useState(false);


  const MIN_PASSWORD_LENGTH = 8;

  const onClickRecover = async () => {
    await props.sendPassword(newPass);
    props.endInitialConnectionPhase();
    props.hideWindow();
  }

  const onClickNewPass = async () => {
    setShowNewPassPage(true);
    await props.sendMnemonic(mnemonics);
  }

  const onClickCancel = async () => {
    props.restartServer();
  }

  const validateMnemonics = () => {
    let is_valid = bip39.validateMnemonic(mnemonics);

    if (is_valid && (!validity)) {
      setValidity(true);
    } else if (!is_valid && (validity)) {
      setValidity(false);
    }
  }

  const validateNewPass = () => {
    let is_valid = newPass.length >= MIN_PASSWORD_LENGTH;

    if (is_valid && (!newPassValidity)) {
      setNewPassValidity(true);
    } else if (!is_valid && (newPassValidity)) {
      setNewPassValidity(false);
    }
  }

  useEffect(() => {
    validateMnemonics();
  }, [mnemonics])

  useEffect(() => {
    validateNewPass();
  }, [newPass])

  return (<>

    {showNewPassPage ?
      <>
        <Header>Pick a new password</Header>
        <Input
          placeholder="New Password"
          type="password"
          onChange={(e) => {
            setNewPass(e.target.value);
          }}
        ></Input>
        <div>
          {newPassValidity ?
            <Button primary className="button ui first" onClick={onClickRecover}>Start</Button> :
            <Button disabled primary className="button ui first">Start</Button>}
        </div>
        {(!newPassValidity && newPass.length != 0) ? <Label color='red'>Invalid Password Length!</Label> : <br></br>}
      </> :
      <>
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
            <Button primary className="button ui first" onClick={onClickNewPass}>Continue</Button> :
            <Button disabled primary className="button ui first">Continue</Button>}
          <Button className="button ui two" onClick={onClickCancel}>Cancel</Button>
        </div>
        {(!validity && mnemonics.length != 0) ? <Label color='red'>Invalid Seed Phrase!</Label> : <br></br>}
      </>}

  </>);
}

export default Recover;