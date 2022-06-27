import { useState } from 'react';
import { Button, Form, Input, Label, Header } from 'semantic-ui-react';
const bip39 = require('bip39');

const MIN_PASSWORD_LENGTH = 8;

const ResetAccount = ({ sendRecoveryInfo, hideWindow }) => {
  const [password, setPassword] = useState('');
  const [recoveryPhrase, setRecoveryPhrase] = useState('');
  const [validSeedPhrase, setValidSeedPhrase] = useState(false);
  const [recoveryPrompt, setRecoveryPrompt] = useState(true);

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
      {recoveryPrompt && (
        <>
          <Header className="recovery-phrase-header">
            Enter Recovery Phrase
          </Header>
          <div className="recovery-phrase-warning">
            ⚠️  Never share your recovery phrase with anyone! ⚠️ 
          </div>
          {!validSeedPhrase && <Label basic color='yellow'>Enter a valid seed phrase</Label> }
          {validSeedPhrase && <Label basic color='blue'>Cool! Hit send!</Label> }
          <Form>
              <Form.TextArea onChange={onEnterSeedPhrase}>
              </Form.TextArea>
          </Form>
          <Button className="button" onClick={onClickConfirmRecoveryPhrase}>
              Set Recovery Phrase
          </Button>
        </>
      )}
      {!recoveryPrompt && (
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
