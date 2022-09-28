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
      <img className="mainlogo" alt="Manta Logo" src={mainLogo} />
    </div>

    <div>
      <h1 className='mainheadline'>Welcome to Manta Signer</h1>
      <p className='subtext'>Use signer to generate zkAddress and control it.</p>
    </div>

    <div className='firstbuttoncontainer'>
      <Button className="button ui first" onClick={onClickStartCreate}>Create New Wallet</Button>
    </div>

    <div className='bottomButtonContainer'>
      <Button className="button ui first" onClick={onClickStartRecover}>I already have a wallet</Button>
    </div>

    <div className='learnAboutManta'>
      <p>Learn more about &nbsp;
        <a href='https://www.manta.network/' target="_blank">
           Manta
        </a>
      </p>
    </div>

  </>);
}

export default CreateOrRecover;