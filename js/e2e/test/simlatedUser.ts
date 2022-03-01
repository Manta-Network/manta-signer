import {
  ALICE_SIGNER_DAEMON_URL,
  BOB_SIGNER_DAEMON_URL,
  CHARLIE_SIGNER_DAEMON_URL,
  DAVE_SIGNER_DAEMON_URL,
  ALICE_POLKADOT_JS_SIGNER,
  BOB_POLKADOT_JS_SIGNER,
  CHARLIE_POLKADOT_JS_SIGNER,
  DAVE_POLKADOT_JS_SIGNER,
  ALICE_PUBLIC_ADDRESS,
  BOB_PUBLIC_ADDRESS,
  CHARLIE_PUBLIC_ADDRESS,
  DAVE_PUBLIC_ADDRESS
} from './constants';
import { NodeJsSignerInterfaceConfig } from 'signer-interface';

export type SimulatedUser = {
  name: string;
  publicAddress: string;
  polkadotJsSigner: string;
  signerInterface: NodeJsSignerInterfaceConfig;
};

// Alice is the "bank" because she stars with all of the money on Dolphin;
// End-to-end tests should not include Alice except during setup
export const Alice: SimulatedUser = {
  name: 'Alice',
  publicAddress: ALICE_PUBLIC_ADDRESS,
  polkadotJsSigner: ALICE_POLKADOT_JS_SIGNER,
  signerInterface: new NodeJsSignerInterfaceConfig(
    1,
    ALICE_SIGNER_DAEMON_URL,
    'Alice'
  )
};

export const Bob: SimulatedUser = {
  name: 'Bob',
  publicAddress: BOB_PUBLIC_ADDRESS,
  polkadotJsSigner: BOB_POLKADOT_JS_SIGNER,
  signerInterface: new NodeJsSignerInterfaceConfig(
    1,
    BOB_SIGNER_DAEMON_URL,
    'Bob'
  )
};

export const Charlie: SimulatedUser = {
  name: 'Charlie',
  publicAddress: CHARLIE_PUBLIC_ADDRESS,
  polkadotJsSigner: CHARLIE_POLKADOT_JS_SIGNER,
  signerInterface: new NodeJsSignerInterfaceConfig(
    1,
    CHARLIE_SIGNER_DAEMON_URL,
    'Charlie'
  )
};

export const Dave: SimulatedUser = {
  name: 'Dave',
  publicAddress: DAVE_PUBLIC_ADDRESS,
  polkadotJsSigner: DAVE_POLKADOT_JS_SIGNER,
  signerInterface: new NodeJsSignerInterfaceConfig(
    1,
    DAVE_SIGNER_DAEMON_URL,
    'Dave'
  )
};
