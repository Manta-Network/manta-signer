import { useEffect, useState } from "react"
import { Button, Input, Label, Form, Dropdown } from 'semantic-ui-react';
import "../App.css";
import { appWindow, LogicalSize } from '@tauri-apps/api/window';
import Loading from './Loading';
import HyperLinkButton from "../components/HyperLinkButton";

const bip39 = require('bip39');

const SEED_PHRASE_PAGE = 0;
const NEW_PASSWORD_PAGE = 1;
const FINISH_PAGE = 2;
const LOADING_TAB = 3;

const MIN_PASSWORD_LENGTH = 8;

const DEFAULT_WINDOW_SIZE = new LogicalSize(460, 500);
const CONFIRM_PHRASE_WINDOW_SIZE = new LogicalSize(460, 750);

const DROPDOWN_OPTIONS = [
  {
    text: "I have a 12 word phrase",
    value: 12
  },
  {
    text: "I have a 18 word phrase",
    value: 18
  },
  {
    text: "I have a 24 word phrase",
    value: 24
  },
];

const DEFAULT_PHRASES = {
  12: ["", "", "", "", "", "", "", "", "", "", "", ""],
  18: ["", "", "", "", "", "", "", "", "", "", "", "",
    "", "", "", "", "", ""],
  24: ["", "", "", "", "", "", "", "", "", "", "", "",
    "", "", "", "", "", "", "", "", "", "", "", ""]
}

