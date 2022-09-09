import { useState } from "react"
import { Button, Input, Label, Header } from 'semantic-ui-react';
import "../fonts/ibm-plex/css/styles.css";

const Reset = (props) => {

  const onClickReset = async () => {
    props.resetAccount();
  }

  const onClickCancel = async () => {
    props.cancelReset();
  }

  return (<>
    <Button className="button ui danger" onClick={onClickReset}>DELETE ACCOUNT</Button>
    <div>
      <Button className="button ui two" onClick={onClickCancel}>Cancel</Button>
    </div>
  </>);
}

export default Reset;