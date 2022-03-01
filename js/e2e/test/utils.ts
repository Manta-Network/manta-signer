import { ApiPromise, WsProvider } from '@polkadot/api'
import { waitReady } from '@polkadot/wasm-crypto'
import { DolphinTypes } from 'dolphin-api'

export async function initDolphinApi (url: string): Promise<ApiPromise > {
  await waitReady()
  const wsProvider = new WsProvider(url)
  return ApiPromise.create({ provider: wsProvider, types: DolphinTypes })
}
