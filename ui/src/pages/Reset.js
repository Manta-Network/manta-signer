import { useState } from "react"
import { Button, Input, Label, Header } from 'semantic-ui-react';

const Reset = (props) => {

  const onClickReset = async () => {
    props.resetAccount();
  }

  return (<>
    <Button color='red' onClick={onClickReset}>DELETE ACCOUNT</Button>
  </>);
}

export default Reset;