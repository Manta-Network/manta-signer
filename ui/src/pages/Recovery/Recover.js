import { useEffect, useState } from "react"
import { appWindow, LogicalSize } from '@tauri-apps/api/window';
import { Outlet } from "react-router-dom";
import { useNavigate, useLocation } from "react-router-dom";
const bip39 = require('bip39');

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
  12: new Array(12).fill(""),
  18: new Array(18).fill(""),
  24: new Array(24).fill("")
}

// By default this component will load using a 12 word phrase.
const Recover = ({
  payloadType,
  sendSelection,
  restartServer,
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
  const [showError, setShowError] = useState(false);

  const navigate = useNavigate();
  const location = useLocation();

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

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [password, confirmPassword]);

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
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [mnemonics]);

  const goBack = async () => {
    if (location.pathname === "/recover/seed-phrase") {
      appWindow.setSize(DEFAULT_WINDOW_SIZE);
      navigate(-1);
    } else if (location.pathname === "/recover/new-password") {
      // we need to throw away the mnemonics that were already stored
      setMnemonics(DEFAULT_PHRASES[12]);
      setMnemonicsValidity(false);
      navigate("/recover/seed-phrase");
    }
  }

  const goForward = async () => {
    if (location.pathname === "/recover/seed-phrase") {
      appWindow.setSize(DEFAULT_WINDOW_SIZE);
      navigate("/recover/new-password");
    } else if (location.pathname === "/recover/new-password") {

      navigate("/recover/loading");
      // If user came from the login page, it means we need to reset their 
      // old account first by wiping their storage, but not restarting the actual app.
      if (payloadType === "Login") {
        await restartServer(true,false);
      }

      await sendSelection("Recover");
      await sendMnemonic(validMnemonics);
      await sendPassword(password);
      navigate("/recover/finish");

    } else if (location.pathname === "/recover/finish") {
      // we restart the actual app here.
      // redirects to login page.
      await restartServer(false,true);
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

  const onChangePassword = (e) => {
    setShowError(false);
    setPassword(e.target.value);
  }

  const onChangeConfirmPassword = (e) => {
    setShowError(false);
    setConfirmPassword(e.target.value);
  }

  const checkPasswords = () => {

    if (!passwordsMatch || !isValidPassword) {
      setShowError(true);
    } else {
      goForward();
    }

  }

  return (<>
    <Outlet context={{
      checkPasswords,
      onChangeDropDown,
      onChangeWord,
      goBack,
      goForward,
      onChangePassword,
      onChangeConfirmPassword,
      DROPDOWN_OPTIONS,
      MIN_PASSWORD_LENGTH,
      isValidPassword,
      passwordsMatch,
      password,
      mnemonicsValidity,
      mnemonics,
      showError
    }} />
  </>);
}

export default Recover;