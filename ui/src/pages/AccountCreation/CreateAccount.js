import { useEffect, useState } from 'react';
import "../../App.css";
import { appWindow, LogicalSize } from '@tauri-apps/api/window';
import { Outlet } from 'react-router-dom';
import { useNavigate, useLocation } from "react-router-dom";

const MIN_PASSWORD_LENGTH = 8;

const DEFAULT_WINDOW_SIZE = new LogicalSize(460, 500);
const CONFIRM_PHRASE_WINDOW_SIZE = new LogicalSize(460, 600);

const CreateAccount = ({
  sendSelection,
  recoveryPhrase,
  sendPassword,
  restartServer
}) => {
  const [password, setPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [passwordsMatch, setPasswordsMatch] = useState(false);
  const [isValidPassword, setIsValidPassword] = useState(false);
  const [recoveryPhraseConfirmed, setRecoveryPhraseConfirmed] = useState(false);
  const [isValidSelectedPhrase, setIsValidSelectedPhrase] = useState(false);
  const [shuffledRecoveryPhrase, setShuffledRecoveryPhrase] = useState(null);
  const [selectedRecoveryPhrase, setSelectedRecoveryPhrase] = useState([]);
  const [actualPhrase, setActualPhrase] = useState(null);
  const [showError, setShowError] = useState(false);

  const navigate = useNavigate();
  const location = useLocation();

  const checkPasswords = () => {

    if (!passwordsMatch || !isValidPassword) {
      setShowError(true);
    } else {
      goForward();
    }

  }

  useEffect(() => {

    const checkPasswordValidity = () => {
      if (isValid(password)) {
        setIsValidPassword(true);
      } else {
        setIsValidPassword(false);
      }
    }
  
    const checkPasswordMatch = () => {
      if (password === confirmPassword) {
        setPasswordsMatch(true);
      } else {
        setPasswordsMatch(false);
      }
    }

    checkPasswordMatch();
    checkPasswordValidity();

  }, [password, confirmPassword, isValidPassword, passwordsMatch]);

  // determines whether or not the inputted recovery phrase matches the one
  // originally provided.
  useEffect(() => {

    const isValidPhraseSelection = () => {
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
    }

    isValidPhraseSelection();

  }, [selectedRecoveryPhrase, actualPhrase, isValidSelectedPhrase, recoveryPhrase]);

  useEffect(() => {

    const shuffleRecoveryPhrase = () => {
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
    }

    shuffleRecoveryPhrase();

  }, [recoveryPhrase]);

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
    await sendSelection("Create");
    await sendPassword(password);
    setPassword('');
    navigate("/create-account/finish");
  };

  // This function navigates back depending on the current page.
  const goBack = async () => {
    if (location.pathname === "/create-account/new-password") {
      console.log("[INFO]: Going back to Create or Recovery Page.")
      navigate("/create-or-recover");
    } else if (location.pathname === "/create-account/show-phrase") {
      console.log("[INFO]: Going back to Password Page.")
      setPassword('');
      setConfirmPassword('');
      setIsValidPassword(false);
      setRecoveryPhraseConfirmed(false);
      navigate("/create-account/new-password");
    } else if (location.pathname === "/create-account/confirm-phrase") {
      console.log("[INFO]: Going back to View Recovery Phrase Page.");
      setSelectedRecoveryPhrase([]);
      appWindow.setSize(DEFAULT_WINDOW_SIZE);
      navigate("/create-account/show-phrase");
    }
  }

  const goForward = async () => {

    if (location.pathname === "/create-account/new-password") {
      navigate("/create-account/show-phrase");
    } else if (location.pathname === "/create-account/show-phrase") {
      appWindow.setSize(CONFIRM_PHRASE_WINDOW_SIZE);
      navigate("/create-account/confirm-phrase");
    } else if (location.pathname === "/create-account/confirm-phrase") {
      // Recovery Phrase has already been confirmed here, the button will stop being disabled
      // Once the user has entered it in the correct order.
      onClickCreateAccount();
      appWindow.setSize(DEFAULT_WINDOW_SIZE);
      navigate("/create-account/loading");
    } else if (location.pathname === "/create-account/finish") {
      onClickFinishSetup();
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

  const onChangePassword = (e) => {
    setShowError(false);
    setPassword(e.target.value);
  }

  const onChangeConfirmPassword = (e) => {
    setShowError(false);
    setConfirmPassword(e.target.value);
  }

  return (
    <>
      <Outlet context={{
        goBack,
        goForward,
        checkPasswords,
        onChangePassword,
        onChangeConfirmPassword,
        onClickConfirmRecoveryPhrase,
        onClickSelectWordButton,
        isValidSelectedPhrase,
        shuffledRecoveryPhrase,
        selectedRecoveryPhrase,
        recoveryPhraseConfirmed,
        recoveryPhrase,
        MIN_PASSWORD_LENGTH,
        isValidPassword,
        passwordsMatch,
        password,
        showError
      }} />
    </>
  );
};

export default CreateAccount;
