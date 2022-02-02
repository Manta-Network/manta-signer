// @ts-nocheck
export class CoinSelection {
  constructor(coins, totalValueAtomicUnits, targetValueAtomicUnits, assetId) {
    this.coins = coins;
    this.totalValueAtomicUnits = totalValueAtomicUnits;
    this.targetValueAtomicUnits = targetValueAtomicUnits;
    this.assetId = assetId;
    this.changeValueAtomicUnits = totalValueAtomicUnits.sub(
      targetValueAtomicUnits
    );
  }

  numberOfZeroCoinsRequired() {
    return Math.max(0, 2 - this.coins.length);
  }

  last() {
    const last = this.coins[this.coins.length - 1];
    if (!last) {
      throw new Error('Coin selection does not contain any coins');
    }
    return last;
  }

  secondLast() {
    const secondLast = this.coins[this.coins.length - 2];
    if (!secondLast) {
      throw new Error('Coin selection contains fewer than two coins');
    }
    return secondLast;
  }
}
