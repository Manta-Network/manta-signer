import { useState } from "react"
import { Button, Input, Label, Header } from 'semantic-ui-react';
import mainLogo from "../icons/Square150x150Logo.png"
import "../fonts/ibm-plex/css/styles.css";
const CreateOrRecover = (props) => {

  const onClickStartCreate = async () => {
    await props.sendCreateOrRecover("Create");
    props.startCreate();
  }

  const onClickStartRecover = async () => {
    await props.sendCreateOrRecover("Recover");
    props.startRecover();
  }

  return (<>
    <div>
      <img draggable="false" unselectable="on" dragstart="false" src={mainLogo} />
    </div>

    <div>
      <Button className="button ui first" onClick={onClickStartCreate}>Create New Wallet</Button>
    </div>
    <a className="secondary-anchor"><h4 onClick={onClickStartRecover}>Recover Wallet</h4></a>
  </>);
}

export default CreateOrRecover;