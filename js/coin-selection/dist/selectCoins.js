"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.selectCoins = void 0;
// @ts-nocheck
const BN = require("bn.js");
const coinSelection_js_1 = require("./coinSelection.js");
/**
 * Add eligible assets to our coin selection until we reach target amount
 * If less than two coins selected, add more if possible, to avoid necessity of
 * minting coins with zero value.
 */
const selectCoins = (targetValueAtomicUnits, spendableAssets, assetId) => {
    let totalValueAtomicUnits = new BN(0);
    const selectedCoins = [];
    spendableAssets
        .filter((asset) => asset.assetId === assetId)
        .forEach((asset) => {
        if (totalValueAtomicUnits.lt(targetValueAtomicUnits) ||
            selectedCoins.length < 2) {
            totalValueAtomicUnits = totalValueAtomicUnits.add(asset.valueAtomicUnits);
            selectedCoins.push(asset);
        }
    });
    if (totalValueAtomicUnits.lt(targetValueAtomicUnits)) {
        throw new Error('Coin selection; insufficient funds');
    }
    return new coinSelection_js_1.CoinSelection(selectedCoins, totalValueAtomicUnits, targetValueAtomicUnits, assetId);
};
exports.selectCoins = selectCoins;
//# sourceMappingURL=selectCoins.js.map