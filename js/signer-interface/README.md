# Manta Signer Interface

An interface to simplify interacting with Manta Signer

## Installation

`yarn add signer-interface`

## Usage

### Initialization

```js
// get an instance of polkadot.js api
import { ApiPromise, WsProvider } from '@polkadot/api';
const api = new ApiPromise({ provider, types, rpc: jsonrpc });
await api.isReady;

// Create a config deppending on your environment
import {
  BrowserSignerInterfaceConfig,
  NodeJsSignerInterfaceConfig
} from 'signer-interface';

const config = NodeJsSignerInterfaceConfig(
  1, // testnet
  'http://localhost:29986', // default Manta Signer url
  'User1'
);
const signerInterface = new SignerInterface(api, config);
```

### Recovering (or refreshing) a wallet

```js
// returns an array of ClientAsset
const myRecoveredAssets = signerInterface.recoverAccount();
```

### Doing a private transfer (or reclaim, similarly)

```js
// The transfer you want to do
const assetId = 1;
const targetValue = 100;
const receivingAddress = 'AAAAAAABBBBBBCCCCCCCDDDDDD...';

// install 'manta-coin-selection' with yarn
import { selectCoins, CoinSelection } from 'manta-coin-selection';

// returns a `CoinSelection`, which contains the selected coins and associated metadata
const coinSelection = selectCoins(targetValue, myRecoveredAssets, assetId);

const transaction = await signerInterface.buildExternalPrivateTransferTxs(
  receivingAddress,
  coinSelection
);

// Submit the transaction using polkadot.js

// wait ...

// If transaction fails, run:
signerInterface.cleanupTxFailure();

// If transaction succeeds, run:
signerInterface.cleanupTxSuccess();
```
