import { useEffect, useState } from 'react';
import { Button, Input, Label, Header } from 'semantic-ui-react';
import "../App.css";

const MIN_PASSWORD_LENGTH = 8;
const PASSWORD_TAB = 0;
const SHOW_PHRASE_TAB = 1;
const CONFIRM_PHRASE_TAB = 2;
const FINAL_TAB = 3;


const CreateAccount = (props) => {
  const [password, setPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [passwordsMatch, setPasswordsMatch] = useState(false);
  const [isValidPassword, setIsValidPassword] = useState(false);
  const [currentTab, setCurrentTab] = useState(PASSWORD_TAB);
  const [recoveryPhraseConfirmed, setRecoveryPhraseConfirmed] = useState(false);
  const [isValidSelectedPhrase, setIsValidSelectedPhrase] = useState(false);
  const [shuffledRecoveryPhrase, setShuffledRecoveryPhrase] = useState(null);
  const [selectedRecoveryPhrase, setSelectedRecoveryPhrase] = useState([]);
  const [activeButtons,setActiveButtons] = useState([]);
  
  // This function gets called whenever a user clicks on a button
  const onClickButton = (e, word, index) => {

    if (!activeButtons.includes(index)) {

      setActiveButtons(prevState => [...prevState, index]);
      e.target.style.backgroundColor = "#0894ec";
      e.target.style.color = "#FFFFFF";

      setSelectedRecoveryPhrase(oldPhrase => [...oldPhrase, word]);


    } else {

      let buttons_copy = activeButtons;
      let button_index = buttons_copy.indexOf(index);
      buttons_copy.splice(button_index, 1);
      setActiveButtons(buttons_copy);

      e.target.style.backgroundColor = "#FFFFFF";
      e.target.style.color = "gray";

      let words_copy = selectedRecoveryPhrase;
      let word_index = words_copy.indexOf(word);
      words_copy.splice(word_index, 1);
      setSelectedRecoveryPhrase(words_copy);
    }

    console.log(selectedRecoveryPhrase);
    console.log(activeButtons)

  }
  


  const isValid = (password) => {
    console.log("[INFO]: Check password validity.");
    return password.length >= MIN_PASSWORD_LENGTH;
  };

  // This function will be called after the user confirms their secret recovery phrase.
  const onClickCreateAccount = async () => {
    console.log("[INFO]: Creating account.");
    if (isValid(password)) {
      await props.sendPassword(password);
      setPassword('');
    }
  };

  // This function navigates back depending on the current page.
  const goBack = async () => {
    if (currentTab == PASSWORD_TAB) {
      console.log("[INFO]: Going back to Create or Recovery Page.")
      await props.restartServer();
    } else if (currentTab == SHOW_PHRASE_TAB) {
      console.log("[INFO]: Going back to Password Page.")
      setPassword('');
      setConfirmPassword('');
      setIsValidPassword(false);
      setCurrentTab(PASSWORD_TAB);
    } else if (currentTab == CONFIRM_PHRASE_TAB) {
      console.log("[INFO]: Going back to View Recovery Phrase Page.");
      setActiveButtons([]);
      setSelectedRecoveryPhrase([]);
      setCurrentTab(SHOW_PHRASE_TAB);
    }
  }

  const goForward = async () => {

    if (currentTab == PASSWORD_TAB) {
      setCurrentTab(SHOW_PHRASE_TAB);
    } else if (currentTab == SHOW_PHRASE_TAB) {
      setCurrentTab(CONFIRM_PHRASE_TAB);
    } else if (currentTab == CONFIRM_PHRASE_TAB) {
      // Recovery Phrase has already been confirmed here, the button will stop being disabled
      // Once the user has entered it in the correct order.
      setCurrentTab(FINAL_TAB)
    }
  }

  // This function is called when the user clicks "Finish" at the final page
  // This function will close the signer tab.
  const onClickConfirmRecoveryPhrase = async () => {
    console.log("[INFO]: Confirming recovery phrase.")
    await props.endInitialConnectionPhase();
  };

  // This function enables the Next button to continue in the account creation
  // process once the user has read the recovery phrase.
  const confirmRecoveryPhrase = () => {
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

  }, [password, confirmPassword]);

  useEffect(() => {

    let actualPhrase = props.recoveryPhrase.split(" ");

    if ((!isValidSelectedPhrase) && selectedRecoveryPhrase == actualPhrase) {
      setIsValidSelectedPhrase(true);
    } else if ((isValidSelectedPhrase) && selectedRecoveryPhrase != actualPhrase) {
      setIsValidSelectedPhrase(false);
    }

  }, [selectedRecoveryPhrase]);


  useEffect(() => {
    // shuffles array so that user gets recovery phrase in a different order.
    let actualPhrase = props.recoveryPhrase.split(" ");

    let shuffled = actualPhrase
      .map(value => ({ value, sort: Math.random() }))
      .sort((a, b) => a.sort - b.sort)
      .map(({ value }) => value);

    setShuffledRecoveryPhrase(shuffled);

  }, []);

  return (
    <>
      {currentTab == PASSWORD_TAB && (
        <>
          <div className='headercontainer'>
            <h1 className='mainheadline'>Create a password</h1>
            <p className='subtext'>You will use it to unlock your wallet.</p>
          </div>
          <br />
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
          <div>
            <a onClick={goBack}>Go Back</a>
          </div>

          {!isValidPassword && password.length > 0 ? <><br /><Label basic color='red' pointing>Please enter a minimum of {MIN_PASSWORD_LENGTH} characters.</Label></> : (
            !passwordsMatch ? <><br /><Label basic color='red' pointing>Passwords do not match.</Label></> : <><br /><br /><br /></>
          )}

        </>
      )}
      {currentTab == SHOW_PHRASE_TAB && (
        <>
          <div className='headercontainer'>
            <h1 className='mainheadline'>Secret Recovery Phrase</h1>
          </div>
          <div className="recovery-phrase-info">
            <p>Please write down your secret recovery phrase and keep it in a safe place.</p>
            <p>This phrase is the only way to recover your wallet. Do not share it with anyone!</p>
          </div>

          <div className='recoveryPhraseContainer'>
            {props.recoveryPhrase.split(" ").map(function (item, index) {
              return (
                <div key={index} className='recoveryPhraseWord'>
                  <h4>{item}</h4>
                </div>
              )
            })}
          </div>

          <Button disabled={recoveryPhraseConfirmed} className="button ui first wide" onClick={goForward}>
            Next
          </Button>
          <div>
            <a onClick={goBack}>Go Back</a>
          </div>
        </>
      )}
      {currentTab == CONFIRM_PHRASE_TAB && (<>
        <div className='tightHeaderContainer'>
          <h1 className='mainheadline'>Confirm Your Secret Recovery Phrase</h1>
          <p className='subtext'>
            Please select each word in order to make sure it is correct.
          </p>
        </div>

        <div className='wordListContainer'>
          {selectedRecoveryPhrase.map(function (item, index) {
            return <div className="selectedWord" key={item}>{item}</div>
          })}
        </div>

        <div className='buttonListContainer'>
          {shuffledRecoveryPhrase.map(function (item,index) {
            return (
              <Button
                onClick={(e) => onClickButton(e, item)}
                className="button ui buttonlist"
                key={item}>
                {item}
              </Button>
            )
          })}
        </div>
        <Button disabled={!isValidSelectedPhrase} className="button ui first wide" onClick={goForward}>
          Confirm
        </Button>
        <div>
          <a onClick={goBack}>Go Back</a>
        </div>

      </>)}
      {currentTab == FINAL_TAB && (null)}
    </>
  );
};

export default CreateAccount;
