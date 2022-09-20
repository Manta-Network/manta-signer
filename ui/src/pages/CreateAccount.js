import { useEffect, useState } from 'react';
import { Button, Input, Label, Header } from 'semantic-ui-react';
import "../App.css";

const MIN_PASSWORD_LENGTH = 8;

const CreateAccount = (props) => {
  const [password, setPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [createdAccount, setCreatedAccount] = useState(false);
  const [passwordsMatch, setPasswordsMatch] = useState(false);
  const [isValidPassword, setIsValidPassword] = useState(false);

  const isValid = (password) => {
    console.log("[INFO]: Check password validity.");
    return password.length >= MIN_PASSWORD_LENGTH;
  };

  const onClickCreateAccount = async () => {
    console.log("[INFO]: Creating account.");
    if (isValid(password)) {
      await props.sendPassword(password);
      setPassword('');
      setCreatedAccount(true);
    }
  };

  const onClickCancel = async () => {
    console.log("[INFO]: Cancel account creation.");
    await props.restartServer();
  }

  const onClickConfirmRecoveryPhrase = async () => {
    console.log("[INFO]: Confirming recovery phrase.")
    await props.endInitialConnectionPhase();
  };

  useEffect(() => {

    if ((password === confirmPassword) && !passwordsMatch) {
      setPasswordsMatch(true);
    } else if (!(password === confirmPassword) && (passwordsMatch)) {
      setPasswordsMatch(false);
    }

    if (isValid(password) && !isValidPassword) {
      setIsValidPassword(true);
    } else if (!isValid(password) && isValidPassword) {
      setPasswordsMatch(false);
    }

  }, [password,confirmPassword])

  return (
    <>
      {!createdAccount && (
        <>
          <div className='headercontainer'>
            <h1 className='mainheadline'>Create a password</h1>
            <p className='subtext'>You will use it to unlock your wallet.</p>
          </div>
          <br/>
          <br/>
          <div>
            <Input
              className='input ui password'
              type="password"
              placeholder="Password (8 characters min)"
              onChange={(e) => setPassword(e.target.value)}
            />
          </div>
          <div>
            <Input
              className='input ui password'
              type="password"
              placeholder="Confirm Password"
              onChange={(e) => setConfirmPassword(e.target.value)}
            />
          </div>
          <Button disabled={!(isValid(password) && passwordsMatch)} className="button ui first" onClick={onClickCreateAccount}>
            Next
          </Button>
          <div>
            <a onClick={onClickCancel}>Go Back</a>
          </div>

          {!isValidPassword && password.length > 0 ? <><br /><Label basic color='red' pointing>Please enter a minimum of {MIN_PASSWORD_LENGTH} characters.</Label></> : (
            !passwordsMatch ? <><br /><Label basic color='red' pointing>Passwords do not match.</Label></> : <><br /><br /><br /></>
          )}

        </>
      )}
      {createdAccount && (
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
            <b>{props.recoveryPhrase}</b>
          </div>
          <Button className="button" onClick={onClickConfirmRecoveryPhrase}>
            I have written down my recovery phrase.
          </Button>
        </>
      )}
    </>
  );
};

export default CreateAccount;
