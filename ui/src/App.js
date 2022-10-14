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

const SEND = "Send";
const WITHDRAW = "Withdraw";
const GET_RECOVERY_PHRASE = "GetRecoveryPhrase";

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
    authorize: false,
    connect: false,
    tray_reset_account: false,
    show_secret_phrase: false
  });
  const [exportedSecretPhrase, setExportedSecretPhrase] = useState(null);
  const [exportingPhrase, setExportingPhrase] = useState(false);
  const exportingPhraseRef = useRef(exportingPhrase);

  useEffect(() => {
    if (isConnected) return;
    if (activeListeners.connect) return;
    const beginInitialConnectionPhase = async () => {
      await listen('connect', (event) => {
        invoke('connect_ui');
        console.log("[INFO]: Connect Event: ", event);
        let payload = event.payload;

        // We don't want to switch the page on reset during recovery process.
        if (currentPageRef.current === RECOVER_PAGE) {
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
    setActiveListeners({ ...activeListeners, connect: true});
  }, [isConnected, activeListeners]);

  // keeps the connect listener in sync with currentPage state
  useEffect(() => {
    currentPageRef.current = currentPage;
  }, [currentPage])

  // keeps show secret phrase listener in sync with exportingPhrase state
  // whether or not we are currently exporting the phrase.
  useEffect(() => {
    exportingPhraseRef.current = exportingPhrase;
  },[exportingPhrase])

  const hideWindow = () => {
    console.log("[INFO]: HIDE.");
    appWindow.hide();
    setCurrentPage(LOADING_PAGE);
  };

  // This function parses the transaction summary message depending
  // on the type of transaction: TransferShape::PrivateTransfer, TransferShape::Reclaim
  const parseTransactionSummary = (summary) => {

    let parsedAuthorizationSummary = {
      sendAmount: summary[1],
      currency: summary[2],
      toAddress: null,
      network: null
    };

    if (summary[0] === SEND) {

      // Send {} to {} on {} network
      let toAddress = summary[4];
      toAddress = toAddress.substr(0, 10)
        + "..." + toAddress.substr(toAddress.length - 10);
        parsedAuthorizationSummary.toAddress = toAddress;
      parsedAuthorizationSummary.network = summary[6];
    } else if (summary[0] === WITHDRAW) {

      // Withdraw {} on {} network
      parsedAuthorizationSummary.toAddress = "Your Public Address";
      parsedAuthorizationSummary.network = summary[4];
    }

    return parsedAuthorizationSummary;
  }

  const listenForAuthorizationRequests = () => {
    console.log("[INFO]: Setup listener.");
    listen('authorize', (event) => {
      console.log("[INFO]: Wake: ", event);

      // Case 1: we need authorization for exporting the recovery phrase.
      if (event.payload === GET_RECOVERY_PHRASE) {
        setCurrentPage(EXPORT_RECOVERY_PHRASE_PAGE);
        appWindow.show();
        return;
      }

      // Case 2: we need authorization for signing a transaction.
      let parsedAuthorizationSummary = parseTransactionSummary(event.payload.split(" "));

      setAuthorizationSummary(parsedAuthorizationSummary);
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

  const listenForShowSecretPhraseRequests = async () => {
    console.log("[INFO]: Setup tray show secret phrase listener.");
    listen('show_secret_phrase', (event) => {
      getSecretRecoveryPhrase();
    })
  }

  const getSecretRecoveryPhrase = async () => {

    if (exportingPhraseRef.current) {
      return;
    } else {
      setExportingPhrase(true);
    }
    
    console.log("[INFO]: Send request to export recovery phrase.");
    let phrase = await invoke('get_recovery_phrase', { prompt: GET_RECOVERY_PHRASE });

    if (phrase) {
      setExportedSecretPhrase(phrase);
    }
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

    await invoke('disconnect_ui');
    await invoke('reset_account', { delete: true });

  }

  const restartServer = async (loginPage = false) => {
    console.log("[INFO]: Restarting Server.");
    await invoke('disconnect_ui');
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
    listenForAuthorizationRequests();
    listenForResetTrayRequests();
    listenForShowSecretPhraseRequests();
    setActiveListeners({
      ...activeListeners,
      authorize: true,
      tray_reset_account: true,
      show_secret_phrase: true
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

  const cancelReset = async () => {
    console.log("[INFO]: Cancel reset process.")
    hideWindow();
  }

  const cancelSign = async () => {
    console.log("[INFO]: Cancelling signing current transaction.");
    await invoke('cancel_sign');
  }

  const endExportPhrase = async () => {
    console.log("[INFO]: Ending export recovery phrase process.")
    setExportingPhrase(false);
    setExportedSecretPhrase(null);
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
            restartServer={restartServer}
            resetAccount={resetAccount}
            sendPassword={sendPassword}
            sendMnemonic={sendMnemonic}
          />
        )}
        {currentPage === CREATE_ACCOUNT_PAGE && (
          <CreateAccount
            recoveryPhrase={recoveryPhrase}
            sendPassword={sendPassword}
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
            cancelSign={cancelSign}
            summary={authorizationSummary}
            sendPassword={sendPassword}
            stopPasswordPrompt={stopPasswordPrompt}
            hideWindow={hideWindow}
          />
        )}
        {currentPage === EXPORT_RECOVERY_PHRASE_PAGE && (
          <ViewSecretPhrase
            endExportPhrase={endExportPhrase}
            exportedSecretPhrase={exportedSecretPhrase}
            sendPassword={sendPassword}
            stopPasswordPrompt={stopPasswordPrompt}
          />
        )}
      </Container>
    </div>
  );
}

export default App;
