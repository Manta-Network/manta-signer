import React from 'react';
import ReactDOM from 'react-dom';
import './fonts/ibm-plex/css/ibm-plex.css';
import './index.css';
import App from './App';
import 'semantic-ui-css/semantic.min.css';
import { HashRouter } from 'react-router-dom';

ReactDOM.render(
  <React.StrictMode>
    <HashRouter>
      <App />
    </HashRouter>
  </React.StrictMode>,
  document.getElementById('root')
);
