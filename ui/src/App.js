import './App.css';
import Authorize from './pages/Authorize';
import CreateAccount from './pages/CreateAccount';
import Loading from './pages/Loading';
import SignIn from './pages/SignIn';
import Reset from "./pages/Reset";
import CreateOrRecover from './pages/CreateOrRecover';
import Recover from './pages/Recover';
import ViewSecretPhrase from './pages/ViewSecretPhrase';
import { Container } from 'semantic-ui-react';
import { appWindow } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';
import { useState, useEffect, useRef } from 'react';



const LOADING_PAGE = 0;
const CREATE_ACCOUNT_PAGE = 1;
const LOGIN_PAGE = 2;
const AUTHORIZE_PAGE = 3;
const RESET_PAGE = 4;
const CREATE_OR_RECOVER_PAGE = 5;
const RECOVER_PAGE = 6;
const EXPORT_RECOVERY_PHRASE_PAGE = 7;

function App() {
  const [currentPage, setCurrentPage] = useState(LOADING_PAGE);
  const currentPageRef = useRef(currentPage);
  const [isConnected, setIsConnected] = useState(false);
  const [payloadType, setPayloadType] = useState(null);
  const [recoveryPhrase, setRecoveryPhrase] = useState(null);
  const [authorizationSummary, setAuthorizationSummary] = useState(null);
  const [receivingKey, setReceivingKey] = useState(null);
  const [receivingKeyDisplay, setReceivingKeyDisplay] = useState(null);
  const [activeListeners, setActiveListeners] = useState({
    tx: false,
    connect: false,
    reset_tray: false
  });

  useEffect(() => {
    if (isConnected) return;
    if (activeListeners.connect) return;
    const beginInitialConnectionPhase = async () => {
      await listen('connect', (event) => {
        invoke('ui_connected');
        console.log("[INFO]: Connect Event: ", event);
        let payload = event.payload;

        // We don't want to switch the page on reset during recovery process.
        if (currentPageRef.current == RECOVER_PAGE) {
          setPayloadType('CreateAccount');
          setRecoveryPhrase(payload.content);
          return;
        }

        switch (payload.type) {
          case 'CreateAccount':
            setRecoveryPhrase(payload.content);
            setPayloadType('CreateAccount');
            setCurrentPage(CREATE_OR_RECOVER_PAGE);
            break;
          case 'Login':
            setPayloadType('Login');
            setCurrentPage(LOGIN_PAGE);
            break;
          default:
            break;
        }
      });
    };
    beginInitialConnectionPhase();
    setActiveListeners({ ...activeListeners, connect: true });
  }, [isConnected]);

  const hideWindow = () => {
    console.log("[INFO]: HIDE.");
    appWindow.hide();
    setCurrentPage(LOADING_PAGE);
  };

  const listenForTxAuthorizationRequests = () => {
    console.log("[INFO]: Setup listener.");
    listen('authorize', (event) => {
      console.log("[INFO]: Wake: ", event);

      // parsing authorization summary for easier legibility and rendering.
      let split_summary = event.payload.split(" ");

      const sendAmount = split_summary[1];
      const currency = split_summary[2];
      const toAddress = split_summary[4];

      const toAddress_short = toAddress.substr(0, 10)
        + "..." + toAddress.substr(toAddress.length - 10);

      // @TODO: add support for multiple networks here
      let parsed_authorization_summary = {
        sendAmount: sendAmount,
        currency: currency,
        toAddress: toAddress_short
      };

      setAuthorizationSummary(parsed_authorization_summary);
      setCurrentPage(AUTHORIZE_PAGE);
      appWindow.show();
    });
  };

  const listenForResetTrayRequests = () => {
    console.log("[INFO]: Setup tray reset listener.");
    listen('tray_reset_account', (event) => {
      console.log("[INFO]: Wake: ", event);
      setCurrentPage(RESET_PAGE);
    })
  }

  const sendCreateOrRecover = async (selection) => {
    console.log("[INFO]: Send selection to signer server.");
    return await invoke('create_or_recover', { selection: selection });
  }

  const sendPassword = async (password) => {
    console.log("[INFO]: Send password to signer server.");
    return await invoke('send_password', { password: password });
  };

  const sendMnemonic = async (mnemonic) => {
    console.log("[INFO]: Send mnemonic to signer server.");
    return await invoke('send_mnemonic', { mnemonic: mnemonic })
  }

  const stopPasswordPrompt = async () => {
    console.log("[INFO]: Stop password prompt.");
    await invoke('stop_password_prompt');
  };

  const resetAccount = async () => {
    console.log("[INFO]: Resetting Account.");

    await invoke('ui_disconnected');
    await invoke('reset_account', { delete: true });

  }

  const restartServer = async (loginPage = false) => {
    console.log("[INFO]: Restarting Server.");
    await invoke('ui_disconnected');
    await invoke('reset_account', { delete: false });

    if (loginPage) {
      setCurrentPage(LOGIN_PAGE);
    } else {
      setCurrentPage(CREATE_OR_RECOVER_PAGE);
    }
  }

  const endInitialConnectionPhase = async () => {
    console.log("[INFO]: End Initial Connection Phase");
    setIsConnected(true);
    hideWindow();
    if (activeListeners.tx || activeListeners.reset_tray) return;
    listenForTxAuthorizationRequests();
    listenForResetTrayRequests();
    setActiveListeners({
      ...activeListeners,
      tx: true,
      reset_tray: true
    })
  };

  const endConnection = async () => {
    console.log("[INFO]: Ending connection.");
    setIsConnected(false);
  }

  const startCreate = async () => {
    console.log("[INFO]: Start wallet creation process.")
    setCurrentPage(CREATE_ACCOUNT_PAGE);
  }

  const startRecover = async () => {
    console.log("[INFO]: Start recovery process.")
    setCurrentPage(RECOVER_PAGE);
  }

  const startShowRecoveryPhrase = async () => {
    console.log("[INFO]: Start recovery process.")
    setCurrentPage(EXPORT_RECOVERY_PHRASE_PAGE);
  }

  const cancelReset = async () => {
    console.log("[INFO]: Cancel reset process.")
    hideWindow();
  }

  const getReceivingKeys = async () => {
    const newReceivingKeys = await invoke('receiving_keys');
    const newReceivingKey = newReceivingKeys[0];
    setReceivingKey(newReceivingKey);
    const newReceivingKeyDisplay = newReceivingKey ?
      `${newReceivingKey.slice(0, 10)}...${newReceivingKey.slice(-10)}` :
      '';
    setReceivingKeyDisplay(newReceivingKeyDisplay);
  }

  // keeps the connect listener in sync with currentPage state
  useEffect(() => {
    currentPageRef.current = currentPage;
  }, [currentPage])

  return (
    <div className="App">
      <Container className="page">
        {currentPage === LOADING_PAGE && (
          <Loading />
        )}
        {currentPage === CREATE_OR_RECOVER_PAGE && (
          <CreateOrRecover
            sendCreateOrRecover={sendCreateOrRecover}
            startCreate={startCreate}
            startRecover={startRecover}
          />
        )}
        {currentPage === RESET_PAGE && (
          <Reset
            isConnected={isConnected}
            hideWindow={hideWindow}
            endConnection={endConnection}
            resetAccount={resetAccount}
            cancelReset={cancelReset}
          />
        )}
        {currentPage === RECOVER_PAGE && (
          <Recover
            payloadType={payloadType}
            sendCreateOrRecover={sendCreateOrRecover}
            endInitialConnectionPhase={endInitialConnectionPhase}
            restartServer={restartServer}
            resetAccount={resetAccount}
            hideWindow={hideWindow}
            sendPassword={sendPassword}
            sendMnemonic={sendMnemonic}
          />
        )}
        {currentPage === CREATE_ACCOUNT_PAGE && (
          <CreateAccount
            recoveryPhrase={recoveryPhrase}
            sendPassword={sendPassword}
            endInitialConnectionPhase={endInitialConnectionPhase}
            restartServer={restartServer}
          />
        )}
        {currentPage === LOGIN_PAGE && (
          <SignIn
            getReceivingKeys={getReceivingKeys}
            receivingKey={receivingKey}
            receivingKeyDisplay={receivingKeyDisplay}
            sendPassword={sendPassword}
            endInitialConnectionPhase={endInitialConnectionPhase}
            startRecover={startRecover}
          />
        )}
        {currentPage === AUTHORIZE_PAGE && (
          <Authorize
            summary={authorizationSummary}
            sendPassword={sendPassword}
            stopPasswordPrompt={stopPasswordPrompt}
            hideWindow={hideWindow}
          />
        )}
        {currentPage === EXPORT_RECOVERY_PHRASE_PAGE && (
          <ViewSecretPhrase
            hideWindow={hideWindow}
            recoveryPhrase={recoveryPhrase}
          />
        )}
      </Container>
    </div>
  );
}

export default App;
