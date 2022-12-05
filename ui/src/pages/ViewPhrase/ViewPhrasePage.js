
import { Button, Form, Input } from 'semantic-ui-react';
import mainLogo from "../../icons/manta.png";
import "../../App.css";

const ViewPhrasePage = ({
  loading,
  exportedSecretPhrase,
  password,
  passwordInvalid,
  onChangePassword,
  onClickSubmitPassword,
  onClickCancel
}) => {


  return (<>

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

  </>)


}

export default ViewPhrasePage;