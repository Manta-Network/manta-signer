import './App.css';
import Account from './pages/Account';
import Authorize from './pages/Authorize';
import CreateAccount from './pages/CreateAccount';
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
const ACCOUNT_PAGE = 4;

function App() {
  const [currentPage, setCurrentPage] = useState(LOADING_PAGE);
  const [isConnected, setIsConnected] = useState(false);
  const [recoveryPhrase, setRecoveryPhrase] = useState(null);
  const [authorizationSummary, setAuthorizationSummary] = useState(null);
  const [receivingKeys, setReceivingKeys] = useState(null);

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
    console.log("[INFO]: Setup tx authorization listener.");
    listen('authorize', (event) => {
      console.log("[INFO]: Wake: ", event);
      setAuthorizationSummary(event.payload);
      setCurrentPage(AUTHORIZE_PAGE);
      appWindow.show();
    });
  };

  const listenForAccountRequests = () => {
    console.log("[INFO]: Setup account display listener.");
    listen('account', (event) => {
      console.log("[INFO]: Wake: ", event);
      if (currentPage !== AUTHORIZE_PAGE) {
        setCurrentPage(ACCOUNT_PAGE);
        appWindow.show();
      }
    });
  }

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
    await getReceivingKeys();
    setCurrentPage(ACCOUNT_PAGE);
    listenForTxAuthorizationRequests();
    listenForAccountRequests();
  };

  const getReceivingKeys = async () => {
    const receivingKeys = await invoke('receiving_keys');
    setReceivingKeys(receivingKeys)
  }

  return (
    <div className="App">
      <Container className="page">
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
        {currentPage === ACCOUNT_PAGE && (
          <Account
          receivingKeys={receivingKeys}
          hideWindow={hideWindow}
          />
        )}
      </Container>
    </div>
  );
}

export default App;
