import './App.css';
import Authorize from './pages/Authorize';
import CreateAccount from './pages/AccountCreation/CreateAccount';
import Loading from './pages/Loading';
import SignIn from './pages/SignIn';
import Reset from "./pages/Reset";
import CreateOrRecover from './pages/CreateOrRecover';
import Recover from './pages/Recovery/Recover';
import ViewSecretPhrase from './pages/ViewSecretPhrase';
import { Container } from 'semantic-ui-react';
import { appWindow } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';
import { useState, useEffect, useRef } from 'react';
import { Navigate, useNavigate, useLocation, Route, Routes } from 'react-router-dom';
import SeedPhrase from './pages/Recovery/SeedPhrase';
import NewPassword from './pages/Recovery/NewPassword';
import Finish from './pages/Recovery/Finish';
import ShowPhrase from './pages/AccountCreation/ShowPhrase';
import ConfirmPhrase from './pages/AccountCreation/ConfirmPhrase';

const SEND = "Send";
const WITHDRAW = "Withdraw";
const GET_RECOVERY_PHRASE = "GetRecoveryPhrase";

function App() {
  const navigate = useNavigate();
  const location = useLocation();

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
  const pathnameRef = useRef(location.pathname);

  useEffect(() => {
    pathnameRef.current = location.pathname;
  }, [location.pathname]);

  useEffect(() => {
    if (isConnected) return;
    if (activeListeners.connect) return;
    const beginInitialConnectionPhase = async () => {
      await listen('connect', (event) => {
        invoke('connect_ui');
        console.log("[INFO]: Connect Event: ", event);
        let payload = event.payload;

        // We don't want to switch the page on reset during recovery process.
        if (pathnameRef.current === ("/recover/loading")) {
          setPayloadType('CreateAccount');
          setRecoveryPhrase(payload.content);
          return;
        }

        switch (payload.type) {
          case 'CreateAccount':
            setRecoveryPhrase(payload.content);
            setPayloadType('CreateAccount');
            navigate("/create-or-recover")
            break;
          case 'Login':
            setPayloadType('Login');
            invoke('enable_reset_menu_item');
            navigate("/sign-in");
            break;
          default:
            break;
        }
      });
    };

    const listenForResetTrayRequests = async () => {
      console.log("[INFO]: Setup tray reset listener.");
      listen('tray_reset_account', async (event) => {
        console.log("[INFO]: Wake: ", event);

        // checking to make sure to end password stalling listener on backend if we switch away
        // from the export secret phrase.
        if (exportingPhraseRef.current) {
          console.log("[INFO]: Ending export phrase process.");
          await stopPasswordPrompt();
          endExportPhrase(false);
        }
        navigate("/reset");
      })
    }

    beginInitialConnectionPhase();
    if (!activeListeners.tray_reset_account) {
      listenForResetTrayRequests();
      setActiveListeners({
        ...activeListeners,
        tray_reset_account: true,
      })
    }
    setActiveListeners({ ...activeListeners, connect: true, tray_reset_account: true });
  }, [isConnected, activeListeners, pathnameRef, navigate]);

  // keeps show secret phrase listener in sync with exportingPhrase state
  // whether or not we are currently exporting the phrase.
  useEffect(() => {
    exportingPhraseRef.current = exportingPhrase;
  }, [exportingPhrase])

  const hideWindow = () => {
    console.log("[INFO]: HIDE.");
    appWindow.hide();
    navigate("/loading");
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
        navigate("/view-secret-phrase");
        appWindow.show();
        return;
      }

      // Case 2: we need authorization for signing a transaction.
      let parsedAuthorizationSummary = parseTransactionSummary(event.payload.split(" "));

      setAuthorizationSummary(parsedAuthorizationSummary);
      navigate("/authorize");
      appWindow.show();
    });
  };

  const listenForShowSecretPhraseRequests = async () => {
    console.log("[INFO]: Setup tray show secret phrase listener.");
    listen('show_secret_phrase', (_event) => {
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

  const sendSelection = async (selection) => {
    console.log("[INFO]: Send selection to signer server.");
    return await invoke('user_selection', { selection: selection });
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

    // for faster redirection
    if (loginPage) {
      navigate("/sign-in");
    } else {
      navigate("/create-or-recover");
    }

    await invoke('disconnect_ui');
    await invoke('reset_account', { delete: false });
  }

  const endInitialConnectionPhase = async () => {
    console.log("[INFO]: End Initial Connection Phase");
    setIsConnected(true);

    if (!activeListeners.authorize) {
      listenForAuthorizationRequests();
      setActiveListeners({
        ...activeListeners,
        authorize: true,
      })
    }

    if (!activeListeners.show_secret_phrase) {
      listenForShowSecretPhraseRequests();
      setActiveListeners({
        ...activeListeners,
        show_secret_phrase: true
      })
    }

  };

  const endConnection = async () => {
    console.log("[INFO]: Ending connection.");
    setIsConnected(false);
  }

  const startCreate = async () => {
    console.log("[INFO]: Start wallet creation process.")
    navigate("/create-account/new-password");
  }

  const startRecover = async () => {
    console.log("[INFO]: Start recovery process.")
    navigate("/recover/seed-phrase");
  }

  const cancelSign = async () => {
    console.log("[INFO]: Cancelling signing current transaction.");
    await invoke('cancel_sign');
  }

  const endExportPhrase = async (hide = true) => {
    console.log("[INFO]: Ending export recovery phrase process.")
    setExportingPhrase(false);
    setExportedSecretPhrase(null);
    if (hide) {
      hideWindow();
    }
  }

  const getReceivingKeys = async () => {
    console.log("[INFO]: Getting receiving keys.")
    const newReceivingKeys = await invoke('receiving_keys');
    const newReceivingKey = newReceivingKeys[0];
    setReceivingKey(newReceivingKey);
    const newReceivingKeyDisplay = newReceivingKey ?
      `${newReceivingKey.slice(0, 10)}...${newReceivingKey.slice(-10)}` :
      '';
    setReceivingKeyDisplay(newReceivingKeyDisplay);
  }

  const cancelReset = async () => {
    navigate(-1);
  }

  return (
    <div className="App">
      <Container className="page">
        <Routes>
          <Route exact path='/' element={<Navigate to={"/loading"} />} />
          <Route path='/loading' element={<Loading />} />
          <Route path='/create-or-recover' element={
            <CreateOrRecover
              startCreate={startCreate}
              startRecover={startRecover}
            />
          } />
          <Route path='/reset' element={
            <Reset
              isConnected={isConnected}
              hideWindow={hideWindow}
              endConnection={endConnection}
              resetAccount={resetAccount}
              cancelReset={cancelReset}
            />
          } />
          <Route path='/recover' element={
            <Recover
              payloadType={payloadType}
              sendSelection={sendSelection}
              restartServer={restartServer}
              resetAccount={resetAccount}
              sendPassword={sendPassword}
              sendMnemonic={sendMnemonic}
            />
          }>
            <Route path='seed-phrase' element={<SeedPhrase />}></Route>
            <Route path='new-password' element={<NewPassword />}></Route>
            <Route path='finish' element={<Finish />}></Route>
            <Route path='loading' element={<Loading />}></Route>
          </Route>
          <Route path='/create-account' element={
            <CreateAccount
              sendSelection={sendSelection}
              recoveryPhrase={recoveryPhrase}
              sendPassword={sendPassword}
              restartServer={restartServer}
            />
          }>
            <Route path='new-password' element={<NewPassword />}></Route>
            <Route path='show-phrase' element={<ShowPhrase />}></Route>
            <Route path='confirm-phrase' element={<ConfirmPhrase />}></Route>
            <Route path='finish' element={<Finish />}></Route>
            <Route path='loading' element={<Loading />}></Route>
          </Route>
          <Route path='/sign-in' element={
            <SignIn
              sendSelection={sendSelection}
              getReceivingKeys={getReceivingKeys}
              receivingKey={receivingKey}
              receivingKeyDisplay={receivingKeyDisplay}
              sendPassword={sendPassword}
              endInitialConnectionPhase={endInitialConnectionPhase}
              startRecover={startRecover}
              hideWindow={hideWindow}
            />
          } />
          <Route path='/authorize' element={
            <Authorize
              cancelSign={cancelSign}
              summary={authorizationSummary}
              sendPassword={sendPassword}
              stopPasswordPrompt={stopPasswordPrompt}
              hideWindow={hideWindow}
            />} />
          <Route path='/view-secret-phrase' element={
            <ViewSecretPhrase
              endExportPhrase={endExportPhrase}
              exportedSecretPhrase={exportedSecretPhrase}
              sendPassword={sendPassword}
              stopPasswordPrompt={stopPasswordPrompt}
            />
          } />
        </Routes>
      </Container>
    </div>
  );
}

export default App;
