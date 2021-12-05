import './App.css';
import React, { useState, useEffect } from 'react';
import CreateAccount from './pages/CreateAccount';
import { Container } from 'semantic-ui-react';
import { appWindow } from '@tauri-apps/api/window';
import Authorize from './pages/Authorize';
import SignIn from './pages/SignIn';

const CREATE_ACCOUNT_PAGE = 1;
const LOGIN_PAGE = 2;
const AUTHORIZE_PAGE = 3;

function App() {
  const [currentPage, setCurrentPage] = useState(null);
  const [isConnected, setIsConnected] = useState(false);
  const [authorizationSummary, setAuthorizationSummary] = useState(null);

  useEffect(() => {
    if (isConnected) return;
    const beginInitialConnectionPhase = async () => {
      const event = await window.__TAURI__.invoke('connect');
      switch (event) {
        case 'create-account':
          setCurrentPage(CREATE_ACCOUNT_PAGE);
          break;
        case 'setup-authorization':
          setCurrentPage(LOGIN_PAGE);
          break;
        default:
          break;
      }
    };
    beginInitialConnectionPhase();
  }, [isConnected]);

  const hideWindow = () => {
    console.log("[INFO]: HIDE.");
    setCurrentPage(null);
    appWindow.hide();
  };

  const listenForTxAuthorizationRequests = () => {
    console.log("[INFO]: Setup listener.");
    window.__TAURI__.event.listen('authorize', (event) => {
      console.log("[INFO]: WAKE: ", event);
      appWindow.show();
      appWindow.center();
      appWindow.setAlwaysOnTop(true);
      setAuthorizationSummary(event.payload);
      setCurrentPage(AUTHORIZE_PAGE);
    });
  };

  const sendPassword = async (password) => {
    console.log("[INFO]: Send password to signer server.");
    const should_retry = await window.__TAURI__.invoke('send_password', {
      password: password,
    });
    return should_retry;
  };

  const stopPasswordPrompt = async () => {
    console.log("[INFO]: Stop password prompt.");
    await window.__TAURI__.invoke('stop_password_prompt');
  };

  const getRecoveryPhrase = async (password) => {
    console.log("[INFO]: Get recovery phase.");
    const mnemonic = await window.__TAURI__.invoke('get_mnemonic', {
      password: password,
    });
    return mnemonic;
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
        {currentPage === CREATE_ACCOUNT_PAGE && (
          <CreateAccount
            getRecoveryPhrase={getRecoveryPhrase}
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
