import { CoinSelection } from './coinSelection.js';
/**
 * Add eligible assets to our coin selection until we reach target amount
 * If less than two coins selected, add more if possible, to avoid necessity of
 * minting coins with zero value.
 */
export declare const selectCoins: (targetValueAtomicUnits: any, spendableAssets: any, assetId: any) => CoinSelection;
//# sourceMappingURL=selectCoins.d.ts.map