import { Button } from 'semantic-ui-react';
import HyperLinkButton from '../../components/HyperLinkButton';
import "../../App.css";
import { useOutletContext } from 'react-router-dom';
import hiddenImage from "../../icons/eye-close.png";
const ShowPhrase = () => {

  const {
    goBack,
    goForward,
    onClickConfirmRecoveryPhrase,
    recoveryPhraseConfirmed,
    recoveryPhrase,
  } = useOutletContext();

  return (
    <>
      <div className='headercontainer'>
        <h1 className='mainheadline'>Secret Recovery Phrase</h1>
      </div>
      <div className="recovery-phrase-info">
        <p>Please write down your secret recovery phrase and keep it in a safe place.</p>
        <p>This phrase is the only way to recover your wallet. Do not share it with anyone!</p>
      </div>

      <div className='recoveryPhraseContainer'>
        {recoveryPhraseConfirmed ? recoveryPhrase.split(" ").map(function (item, index) {
          let idx = index+1;
          return (
            <div key={index} className='recoveryPhraseWord'>
              <tr>
                <th><h4>{idx+"."}&nbsp;</h4></th>
                <th><h4>{item}</h4></th>
              </tr>
            </div>
          )
        }) :
          <div>
            <img className='hideImage' src={hiddenImage} alt="hidden" onClick={onClickConfirmRecoveryPhrase} />
          </div>
        }
      </div>

      <Button disabled={!recoveryPhraseConfirmed} className="button ui first wide" onClick={goForward}>
        Next
      </Button>
      <HyperLinkButton
        text={"Go Back"}
        onclick={goBack}
      />
    </>
  )
}

export default ShowPhrase;