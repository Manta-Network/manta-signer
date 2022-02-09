// @ts-ignore FIXME
import {
  transferAssetWorkflow,
  mintAssetWorkflow,
  privateTransferAssetWorkflow,
  reclaimAssetWorkflow,
  recoverAccountWorkflow,
  deriveAddressWorkflow,
  resetStorageWorkflow
} from 'workflows';
import { waitReady } from '@polkadot/wasm-crypto';
import { ApiPromise } from '@polkadot/api';
import { initDolphinApi } from '../test/utils';
import { assert, expect } from 'chai';
import { readFileSync } from 'fs';
import { Alice, Bob, Charlie, Dave, SimulatedUser } from './simlatedUser';
import { LOCAL_NODE_URL } from './constants';

// TODO: To start simulation run the following:
// ```
// cd simulation
// cargo run --release <STEPS> --output=out.json
// ```
// where `<STEPS>` is the number of simulation steps to run.

function loadSimulation(path) {
  var simulation = null;
  try {
    const data = readFileSync(path, 'utf8');
    simulation = JSON.parse(data);
    console.log('[INFO] Simulation Configuration:', simulation.config);
  } catch (err) {
    console.log('Error loading simulation:', err);
  }
  return simulation;
}

async function getFinalPrivateBalances(api, user) {
  const assets = await recoverAccount(api, user);
  const balancesByAssetId = {};
  for (let i = 0; i <= 4; i++) {
    balancesByAssetId[i] = 0;
  }
  assets.forEach((asset) => {
    const prevBalance = balancesByAssetId[asset.assetId];
    balancesByAssetId[asset.assetId] =
      prevBalance + asset.valueAtomicUnits.toNumber();
  });
  return balancesByAssetId;
}

async function getFinalPublicBalance(api, user, assetId) {
  const balance = await api.query.mantaPay.balances(
    user.publicAddress,
    assetId
  );
  return balance.toNumber();
}

async function publicDeposit(
  api,
  recipient: SimulatedUser,
  assetId: number,
  valueAtomicUnits: number
) {
  return await transfer(
    api,
    Alice,
    assetId,
    valueAtomicUnits,
    recipient.publicAddress
  );
}

async function publicWithdraw(
  api,
  user: SimulatedUser,
  assetId: string,
  valueAtomicUnits: number
) {
  return await transfer(
    api,
    user,
    assetId,
    valueAtomicUnits,
    Alice.publicAddress
  );
}

async function mint(
  api,
  user: SimulatedUser,
  assetId: number,
  valueAtomicUnits: number
) {
  return await mintAssetWorkflow(
    {
      assetId: assetId,
      valueAtomicUnits: valueAtomicUnits,
      polkadotJsSigner: user.polkadotJsSigner,
      signerInterface: user.signerInterface
    },
    api
  );
}

async function transfer(api, user, assetId, valueAtomicUnits, receiver) {
  return await transferAssetWorkflow(
    {
      assetId: assetId,
      valueAtomicUnits: valueAtomicUnits,
      polkadotJsSigner: user.polkadotJsSigner,
      receiver: receiver
    },
    api
  );
}

async function privateTransfer(
  api,
  user: SimulatedUser,
  assetId: number,
  valueAtomicUnits: number,
  receiver: string
) {
  return await privateTransferAssetWorkflow(
    {
      assetId: assetId,
      valueAtomicUnits: valueAtomicUnits,
      receiver: receiver,
      polkadotJsSigner: user.polkadotJsSigner,
      signerInterface: user.signerInterface
    },
    api
  );
}

async function reclaim(
  api,
  user: SimulatedUser,
  assetId: number,
  valueAtomicUnits: number
) {
  return await reclaimAssetWorkflow(
    {
      assetId: assetId,
      valueAtomicUnits: valueAtomicUnits,
      polkadotJsSigner: user.polkadotJsSigner,
      signerInterface: user.signerInterface
    },
    api
  );
}

async function driveAddress(api, user: SimulatedUser) {
  return await deriveAddressWorkflow(
    {
      signerInterface: user.signerInterface
    },
    api
  );
}

async function recoverAccount(api, user: SimulatedUser) {
  return await recoverAccountWorkflow(
    {
      signerInterface: user.signerInterface
    },
    api
  );
}

function resetStorage(api, user: SimulatedUser) {
  resetStorageWorkflow(
    {
      signerInterface: user.signerInterface
    },
    api
  );
}

