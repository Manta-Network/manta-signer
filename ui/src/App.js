import './App.css';
import Authorize from './pages/Authorize';
import CreateAccount from './pages/CreateAccount';
import ResetAccount from './pages/ResetAccount';
import Loading from './pages/Loading';
import SignIn from './pages/SignIn';
import { Container } from 'semantic-ui-react';
import { appWindow } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api/tauri';
import { listen, once } from '@tauri-apps/api/event';
import { useState, useEffect } from 'react';

const LOADING_PAGE = 0;
const CREATE_ACCOUNT_PAGE = 1;
const LOGIN_PAGE = 2;
const AUTHORIZE_PAGE = 3;
const RESET_PAGE = 4;

function App() {
  const [currentPage, setCurrentPage] = useState(LOADING_PAGE);
  const [isConnected, setIsConnected] = useState(false);
  const [recoveryPhrase, setRecoveryPhrase] = useState(null);
  const [authorizationSummary, setAuthorizationSummary] = useState(null);

  useEffect(() => {
    if (isConnected) return;
    const beginInitialConnectionPhase = async () => {
      await once('connect', (event) => {
        console.log("[INFO]: Connect Event: ", event);
        let payload = event.payload;
        switch (payload.type) {
          case 'CreateAccount':
            setRecoveryPhrase(payload.content);
            setCurrentPage(CREATE_ACCOUNT_PAGE);
            break;
          case 'Login':
            setCurrentPage(LOGIN_PAGE);
            break;
          case 'Recover':
            setCurrentPage(RESET_PAGE);
            break;
          default:
            break;
        }
      });
    };
    listenForResetRequests();
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

  const listenForResetRequests = () => {
    console.log("[INFO]: Setup listener.");
    listen('reset', (event) => {
      console.log("[INFO]: Wake: ", event);
      setCurrentPage(RESET_PAGE);
      appWindow.show();
    });
  };

  const sendRecoveryInfo = async (phrase, password) => {
    console.log("[INFO]: Send info to signer server.");
    return await invoke('send_recovery_info', { phrase: phrase, password: password });
  };

  const sendPassword = async (password) => {
    console.log("[INFO]: Send password to signer server.");
    return await invoke('send_password', { password: password });
  };

  const stopPasswordPrompt = async () => {
    console.log("[INFO]: Stop password prompt.");
    await invoke('stop_password_prompt');
  };

  const endInitialConnectionPhase = async () => {
    console.log("[INFO]: End Initial Connection Phase");
    setIsConnected(true);
    hideWindow();
    listenForTxAuthorizationRequests();
  };

  return (
    <div className="App">
      <Container className="page">
        {currentPage === RESET_PAGE && (
          <ResetAccount
            sendRecoveryInfo={sendRecoveryInfo}
            hideWindow={hideWindow}
          />
        )}
        {currentPage === LOADING_PAGE && (
          <Loading/>
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
