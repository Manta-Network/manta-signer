import { Button } from 'semantic-ui-react';
import mainLogo from "../icons/Square150x150Logo.png";
import "../fonts/ibm-plex/css/styles.css";
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
    <div>
      <img draggable="false" unselectable="on" dragstart="false" src={mainLogo} alt={"Manta Logo"} />
    </div>

    <div>
      <Button className="button ui first" onClick={onClickStartCreate}>Create New Wallet</Button>
    </div>
    <div className='recoverWalletContainer'>
      <Button className='button ui recoverWallet' onClick={onClickStartRecover}>Recover Wallet</Button>
    </div>
  </>);
}

export default CreateOrRecover;