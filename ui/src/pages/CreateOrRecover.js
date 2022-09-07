import { useState } from "react"
import { Button, Input, Label, Header } from 'semantic-ui-react';

const CreateOrRecover = (props) => {

  const onClickStartCreate = async () => {
    props.startCreate();
  }

  const onClickStartRecover = async () => {
    props.startRecover();
  }

  return (<>
    <Header>
      Create or recover wallet
    </Header>

    <Button primary className="button" onClick={onClickStartCreate}>Create New Wallet</Button>
    <Button secondary className="button" onClick={onClickStartRecover}>Recover Wallet</Button>

  </>);
}

export default CreateOrRecover;