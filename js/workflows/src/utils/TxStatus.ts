const FINALIZED = 'finalized';
const FAILED = 'failed';
const PROCESSING = 'processing';

export default class TxStatus {
  status: string;
  block: string;
  message: string;

  constructor(status: string, block: string = null, message: string = null) {
    this.status = status;
    this.block = block;
    this.message = message;
  }

  static processing(message: string): TxStatus {
    return new TxStatus(PROCESSING, null, message);
  }

  static finalized(block: string): TxStatus {
    return new TxStatus(FINALIZED, block, null);
  }

  static failed(block: string, message: string): TxStatus {
    return new TxStatus(FAILED, block, message);
  }

  isProcessing(): boolean {
    return this.status === PROCESSING;
  }

  isFinalized(): boolean {
    return this.status === FINALIZED;
  }

  isFailed(): boolean {
    return this.status === FAILED;
  }

  toString(): string {
    let message = this.status;
    if (this.block) {
      message += `;\n block hash: ${this.block}`;
    }
    if (this.message) {
      message += `;\n ${this.message}`;
    }
    return message;
  }
}
