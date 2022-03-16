import logo from './logo.svg';
import './App.css';
import React, { useState } from "react";

function App() {
  const [version, setVersion] = useState("0.0");
  const ws = new WebSocket("ws://localhost:29987/ws");

  ws.onopen = (event) => {
      ws.send(JSON.stringify("version"));
  };

  ws.onmessage = (event) => {
      console.log(`Message is: ${event} ${event.data}`);
      const message = JSON.parse(event.data);

      if ("version" in message) {
          setVersion(message.version);
      } else {
          console.log(`Got other message: ${message}`);
      }
  };

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
            Manta Signer Version: {version}
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
