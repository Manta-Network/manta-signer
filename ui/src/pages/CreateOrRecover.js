import { Button } from 'semantic-ui-react';
import mainLogo from "../icons/manta.png";
import "../App.css";
const CreateOrRecover = ({
  sendCreateOrRecover,
  startCreate,
  startRecover
}) => {

  const onClickStartCreate = async () => {
    await sendCreateOrRecover("Create");
    startCreate();
  }

  const onClickStartRecover = async () => {
    await sendCreateOrRecover("Recover");
    startRecover();
  }

  return (<>

    <div className='mainlogocontainer'>
      <img className="mainlogo" alt="Manta Logo" src={mainLogo} />
    </div>

    <div>
      <h1 className='mainheadline'>Welcome to Manta Signer</h1>
      <p className='subtext'>The signer is a secret manager and zero-knowledge proof generator.</p>
    </div>

    <div className='firstbuttoncontainer'>
      <Button className="button ui first" onClick={onClickStartCreate}>Create New Wallet</Button>
    </div>

    <div className='bottomButtonContainer'>
      <Button className="button ui first" onClick={onClickStartRecover}>I already have a wallet</Button>
    </div>

    <div className='learnAboutManta'>
      <p>Learn more about &nbsp;
        <a href='https://www.manta.network/' target="_blank" rel="noreferrer">
          Manta
        </a>
      </p>
    </div>

  </>);
}

export default CreateOrRecover;