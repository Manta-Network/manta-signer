import { Button } from 'semantic-ui-react';
import mainLogo from "../icons/manta.png";
import "../App.css";
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

    <div className='mainlogocontainer'>
      <img className="mainlogo"src={mainLogo}/>
    </div>

    <div>
      <h1 className='mainheadline'>Welcome to Manta Signer</h1>
      <p className='subtext'>Use signer to generate zkAddress and control it.</p>
    </div>

    <div className='firstbuttoncontainer'>
      <Button className="button ui first" onClick={onClickStartCreate}>Create New Wallet</Button>
    </div>

    <div>
      <Button className="button ui first" onClick={onClickStartRecover}>I already have a wallet</Button>
    </div>

  </>);
}

export default CreateOrRecover;