import './App.css';
import Authorize from './pages/Authorize';
import CreateAccount from './pages/CreateAccount';
import Loading from './pages/Loading';
import SignIn from './pages/SignIn';
import SignInOrReset from './pages/SignInOrReset';
import Reset from "./pages/Reset";
import CreateOrRecover from './pages/CreateOrRecover';
import Recover from './pages/Recover';
import { Container } from 'semantic-ui-react';
import { appWindow } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';
import { useState, useEffect } from 'react';


const LOADING_PAGE = 0;
const CREATE_ACCOUNT_PAGE = 1;
const LOGIN_PAGE = 2;
const AUTHORIZE_PAGE = 3;
const SIGN_IN_OR_RESET_PAGE = 4;
const RESET_PAGE = 5;
const CREATE_OR_RECOVER_PAGE = 6;
const RECOVER_PAGE = 7;

function App() {
  const [currentPage, setCurrentPage] = useState(LOADING_PAGE);
  const [isConnected, setIsConnected] = useState(false);
  const [recoveryPhrase, setRecoveryPhrase] = useState(null);
  const [authorizationSummary, setAuthorizationSummary] = useState(null);

  useEffect(() => {
    if (isConnected) return;
    const beginInitialConnectionPhase = async () => {
      await listen('connect', (event) => {
        console.log("[INFO]: Connect Event: ", event);
        let payload = event.payload;
        switch (payload.type) {
          case 'CreateAccount':
            setRecoveryPhrase(payload.content);
            setCurrentPage(CREATE_OR_RECOVER_PAGE);
            break;
          case 'Login':
            setCurrentPage(SIGN_IN_OR_RESET_PAGE);
            break;
          default:
            break;
        }
      });
    };
    beginInitialConnectionPhase();
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
      setAuthorizationSummary(event.payload);
      setCurrentPage(AUTHORIZE_PAGE);
      appWindow.show();
    });
  };

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
    return await invoke('send_mnemonic', { mnemonic : mnemonic })
  }

  const stopPasswordPrompt = async () => {
    console.log("[INFO]: Stop password prompt.");
    await invoke('stop_password_prompt');
  };

  const resetAccount = async () => {
    console.log("[INFO]: Resetting Account.");
    await invoke('reset_account');
    setCurrentPage(CREATE_OR_RECOVER_PAGE);
  }

  const endInitialConnectionPhase = async () => {
    console.log("[INFO]: End Initial Connection Phase");
    setIsConnected(true);
    hideWindow();
    listenForTxAuthorizationRequests();
  };

  const endConnection = async () => {
    console.log("[INFO]: Ending connection.");
    setIsConnected(false);
  }

  const startSignIn = async () => {
    console.log("[INFO]: Start wallet sign in.")
    setCurrentPage(LOGIN_PAGE);
  }

  const startReset = async () => {
    console.log("[INFO]: Start wallet reset process.")
    setCurrentPage(RESET_PAGE);
  }

  const startCreate = async () => {
    console.log("[INFO]: Start wallet creation process.")
    setCurrentPage(CREATE_ACCOUNT_PAGE);
  }

  const startRecover = async () => {
    console.log("[INFO]: Start recovery process.")
    setCurrentPage(RECOVER_PAGE);
  }

  const cancelRecover = async () => {
    console.log("[INFO]: Cancel recovery process.")
    setCurrentPage(CREATE_OR_RECOVER_PAGE);
  }

  const cancelReset = async () => {
    console.log("[INFO]: Cancel reset process.")
    setCurrentPage(SIGN_IN_OR_RESET_PAGE);
  }

  return (
    <div className="App">
      <Container className="page">
        {currentPage === LOADING_PAGE && (
          <Loading />
        )}
        {currentPage === SIGN_IN_OR_RESET_PAGE && (
          <SignInOrReset startSignIn={startSignIn} startReset={startReset} />
        )}
        {currentPage === CREATE_OR_RECOVER_PAGE && (
          <CreateOrRecover sendCreateOrRecover={sendCreateOrRecover} startCreate={startCreate} startRecover={startRecover} />
        )}
        {currentPage === RESET_PAGE && (
          <Reset endConnection={endConnection} resetAccount={resetAccount} cancelReset={cancelReset}/>
        )}
        {currentPage === RECOVER_PAGE && (
          <Recover hideWindow={hideWindow} sendPassword={sendPassword} sendMnemonic={sendMnemonic} cancelRecover={cancelRecover}/>
        )}
        {currentPage === CREATE_ACCOUNT_PAGE && (
          <CreateAccount
            recoveryPhrase={recoveryPhrase}
            sendPassword={sendPassword}
            endInitialConnectionPhase={endInitialConnectionPhase}
          />
        )}
        {currentPage === LOGIN_PAGE && (
          <SignIn
            sendPassword={sendPassword}
            endInitialConnectionPhase={endInitialConnectionPhase}
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
      </Container>
    </div>
  );
}

export default App;