// By default this component will load using a 12 word phrase.
const Recover = ({
  payloadType,
  sendCreateOrRecover,
  restartServer,
  resetAccount,
  sendPassword,
  sendMnemonic
}) => {

  // recovery phrase
  const [mnemonics, setMnemonics] = useState(DEFAULT_PHRASES[12]);
  const [mnemonicsValidity, setMnemonicsValidity] = useState(false);
  const [validMnemonics, setValidMnemonics] = useState(null);

  // new passwords
  const [password, setPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [passwordsMatch, setPasswordsMatch] = useState('');
  const [isValidPassword, setIsValidPassword] = useState(false);

  // currently active page
  const [currentPage, setCurrentPage] = useState(SEED_PHRASE_PAGE);

  useEffect(() => {

    const checkPasswordMatch = () => {
      if (password === confirmPassword) {
        setPasswordsMatch(true);
      } else {
        setPasswordsMatch(false);
      }
    }

    const checkPasswordValidity = () => {
      if (isValid(password)) {
        setIsValidPassword(true);
      } else {
        setIsValidPassword(false);
      }
    }

    checkPasswordMatch();
    checkPasswordValidity();

  }, [password, confirmPassword, isValidPassword, passwordsMatch]);

  useEffect(() => {

    const validateSelectedMnemonic = () => {
      // we need to preprocess the mnemonics first for the bip39 library, 
      // by removing any whitespace and setting all words to lowercase.
      const trimmedMnemonics = mnemonics.map(x => x.trim());
      const lowerCaseMnemonics = trimmedMnemonics.map(x => x.toLowerCase());
      const mnemonicsString = lowerCaseMnemonics.join(" ");
      const emptyStrings = mnemonics.filter(x => x.length === 0).length;

      // we only verify mnemonics validity if all strings have been filled.
      if (emptyStrings === 0) {

        const isValid = bip39.validateMnemonic(mnemonicsString);

        if (isValid && (!mnemonicsValidity)) {
          console.log("[INFO]: Selected mnemonics are valid.")
          setMnemonicsValidity(true);
          setValidMnemonics(mnemonicsString);
        } else if (!isValid && (mnemonicsValidity)) {
          console.log("[INFO]: Selected mnemonics are invalid.")
          setMnemonicsValidity(false);
          setValidMnemonics(null);
        }

      } else if (mnemonicsValidity) {
        console.log("[INFO]: Selected mnemonics are invalid.")
        setMnemonicsValidity(false);
        setValidMnemonics(null);
      }
    }

    validateSelectedMnemonic();

  }, [mnemonics, mnemonicsValidity]);

  const goBack = async () => {
    if (currentPage === SEED_PHRASE_PAGE) {
      appWindow.setSize(DEFAULT_WINDOW_SIZE);
      await restartServer(payloadType === "Login");
    } else if (currentPage === NEW_PASSWORD_PAGE) {
      // we need to throw away the mnemonics that were already stored
      setMnemonics(DEFAULT_PHRASES[12]);
      setMnemonicsValidity(false);
      setCurrentPage(SEED_PHRASE_PAGE);
    }
  }

  const goForward = async () => {
    if (currentPage === SEED_PHRASE_PAGE) {
      appWindow.setSize(DEFAULT_WINDOW_SIZE);
      setCurrentPage(NEW_PASSWORD_PAGE);
    } else if (currentPage === NEW_PASSWORD_PAGE) {

      setCurrentPage(LOADING_TAB);
      // If user came from the login page, it means we need to reset their 
      // old account first by wiping their storage.
      if (payloadType === "Login") {
        await resetAccount(true);
      }

      await sendCreateOrRecover("Recover");
      await sendMnemonic(validMnemonics);
      await sendPassword(password);
      setCurrentPage(FINISH_PAGE);

    } else if (currentPage === FINISH_PAGE) {
      await restartServer(true); // redirect to login page
    }
  }

  const isValid = (password) => {
    console.log("[INFO]: Check password validity.");
    return password.length >= MIN_PASSWORD_LENGTH;
  };

  const onChangeDropDown = async (_e, data) => {

    const amountOfWords = data.value;
    const newWords = [...DEFAULT_PHRASES[amountOfWords]];

    console.log("[INFO]: Selected " + amountOfWords + " word recovery phrase.");

    if (amountOfWords === 12) {
      appWindow.setSize(DEFAULT_WINDOW_SIZE);
    } else if ((amountOfWords === 24) || (amountOfWords === 18)) {
      appWindow.setSize(CONFIRM_PHRASE_WINDOW_SIZE);
    }

    setMnemonics(newWords)
  }

  // update mnemonics state when text box has word changed
  const onChangeWord = (_e, textObj, index) => {

    const word = textObj.value;
    const newWords = [...mnemonics];
    newWords[index] = word;
    setMnemonics(newWords);

  }

  return (<>

    {currentPage === SEED_PHRASE_PAGE && <>

      <div className='recoverHeaderContainer'>
        <h1 className='mainheadline'>Reset Wallet</h1>
        <p className='subtext'>
          You can reset your password by entering your secret recovery phrase.
        </p>
      </div>

      <div>
        <Dropdown
          className="ui fluid dropdown compressed"
          fluid
          selection
          options={DROPDOWN_OPTIONS}
          onChange={onChangeDropDown}
          defaultValue={DROPDOWN_OPTIONS[0].value}
        />
      </div>

      <Form className="ui form adjusted">
        {mnemonics.map(function (_item, index) {
          return (
            <Form.Field
              className="ui form field thin"
              placeholder={(index + 1).toString() + "."}
              control={Input}
              key={index}
              onChange={(e, textObj) => onChangeWord(e, textObj, index)}
            />
          )
        })}
      </Form>

      <div>
        {mnemonicsValidity ?
          <Button primary className="button ui first" onClick={goForward}>Next</Button> :
          <Button disabled primary className="button ui first">Next</Button>}
      </div>
      <HyperLinkButton
        text={"Go Back"}
        onclick={goBack}
      />
    </>
    }


    {currentPage === NEW_PASSWORD_PAGE && (
      <>
        <div className='headercontainer'>
          <h1 className='mainheadline'>Create a password</h1>
          <p className='subtext'>Your password will unlock the Manta Signer.</p>
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
        <HyperLinkButton
          text={"Go Back"}
          onclick={goBack}
        />

        {!isValidPassword && password.length > 0 ? <><br /><Label basic color='red' pointing>Please enter a minimum of {MIN_PASSWORD_LENGTH} characters.</Label></> : (
          !passwordsMatch ? <><br /><Label basic color='red' pointing>Passwords do not match.</Label></> : <><br /><br /><br /></>
        )}

      </>
    )}

    {currentPage === FINISH_PAGE && (<>
      <div className='headercontainerFat'>
        <h1 className='mainheadline'>You're all done!</h1>
        <h3 className='mediumSubText'>It's time to start using the Manta Signer.</h3>
      </div>
      <Button className="button ui first wide" onClick={goForward}>
        Finish
      </Button>
    </>
    )}
    {currentPage === LOADING_TAB && (
      <>
        <Loading />
      </>
    )}
  </>);
}

export default Recover;