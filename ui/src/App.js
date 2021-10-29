import React, { useEffect, useState } from 'react';
import logo from './logo.svg';
import './App.css';
import TitleBar from './TitleBar';
import CreateAccount from './pages/CreacteAccount';
import { Container } from 'semantic-ui-react';

const CREATE_ACCOUNT_PAGE = 1;

function App() {
  const [currentPage, setCurrentPage] = useState(CREATE_ACCOUNT_PAGE);

  if (window.__TAURI__) {
    window.__TAURI__.invoke('connect').then((event) => {
      switch (event) {
        case 'create-account':
          setCurrentPage(CREATE_ACCOUNT_PAGE);
          break;
        default:
          console.log('end connect?');
          // endConnect();
          break;
      }
    });
  }

  // function getMnemonic() {
  //   let password = document.getElementById('new-password');
  //   if (password.reportValidity()) {
  //     let promise = window.__TAURI__.invoke('get_mnemonic', {
  //       password: password.value,
  //     });
  //     password.value = '';
  //     document.getElementById('create-account-form').hidden = true;
  //     document.getElementById('mnemonic-header').hidden = false;
  //     document.getElementById('mnemonic').hidden = false;
  //     document.getElementById('mnemonic').innerText = '...';
  //     promise.then((event) => {
  //       document.getElementById('mnemonic').innerText = event;
  //       document.getElementById('close').hidden = false;
  //     });
  //   }
  // }

  function endConnect() {
    window.__TAURI__.invoke('end_connect').then(() => {
      setCurrentPage(CREATE_ACCOUNT_PAGE);
      // document.getElementById('create-account').hidden = true;
      // document.getElementById('authorize').hidden = false;
    });
    window.__TAURI__.event.listen('authorize', (event) => {
      // TODO: Show the prompt to the user.
      console.log(event);
    });
  }

  // function loadPassword() {
  //   let password = document.getElementById('known-password');
  //   if (password.reportValidity()) {
  //     let promise = window.__TAURI__.invoke('load_password', {
  //       password: password.value,
  //     });
  //     password.value = '';
  //     promise.then(() => {});
  //   }
  // }

  // function clearPassword() {
  //   document.getElementById('known-password').value = '';
  //   window.__TAURI__.invoke('clear_password').then(() => {});
  // }

  return (
    // <body>
    <div className="App">
      <Container className="page">
        {currentPage === CREATE_ACCOUNT_PAGE && <CreateAccount />}
      </Container>
    </div>
    // </body>
  );
}

export default App;
