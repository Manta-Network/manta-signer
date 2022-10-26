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
    const shouldRetry = await sendPassword(password);

    if (!shouldRetry) {
      setPassword('');
      setPasswordInvalid(false);
      setLoading(true);
    } else {
      console.log("RETRY!");
      setPasswordInvalid(true);
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

      <div className='mainlogocontainer viewPhrase'>
        <img className="mainlogo" alt="Manta Logo" src={mainLogo} />
      </div>

      <div>
        <h1 className='mainheadline'>Show Secret Recovery Phrase</h1>
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

      <div className="secretPhraseWarningContainer">
        <div className="secretPhraseWarning">
          <p className="secretPhraseWarningText">Warning: Never share your secret recovery phrase. {<br />} Anyone with your secret recovery phrase can steal your assets in your wallet.</p>
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
      <div className="cancelShowRecoveryButtonContainer">
        <Button className="button ui cancel" onClick={onClickCancel}>Cancel</Button>
      </div>

    </>)}
    {exportedSecretPhrase && (<>
      <div className='headercontainer'>
        <h1 className='mainheadline'>Secret Recovery Phrase</h1>
      </div>

      <div className='recoveryPhraseContainer'>
        {recoveryPhraseConfirmed ? exportedSecretPhrase.split(" ").map(function (item, index) {
          return (
            <div key={index} className='recoveryPhraseWord'>
              <h4>{item}</h4>
            </div>
          )
        }) :
          <div>
            <img className='hideImage' src={hiddenImage} alt="Hide Logo" onClick={onClickConfirmRecoveryPhrase} />
          </div>
        }
      </div>

      <div className="secretPhraseWarningContainer">
        <div className="secretPhraseWarning">
          <p className="secretPhraseWarningText">Warning: Never share your secret recovery phrase. {<br />} Anyone with your secret recovery phrase can steal assets in your wallet.</p>
        </div>
      </div>

      <Button disabled={!recoveryPhraseConfirmed} className="button ui first wide" onClick={onClickFinish}>Done</Button>
    </>)}
  </>);
}

export default ViewSecretPhrase