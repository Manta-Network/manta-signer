import { useEffect, useState } from 'react';
import { Button, Input, Label } from 'semantic-ui-react';
import "../App.css";
import hiddenImage from "../icons/eye-close.png";
import { appWindow, LogicalSize } from '@tauri-apps/api/window';
import Loading from './Loading';
import HyperLinkButton from '../components/HyperLinkButton';

const MIN_PASSWORD_LENGTH = 8;
const PASSWORD_TAB = 0;
const SHOW_PHRASE_TAB = 1;
const CONFIRM_PHRASE_TAB = 2;
const FINAL_TAB = 3;
const LOADING_TAB = 4;

const DEFAULT_WINDOW_SIZE = new LogicalSize(460, 500);
const CONFIRM_PHRASE_WINDOW_SIZE = new LogicalSize(460, 900);

const CreateAccount = ({
  recoveryPhrase,
  sendPassword,
  restartServer
}) => {
  const [password, setPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [passwordsMatch, setPasswordsMatch] = useState(false);
  const [isValidPassword, setIsValidPassword] = useState(false);
  const [currentTab, setCurrentTab] = useState(PASSWORD_TAB);
  const [recoveryPhraseConfirmed, setRecoveryPhraseConfirmed] = useState(false);
  const [isValidSelectedPhrase, setIsValidSelectedPhrase] = useState(false);
  const [shuffledRecoveryPhrase, setShuffledRecoveryPhrase] = useState(null);
  const [selectedRecoveryPhrase, setSelectedRecoveryPhrase] = useState([]);
  const [actualPhrase, setActualPhrase] = useState(null);

  // onClickSelectWordButton is called whenever a word selection button
  // gets clicked when the user is reciting their seed phrase.
  const onClickSelectWordButton = (e, word) => {

    if (selectedRecoveryPhrase.includes(word)) {

      const newSelectedRecoveryPhrase = [...selectedRecoveryPhrase];
      newSelectedRecoveryPhrase.splice(newSelectedRecoveryPhrase.indexOf(word), 1);
      setSelectedRecoveryPhrase(newSelectedRecoveryPhrase);
      e.target.style.backgroundColor = "#FFFFFF";
      e.target.style.color = "gray";

      console.log("[INFO]: Removed ", word);
    } else {
      const newSelectedRecoveryPhrase = [...selectedRecoveryPhrase];
      newSelectedRecoveryPhrase.push(word);
      setSelectedRecoveryPhrase(newSelectedRecoveryPhrase);
      e.target.style.backgroundColor = "#0894ec";
      e.target.style.color = "#FFFFFF";

      console.log("[INFO]: Pushed ", word);
    }

  }

  const isValid = (password) => {
    console.log("[INFO]: Check password validity.");
    return password.length >= MIN_PASSWORD_LENGTH;
  };

  // This function will be called after the user confirms their secret recovery phrase.
  const onClickCreateAccount = async () => {
    console.log("[INFO]: Creating account.");
    await sendPassword(password);
    setPassword('');
    setCurrentTab(FINAL_TAB);
  };

  // This function navigates back depending on the current page.
  const goBack = async () => {
    if (currentTab === PASSWORD_TAB) {
      console.log("[INFO]: Going back to Create or Recovery Page.")
      await restartServer();
    } else if (currentTab === SHOW_PHRASE_TAB) {
      console.log("[INFO]: Going back to Password Page.")
      setPassword('');
      setConfirmPassword('');
      setIsValidPassword(false);
      setRecoveryPhraseConfirmed(false);
      setCurrentTab(PASSWORD_TAB);
    } else if (currentTab === CONFIRM_PHRASE_TAB) {
      console.log("[INFO]: Going back to View Recovery Phrase Page.");
      setSelectedRecoveryPhrase([]);
      appWindow.setSize(DEFAULT_WINDOW_SIZE);
      setCurrentTab(SHOW_PHRASE_TAB);
    }
  }

  const goForward = async () => {

    if (currentTab === PASSWORD_TAB) {
      setCurrentTab(SHOW_PHRASE_TAB);
    } else if (currentTab === SHOW_PHRASE_TAB) {
      appWindow.setSize(CONFIRM_PHRASE_WINDOW_SIZE);
      setCurrentTab(CONFIRM_PHRASE_TAB);
    } else if (currentTab === CONFIRM_PHRASE_TAB) {
      // Recovery Phrase has already been confirmed here, the button will stop being disabled
      // Once the user has entered it in the correct order.
      onClickCreateAccount();
      appWindow.setSize(DEFAULT_WINDOW_SIZE);
      setCurrentTab(LOADING_TAB);
    }
  }

  // This function is called when the user clicks "Finish" at the final page
  // This function will redirect the user to log in with the password they just made.
  const onClickFinishSetup = async () => {
    console.log("[INFO]: Finishing Setup.")
    await restartServer(true);
  };

  // This function enables the Next button to continue in the account creation
  // process once the user has read the recovery phrase.
  const onClickConfirmRecoveryPhrase = () => {
    setRecoveryPhraseConfirmed(true);
  }

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

  }, [password, confirmPassword, isValidPassword, passwordsMatch]);

  useEffect(() => {

    // converting both arrays to strings in order to compare them
    let stringedActual = JSON.stringify(actualPhrase);
    let selectedRecoveryPhraseCopy = [...selectedRecoveryPhrase];

    // removing indexes from words to compare properly in case of duplicate words.
    for (let i = 0; i < selectedRecoveryPhraseCopy.length; i++) {
      selectedRecoveryPhraseCopy[i] = selectedRecoveryPhraseCopy[i].split("_")[0];
    }
    let stringedSelected = JSON.stringify(selectedRecoveryPhraseCopy);

    if (selectedRecoveryPhrase.length !== recoveryPhrase.split(" ").length) return;
    if ((!isValidSelectedPhrase) &&
      (stringedSelected === stringedActual)) {
      console.log("[INFO]: Valid phrase chosen.");
      setIsValidSelectedPhrase(true);
    } else if ((isValidSelectedPhrase) &&
      (stringedSelected !== stringedActual)) {
      console.log("[INFO]: Invalid phrase chosen.")
      setIsValidSelectedPhrase(false);
    }

  }, [selectedRecoveryPhrase, actualPhrase, isValidSelectedPhrase, recoveryPhrase]);


  useEffect(() => {
    // shuffles array so that user gets recovery phrase in a different order.
    let actualRecoveryPhrase = recoveryPhrase.split(" ");

    let shuffled = actualRecoveryPhrase
      .map(value => ({ value, sort: Math.random() }))
      .sort((a, b) => a.sort - b.sort)
      .map(({ value }) => value);

    for (let i = 0; i < shuffled.length; i++) {
      shuffled[i] = shuffled[i] + "_" + i;
    }

    // adding the item index so that we can distinguish between duplicate words.

    setShuffledRecoveryPhrase(shuffled);
    setActualPhrase(actualRecoveryPhrase);

  }, [recoveryPhrase]);

  return (
    <>
      {currentTab === PASSWORD_TAB && (
        <>
          <div className='tightHeaderContainer'>
            <h1 className='mainheadline'>Create your password</h1>
            <p className='subtext'>
              This is important. Your password will unlock the Manta Signer software in order
              to utilize your zkAddress and to sign transactions.
            </p>
          </div>
          <br />
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
          <Button disabled={!(isValid(password) && passwordsMatch)} className="button ui first"
            onClick={goForward}>
            Next
          </Button>
          <HyperLinkButton
            text={"Go Back"}
            onclick={goBack}
          />
          {!isValidPassword && password.length > 0 ? <><br /><Label basic color='red' pointing>Please enter a minimum of {MIN_PASSWORD_LENGTH} characters.</Label></> : (
            !passwordsMatch ? <><br /><Label basic color='red' pointing>Passwords do not match.</Label></> : <><br /><br /><br /></>
          )}

        </>
      )}
      {currentTab === SHOW_PHRASE_TAB && (
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
              return (
                <div key={index} className='recoveryPhraseWord'>
                  <h4>{item}</h4>
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
      )}
      {currentTab === CONFIRM_PHRASE_TAB && (<>
        <div className='tallHeaderContainer'>
          <h1 className='mainheadline'>Confirm Your Secret Recovery Phrase</h1>
          <p className='subtext'>
            Please select the appropriate phrase in the correct order.
          </p>
        </div>

        <div className='wordListContainer'>
          {selectedRecoveryPhrase.map(function (item, index) {
            let word = item.split("_")[0];
            return <div className='button ui buttonlist' key={index}>{word}</div>
          })}
        </div>

        <div className='buttonListContainer'>
          {shuffledRecoveryPhrase.map(function (item) {
            let word = item.split("_")[0];
            return (
              <Button
                onClick={(e) => onClickSelectWordButton(e, item)}
                className="button ui buttonlist"
                key={item}>
                {word}
              </Button>
            )
          })}
        </div>
        <Button disabled={!isValidSelectedPhrase} className="button ui first wide" onClick={goForward}>
          Confirm
        </Button>
        <HyperLinkButton
          text={"Go Back"}
          onclick={goBack}
        />

      </>)}
      {currentTab === FINAL_TAB && (<>
        <div className='headercontainerFat'>
          <h1 className='mainheadline'>You're all done!</h1>
          <h3 className='mediumSubText'>It's time to start using the Manta Signer.</h3>
        </div>
        <Button className="button ui first wide" onClick={onClickFinishSetup}>
          Finish
        </Button>
      </>
      )}
      {currentTab === LOADING_TAB && (
        <>
          <Loading />
        </>
      )}
    </>
  );
};

export default CreateAccount;
