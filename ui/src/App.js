import './App.css';
import React, { useState, useEffect } from 'react';
import CreateAccount from './pages/CreateAccount';
import { Container } from 'semantic-ui-react';
import { appWindow } from '@tauri-apps/api/window';
import AuthorizeTx from './pages/AuthorizeTx';
import SignIn from './pages/SignIn';

const CREATE_ACCOUNT_PAGE = 1;
const LOGIN_PAGE = 2;
const AUTHORIZE_TX_PAGE = 3;

function App() {
  const [currentPage, setCurrentPage] = useState(null);
  const [isConnected, setIsConnected] = useState(false);
  const [txSummary, setTxSummary] = useState(null);

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
    setCurrentPage(null);
    appWindow.hide();
  };

  const listenForTxAuthorizationRequests = () => {
    window.__TAURI__.event.listen('authorize', (event) => {
      appWindow.show();
      appWindow.center();
      appWindow.setAlwaysOnTop(true);
      setTxSummary(event.payload);
      setCurrentPage(AUTHORIZE_TX_PAGE);
    });
  };

  const loadPasswordToSignerServer = async (password) => {
    const should_retry = await window.__TAURI__.invoke('load_password', {
      password: password,
    });
    return should_retry;
  };

  const getRecoveryPhrase = async (password) => {
    const mnemonic = await window.__TAURI__.invoke('get_mnemonic', {
      password: password,
    });
    return mnemonic;
  };

  const endInitialConnectionPhase = async () => {
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
            loadPasswordToSignerServer={loadPasswordToSignerServer}
            endInitialConnectionPhase={endInitialConnectionPhase}
          />
        )}
        {currentPage === AUTHORIZE_TX_PAGE && (
          <AuthorizeTx
            txSummary={txSummary}
            loadPasswordToSignerServer={loadPasswordToSignerServer}
            hideWindow={hideWindow}
          />
        )}
      </Container>
    </div>
  );
}

export default App;
