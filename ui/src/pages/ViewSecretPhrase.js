import { useState } from "react";
import { Button, Form, Input } from 'semantic-ui-react';
import mainLogo from "../icons/manta.png";
import hiddenImage from "../icons/eye-close.png";
import "../App.css";

const ViewSecretPhrase = ({
  endExportPhrase,
  exportedSecretPhrase,
  sendPassword,
  stopPasswordPrompt
}) => {

  const [password, setPassword] = useState("");
  const [passwordInvalid, setPasswordInvalid] = useState(false);
  const [recoveryPhraseConfirmed, setRecoveryPhraseConfirmed] = useState(false);
  const [loading, setLoading] = useState(false);

  const onChangePassword = password => {
    setPassword(password)
    setPasswordInvalid(false)
  }

  const onClickCancel = async () => {
    await stopPasswordPrompt();
    endExportPhrase();
  }

  const onClickSubmitPassword = async () => {
    setLoading(true);
    const shouldRetry = await sendPassword(password);

    if (!shouldRetry) {
      setPassword('');
      setPasswordInvalid(false);
    } else {
      console.log("[INFO]: Invalid password, RETRY!");
      setPasswordInvalid(true);
      setLoading(false);
    }
  }

  // This function enables the Next button to continue in the account creation
  // process once the user has read the recovery phrase.
  const onClickConfirmRecoveryPhrase = () => {
    setRecoveryPhraseConfirmed(true);
  }

  const onClickFinish = () => {
    setRecoveryPhraseConfirmed(false);
    endExportPhrase();
    setLoading(false);
  }

  return (<>
    {!exportedSecretPhrase && (<>

      <div className='main-logo-container view-phrase'>
        <img className="main-logo" alt="Manta Logo" src={mainLogo} />
      </div>

      <div>
        <h1 className='main-headline'>Show Secret Recovery Phrase</h1>
      </div>

      <Form.Field>
        <Input
          className='input ui password tall wide'
          type="password"
          placeholder="Password"
          value={password}
          onChange={(e) => onChangePassword(e.target.value)}
          error={passwordInvalid}
        />
      </Form.Field>

      <div className="secret-phrase-warning-container">
        <div className="secret-phrase-warning">
          <p className="secret-phrase-warning-text">Warning: Never share your secret recovery phrase. {<br />} Anyone with your secret recovery phrase can steal your assets in your wallet.</p>
        </div>
      </div>


      {(loading && !exportedSecretPhrase) ?
        <div>
          <Button disabled={true} className="button ui first wide">Loading</Button>
        </div>
        :
        <div>
          <Button className="button ui first wide" onClick={onClickSubmitPassword}>Confirm</Button>
        </div>
      }
      <div className="cancel-show-recovery-button-container">
        <Button className="button ui cancel" onClick={onClickCancel}>Cancel</Button>
      </div>

    </>)}
    {exportedSecretPhrase && (<>
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
            <img className='hide-image' src={hiddenImage} alt="Hide Logo" onClick={onClickConfirmRecoveryPhrase} />
          </div>
        }
      </div>

      <div className="secret-phrase-warning-container">
        <div className="secret-phrase-warning">
          <p className="secret-phrase-warning-text">Warning: Never share your secret recovery phrase. {<br />} Anyone with your secret recovery phrase can steal assets in your wallet.</p>
        </div>
      </div>

      <Button disabled={!recoveryPhraseConfirmed} className="button ui first wide" onClick={onClickFinish}>Done</Button>
    </>)}
  </>);
}

export default ViewSecretPhrase