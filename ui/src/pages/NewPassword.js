import { useEffect, useState } from "react"
import { Button, Input, Label, TextArea, Form } from 'semantic-ui-react';
import "../fonts/ibm-plex/css/styles.css";

const NewPassword = (props) => {

  const [newPass, setNewPass] = useState('');
  const [validity, setValidity] = useState(false);

  const MIN_PASSWORD_LENGTH = 8;

  const onClickStartRecover = async () => {

  }

  const validateNewPass = () => {
    let is_valid = newPass.length >= MIN_PASSWORD_LENGTH;

    if (is_valid && (!validity)) {
      setValidity(true);
    } else if (!is_valid && (validity)) {
      setValidity(false);
    }
  }

  useEffect(() => {
    validateNewPass();
  }, [newPass])

  return (<>


    <Input placeholder="New Password"
      onChange={(e) => {
        setNewPass(e.target.value);
      }}
    />

    <div>
      {validity ?
        <Button primary className="button ui first" onClick={onClickStartRecover}>Start</Button> :
        <Button disabled primary className="button ui first" onClick={onClickStartRecover}>Start</Button>}
    </div>
    {(!validity && mnemonics.length != 0) ? <Label color='red'>Invalid Password Length!</Label> : <br></br>}
  </>);
}

export default NewPassword;