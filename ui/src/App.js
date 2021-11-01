import React, { useState, useEffect } from 'react';
import './App.css';
import CreateAccount from './pages/CreacteAccount';
import { Container } from 'semantic-ui-react';
import { appWindow } from '@tauri-apps/api/window';
import AuthorizeTx from './pages/AuthorizeTx';
import Login from './pages/Login';

const CREATE_ACCOUNT_PAGE = 1;
const LOGIN_PAGE = 2;
const AUTHORIZE_TX_PAGE = 3;

function App() {
  const [currentPage, setCurrentPage] = useState(null);
  const [isConnected, setIsConnected] = useState(false);
  const [txSummary, setTxSummary] = useState(null);

  useEffect(() => {
    if (isConnected) return;
    const connect = async () => {
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
    connect();
  }, [isConnected]);

  async function disconnect() {
    await window.__TAURI__.invoke('clear_password');
    setIsConnected(false);
  }

  // todo: get rid of
  function isNonsensitiveTransaction(payload) {
    return (
      payload === 'recover_account' ||
      payload === 'derive_shielded_address' ||
      payload === 'mint' ||
      payload === 'generate_asset'
    );
  }

  async function listen() {
    setIsConnected(true);
    appWindow.hide();
    window.__TAURI__.event.listen('authorize', (event) => {
      appWindow.show();
      appWindow.center();
      appWindow.setAlwaysOnTop(true);
      if (isNonsensitiveTransaction(event.payload)) {
        setCurrentPage(LOGIN_PAGE);
      } else {
        setTxSummary(event.payload);
        setCurrentPage(AUTHORIZE_TX_PAGE);
      }
    });
  }

  async function loadPassword(password) {
    await window.__TAURI__.invoke('load_password', {
      password: password,
    });
    setIsConnected(true);
  }

  return (
    <div className="App">
      <Container className="page">
        {currentPage === CREATE_ACCOUNT_PAGE && (
          <CreateAccount listen={listen} loadPassword={loadPassword} />
        )}
        {currentPage === LOGIN_PAGE && (
          <Login listen={listen} loadPassword={loadPassword} />
        )}
        {currentPage === AUTHORIZE_TX_PAGE && (
          <AuthorizeTx
            txSummary={txSummary}
            disconnect={disconnect}
            listen={listen}
            loadPassword={loadPassword}
          />
        )}
      </Container>
    </div>
  );
}

export default App;
