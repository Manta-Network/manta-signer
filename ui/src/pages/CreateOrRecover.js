import { Button } from 'semantic-ui-react';
import mainLogo from "../icons/manta.png";
import "../App.css";
const CreateOrRecover = ({
  startCreate,
  startRecover
}) => {

  const onClickStartCreate = async () => {
    startCreate();
  }

  const onClickStartRecover = async () => {
    startRecover();
  }

  return (<>

    <div className='main-logo-container'>
      <img className="main-logo" alt="Manta Logo" src={mainLogo} />
    </div>

    <div>
      <h1 className='main-headline'>Welcome to Manta Signer</h1>
      <p className='sub-text'>The signer is a secret manager and zero-knowledge proof generator.</p>
    </div>

    <div className='first-button-container'>
      <Button className="button ui first" onClick={onClickStartCreate}>Create New Wallet</Button>
    </div>

    <div className='bottom-button-container'>
      <Button className="button ui first" onClick={onClickStartRecover}>I already have a wallet</Button>
    </div>

    <div className='learn-about-manta'>
      <p>Learn more about &nbsp;
        <a href='https://www.manta.network/' target="_blank" rel="noreferrer">
          Manta
        </a>
      </p>
    </div>

  </>);
}

export default CreateOrRecover;