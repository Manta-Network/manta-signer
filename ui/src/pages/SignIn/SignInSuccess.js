import { Button } from 'semantic-ui-react';
import mantaLogo from "../../icons/manta.png";
import dolphinLogo from "../../icons/Square150x150Logo.png";
import calamariLogo from "../../icons/calamari.png";
import newAccount from "../../icons/new_account.png";
import CopyButton from '../../components/copyButton';
import "../../App.css";

const SignInSuccess = ({
  receivingKeyDisplay,
  showCopyNotification,
  onClickCopyZkAddress,
  onClickFinishSignIn
}) => {

  return (<div>
    <div className='finish-logo-container'>
      <img className="main-logo" alt="Manta Logo" src={newAccount} />
    </div>

    <div>
      <h1 className='main-headline'>Your zkAddress</h1>
    </div>
    <div className='zk-address-container'>
      <p className='sub-text'>{receivingKeyDisplay}</p>
      <CopyButton
        showCopyNotification={showCopyNotification}
        onClickCopyPhrase={onClickCopyZkAddress}
      />
    </div>
    <div className='supported-networks-container'>
      <div className='supported-networks-child'>
        <div className='supported-networks-header'>
          <h4>Supported Networks</h4>
        </div>

        <table className='network-table'>
          <tbody>
            <tr>
              <th><img className='mini-dolphin-logo' alt="Dolphin Logo" src={dolphinLogo} /></th>
              <th><p className='network-text'>Dolphin Network</p></th>
              <th></th>
            </tr>
            <tr>
              <th><img className='mini-calamari-logo' alt="Calamari Logo" src={calamariLogo} /></th>
              <th><p className='network-text'>&nbsp;Calamari Network&nbsp;&nbsp;</p></th>
              <th><a href='https://calamari.network/' target="_blank" rel="noreferrer">(soon)</a></th>
            </tr>
            <tr>
              <th><img className='mini-manta-logo' alt="Manta Logo" src={mantaLogo} /></th>
              <th><p className='network-text'>Manta Network</p></th>
              <th><a href='https://calamari.network/' target="_blank" rel="noreferrer">(soon)</a></th>
            </tr>
          </tbody>
        </table>
      </div>

    </div>
    <Button className="button ui first" onClick={onClickFinishSignIn}>
      Start
    </Button>
  </div>)
}

export default SignInSuccess;