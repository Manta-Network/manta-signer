import { EventRecord, ExtrinsicStatus } from '@polkadot/types/interfaces';
import getFailedExtrinsicError from './GetFailedExtrinsicError';
import TxStatus from './TxStatus';

interface EventHandler {
  status: ExtrinsicStatus;
  events: EventRecord[];
}

export function makeTxResHandler(
  api,
  onSuccess = (block) => null,
  onFailure = (block, error) => null,
  onUpdate = (message) => null
) {
  return (input: EventHandler) => {
    let error;
    if (input.status.isInBlock || input.status.isFinalized) {
      error = getFailedExtrinsicError(input.events, api);
    }
    if (input.status.isInBlock && error) {
      onFailure(input.status.asInBlock.toString(), error);
    } else if (input.status.isFinalized && error) {
      onFailure(input.status.asFinalized.toString(), error);
    } else if (input.status.isFinalized) {
      onSuccess(input.status.asFinalized.toString());
    } else {
      onUpdate(input.status.type);
    }
  };
}

export function makeDefaultTxResHandler(api, setStatus, logStatus: boolean) {
  const onSuccess = (block) => {
    if (logStatus) {
      console.log(TxStatus.finalized(block));
    }
    setStatus(TxStatus.finalized(block));
  };
  const onFailure = (block, error) => {
    if (logStatus) {
      console.log(TxStatus.failed(block, error));
    }
    setStatus(TxStatus.failed(block, error));
  };
  const onUpdate = (message) => {
    if (logStatus) {
      console.log(TxStatus.processing(message));
    }
  };
  return makeTxResHandler(api, onSuccess, onFailure, onUpdate);
}
