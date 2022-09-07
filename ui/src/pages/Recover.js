import { useState } from "react"
import { Button, Input, Label, Header } from 'semantic-ui-react';

const Recover = (props) => {

  const [mnemonics, setMnemonics] = useState('');

  const onClickStartRecover = async () => {
    
  }

  const onClickCancel = async () => {
    props.cancelRecover();
  }

  return (<>
    <Header>
      Input Mnemonics.
    </Header>

    <Input
      placeholder="Input Mnemonics"
      onChange={(e) => setMnemonics(e.target.value)} 
    />

    <Button primary className="button" onClick={onClickStartRecover}>Start</Button>
    <Button secondary className="button" onClick={onClickCancel}>Cancel</Button>

  </>);
}

export default Recover;