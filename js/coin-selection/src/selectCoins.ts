// @ts-nocheck
import * as BN from 'bn.js';
import { CoinSelection } from './coinSelection.js';

/**
 * Add eligible assets to our coin selection until we reach target amount
 * If less than two coins selected, add more if possible, to avoid necessity of
 * minting coins with zero value.
 */
export const selectCoins = (
  targetValueAtomicUnits,
  spendableAssets,
  assetId
) => {
  let totalValueAtomicUnits = new BN(0);
  const selectedCoins = [];
  spendableAssets
    .filter((asset) => asset.assetId === assetId)
    .forEach((asset) => {
      if (
        totalValueAtomicUnits.lt(targetValueAtomicUnits) ||
        selectedCoins.length < 2
      ) {
        totalValueAtomicUnits = totalValueAtomicUnits.add(
          asset.valueAtomicUnits
        );
        selectedCoins.push(asset);
      }
    });

  if (totalValueAtomicUnits.lt(targetValueAtomicUnits)) {
    throw new Error('Coin selection; insufficient funds');
  }

  return new CoinSelection(
    selectedCoins,
    totalValueAtomicUnits,
    targetValueAtomicUnits,
    assetId
  );
};
