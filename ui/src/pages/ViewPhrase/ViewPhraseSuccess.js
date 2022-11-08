import { Button } from 'semantic-ui-react';
import hiddenImage from "../../icons/eye-close.png";
import "../../App.css";

const ViewPhraseSuccess = ({
  recoveryPhraseConfirmed,
  exportedSecretPhrase,
  onClickConfirmRecoveryPhrase,
  onClickFinish
}) => {

  return (<>
    <div className='header-container'>
      <h1 className='main-headline'>Secret Recovery Phrase</h1>
    </div>

    <div className='export-recovery-phrase-container'>
      {recoveryPhraseConfirmed ? exportedSecretPhrase.split(" ").map(function (item, index) {
        let idx = index + 1;
        return (
          <div key={index} className='recovery-phrase-word'>
            <table>
              <tbody>
                <tr>
                  <th><h4>{idx + "."}&nbsp;</h4></th>
                  <th><h4>{item}</h4></th>
                </tr>
              </tbody>
            </table>
          </div>
        )
      }) :
        <div>
          <img className='hide-image-recovery' src={hiddenImage} alt="Hide Logo" onClick={onClickConfirmRecoveryPhrase} />
        </div>
      }
    </div>

    <div className="secret-phrase-warning-container">
      <div className="secret-phrase-warning">
        <p className="secret-phrase-warning-text">Warning: Never share your secret recovery phrase. {<br />} Anyone with your secret recovery phrase can steal assets in your wallet.</p>
      </div>
    </div>

    <Button disabled={!recoveryPhraseConfirmed} className="button ui first wide" onClick={onClickFinish}>Done</Button>
  </>)


}

export default ViewPhraseSuccess;