describe('Manta e2e test suite', async () => {
  let api: ApiPromise;

  before(async () => {
    await waitReady();
    api = await initDolphinApi(LOCAL_NODE_URL);
  });

  describe('simulation', () => {
    it('runs', async () => {
      const simulation = loadSimulation('./simulation/out.json');
      assert.notEqual(simulation, null, 'Unable to load simulation file.');
      assert.equal(simulation.config.starting_account_count, 3);
      assert.equal(simulation.config.new_account_sampling_cycle, 0);

      // No Alice because she has all the money to begin with on Dolphin
      const users = [Bob, Charlie, Dave];

      for (let i = 0; i < simulation.initial_accounts.length; i++) {
        const user = users[i];
        console.log('\n[INFO] Setting up account for:', user.name, '\n');
        const balances: Record<string, number> =
          simulation.initial_accounts[i].public;
        for (const [assetIdString, valueAtomicUnits] of Object.entries(
          balances
        )) {
          const assetId = parseInt(assetIdString);
          if (valueAtomicUnits > 0) {
            const status = await publicDeposit(
              api,
              user,
              assetId,
              valueAtomicUnits
            );
            expect(status.isFinalized()).equal(true);
          }
        }
      }

      for (let i = 0; i < simulation.updates.length; i++) {
        const next = simulation.updates[i];
        console.log('\n[INFO] Running Update:', next, '\n');
        if (next.type === 'PublicDeposit') {
          const publicDepositStatus = await publicDeposit(
            api,
            users[next.account_index],
            next.asset.id,
            next.asset.value
          );
          if (next.asset.value > 0) {
            expect(publicDepositStatus.isFinalized()).equal(true);
          } else {
            expect(publicDepositStatus.isFinalized()).equal(false);
          }
        } else if (next.type === 'PublicWithdraw') {
          const publicWithdrawStatus = await publicWithdraw(
            api,
            users[next.account_index],
            next.asset.id,
            next.asset.value
          );
          expect(publicWithdrawStatus.isFinalized()).equal(true);
        } else if (next.type === 'Mint') {
          const mintStatus = await mint(
            api,
            users[next.source_index],
            next.asset.id,
            next.asset.value
          );
          expect(mintStatus.isFinalized()).equal(true);
        } else if (next.type === 'PrivateTransfer') {
          const receivingAddress = await driveAddress(
            api,
            users[next.receiver_index]
          );
          const privateTransferStatus = await privateTransfer(
            api,
            users[next.sender_index],
            next.asset.id,
            next.asset.value,
            receivingAddress
          );
          expect(privateTransferStatus.isFinalized()).equal(true);
        } else if (next.type === 'Reclaim') {
          const reclaimStatus = await reclaim(
            api,
            users[next.sender_index],
            next.asset.id,
            next.asset.value
          );
          expect(reclaimStatus.isFinalized()).equal(true);
        } else {
          throw Error('Unknown update');
        }
      }

      for (let i = 0; i < simulation.final_accounts.length; i++) {
        const user = users[i];
        assert.notEqual(user, undefined);
        console.log(
          '\n[INFO] Checking final simulation state for:',
          user.name,
          '\n'
        );
        const publicBalancesExpected: Record<string, number> =
          simulation.final_accounts[i].public;
        for (const [assetId, value] of Object.entries(publicBalancesExpected)) {
          const expected = value;
          const actual = await getFinalPublicBalance(api, users[i], assetId);
          console.log(`Expecting public balance of ${expected} for user=${user.name} and assetId=${assetId}`);
          assert.equal(expected, actual);
        }
        const privateBalancesExpected: Record<string, number> =
          simulation.final_accounts[i].secret;
        const privateBalancesActual = await getFinalPrivateBalances(
          api,
          users[i]
        );
        for (const [assetId, value] of Object.entries(
          privateBalancesExpected
        )) {
          const expected = value;
          const actual = privateBalancesActual[assetId];
          console.log(`Expecting private balance of ${expected} for user=${user.name} and assetId=${assetId}`);
          assert.equal(expected, actual);
        }
      }
    }).timeout(100000000000);
  });

  after(() => {
    resetStorage(api, Alice);
    resetStorage(api, Bob);
    resetStorage(api, Charlie);
    resetStorage(api, Dave);
    api.disconnect();
  });
});
