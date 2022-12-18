import './App.css';
import Authorize from './pages/Authorize';
import CreateAccount from './pages/AccountCreation/CreateAccount';
import Loading from './pages/Loading';
import SignIn from './pages/SignIn/SignIn';
import Reset from "./pages/Reset";
import CreateOrRecover from './pages/CreateOrRecover';
import Recover from './pages/Recovery/Recover';
import ViewSecretPhrase from './pages/ViewPhrase/ViewSecretPhrase';
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
const PUBLIC = "Public";
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
  const [loginFailedOccured, setLoginFailedOccured] = useState(false);

  // keeps show secret phrase listener in sync with exportingPhrase state
  // whether or not we are currently exporting the phrase.
  const exportingPhraseRef = useRef(false);

  const pathnameRef = useRef(location.pathname);

  useEffect(() => {
    pathnameRef.current = location.pathname;
  }, [location.pathname]);

  useEffect(() => {
    if (isConnected) return;
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
            navigate("/create-or-recover");
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

    if (!activeListeners.connect) {
      beginInitialConnectionPhase();
      setActiveListeners({
        ...activeListeners,
        connect: true,
      });
    }
    if (!activeListeners.tray_reset_account) {
      listenForResetTrayRequests();
      setActiveListeners({
        ...activeListeners,
        tray_reset_account: true,
      });
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isConnected, activeListeners, pathnameRef, navigate]);

  const hideWindow = () => {
    console.log("[INFO]: HIDE.");
    appWindow.hide();
    navigate("/loading");
  };

  // This function parses the transaction summary message depending
  // on the type of transaction: TransferShape::PrivateTransfer, TransferShape::Reclaim
  const parseTransactionSummary = (summary) => {

    let parsedAuthorizationSummary = {
      sendAmount: parseFloat(summary[1]),
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
    } else if (summary[0] === PUBLIC) {

      // Public {} on {} network
      parsedAuthorizationSummary.toAddress = "Your Public Address";
      parsedAuthorizationSummary.network = summary[4];
    } else {
      // Privatize {} on {} network
      parsedAuthorizationSummary.toAddress = "Your Private zkAddress";
      parsedAuthorizationSummary.network = summary[4];
    }

    return parsedAuthorizationSummary;
  }

  const listenForAuthorizationRequests = () => {
    console.log("[INFO]: Setup listener.");
    listen('authorize', async (event) => {
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
      exportingPhraseRef.current = true;
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

  const restartServer = async (deleteAccount, restartApp) => {
    console.log("[INFO]: Restarting Server with current parameters: "
      + "Delete Account: " + deleteAccount + " Restart App: " + restartApp
    );
    await endConnection();
    await invoke('disconnect_ui');

    const payload = {
      delete: deleteAccount,
      restart: restartApp
    };

    await invoke('reset_account', payload);
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
    exportingPhraseRef.current = false;
    setExportedSecretPhrase(null);
    if (hide) {
      hideWindow();
    }
  }

  const getReceivingKeys = async () => {
    console.log("[INFO]: Getting receiving keys.")
    const newReceivingKey = await invoke('address');
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
          <Route path='/loading' element={
            <Loading
              isConnected={isConnected}
            />
          } />
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
              restartServer={restartServer}
              cancelReset={cancelReset}
            />
          } />
          <Route path='/recover' element={
            <Recover
              payloadType={payloadType}
              sendSelection={sendSelection}
              restartServer={restartServer}
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
              loginFailedOccured={loginFailedOccured}
              setLoginFailedOccured={setLoginFailedOccured}
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
