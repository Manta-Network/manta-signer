import { useState } from 'react';
import { Button, Form, Input, Label, Header } from 'semantic-ui-react';
const bip39 = require('bip39');

const MIN_PASSWORD_LENGTH = 8;

const ResetAccount = ({ sendCreateNew, sendRecoveryInfo, hideWindow }) => {
  const [password, setPassword] = useState('');
  const [recoveryPhrase, setRecoveryPhrase] = useState('');
  const [validSeedPhrase, setValidSeedPhrase] = useState(false);
  const [recoveryPrompt, setRecoveryPrompt] = useState(true);
  const [creatingNew, setCreateNew] = useState(false);
  const [displayRecovery, setDisplayRecovery] = useState(false);

  const isValid = (password) => {
    console.log("[INFO]: Check password validity.")
    return password.length >= MIN_PASSWORD_LENGTH;
  };

  const isValidSeed = (phrase) => {
      return bip39.validateMnemonic(phrase);
  };

  const onEnterSeedPhrase = async (e, data) => {
      setRecoveryPhrase(data.value);
      setValidSeedPhrase(isValidSeed(data.value));
  };

  const onClickCreateNew = async () => {
    console.log("[INFO]: Creating account.")
    if (isValid(password)) {
        const mnemonic = await sendCreateNew(password);
        setRecoveryPhrase(mnemonic);
        setDisplayRecovery(true);
    }
  };

  const onConfirmRecoveryPhrase = async () => {
      setRecoveryPhrase('');
      await hideWindow();
  };

  const setCreateNewPage = async () => {
      setPassword('');
      setRecoveryPhrase('');
      setCreateNew(true);
  };

  const setRecoverPage = async () => {
      setPassword('');
      setRecoveryPhrase('');
      setCreateNew(false);
  };

  const onClickConfirmRecoveryPhrase = async () => {
      const valid = isValidSeed(recoveryPhrase);
      setValidSeedPhrase(valid);
      if (valid) {
          setRecoveryPrompt(false);
      }
  };

  const onClickCreateAccount = async () => {
    console.log("[INFO]: Creating account.")
    if (isValid(password)) {
      await sendRecoveryInfo(recoveryPhrase, password);
      setPassword('');
      setRecoveryPhrase('');
      await hideWindow();
    }
  };

  return (
    <>
      {displayRecovery && (
        <>
          <Header className="recovery-phrase-header">
            Recovery Phrase
          </Header>
          <div className="recovery-phrase-info">
            <p>This phrase can restore your funds if you lose access to your account.</p>
            <p>Write it down on paper and store it somewhere secure.</p>
          </div>
          <div className="recovery-phrase-warning">
            ⚠️  Never share your recovery phrase with anyone! ⚠️ 
          </div>
          <div className="recovery-phrase">
            <b>{recoveryPhrase}</b>
          </div>
          <Button className="button" onClick={onConfirmRecoveryPhrase}>
            I have written down my recovery phrase.
          </Button>
        </>
      )}
      {!displayRecovery && creatingNew && (
        <>
           <Header>Creating New Account</Header>
           <Input
             type="password"
             label="Password"
             onChange={(e) => setPassword(e.target.value)}
           />
           <Button className="button" onClick={setRecoverPage}>Recover From phrase</Button>
           <Button className="button" onClick={onClickCreateNew}>Create</Button>
           {password.length > 0 && !isValid(password) && (<><br/><Label basic color='red' pointing>Please enter a minimum of {MIN_PASSWORD_LENGTH} characters.</Label></>)}
        </>
      )}
      {!displayRecovery && !creatingNew && recoveryPrompt && (
        <>
          <Header className="recovery-phrase-header">
            Enter Recovery Phrase
          </Header>
          <div className="recovery-phrase-warning">
            ⚠️  Never share your recovery phrase with anyone! ⚠️ 
          </div>
          {!validSeedPhrase && recoveryPhrase.length > 0 && <Label basic color='yellow'>Enter a valid seed phrase</Label> }
          {validSeedPhrase && <Label basic color='blue'>Cool! Hit send!</Label> }
          <Form>
              <Form.TextArea onChange={onEnterSeedPhrase}>
              </Form.TextArea>
          </Form>
          <Button className="button" onClick={onClickConfirmRecoveryPhrase}>
              Set Recovery Phrase
          </Button>
          <Button className="button" onClick={setCreateNewPage}>
              Create New Account Instead
          </Button>
        </>
      )}
      {!displayRecovery && !recoveryPrompt && (
          <>
              <Header> Enter Password </Header>
              <Input
                type="password"
                label="Password"
                onChange={(e) => setPassword(e.target.value)}
              />
              <Button className="button" onClick={onClickCreateAccount}>
                Create Account
              </Button>
              {password.length > 0 && !isValid(password) && (<><br/><Label basic color='red' pointing>Please enter a minimum of {MIN_PASSWORD_LENGTH} characters.</Label></>)}
          </>
      )}
    </>
  );
};

export default ResetAccount;
