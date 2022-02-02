import { EventRecord } from '@polkadot/types/interfaces';
import { ApiPromise } from '@polkadot/api';

export default function getFailedExtrinsicError(
  events: Array<EventRecord>,
  api: ApiPromise
): String {
  let errorMessage = null;

  events
    .filter(({ event }) =>
      // @ts-ignore FIXME
      api.events.system.ExtrinsicFailed.is(event)
    )
    .forEach(
      ({
        event: {
          data: [error, info]
        }
      }) => {
        // @ts-ignore FIXME
        if (error.isModule) {
          // for module errors, we have the section indexed, lookup
          // @ts-ignore FIXME
          const decoded = api.registry.findMetaError(error.asModule);
          // @ts-ignore FIXME
          const { docs, method, section } = decoded;
          errorMessage = `${section}.${method}: ${docs.join(' ')}`;
        } else {
          // Other, CannotLookup, BadOrigin, no extra info
          errorMessage = error.toString();
        }
      }
    );
  return errorMessage; // TODO: JS option type?
}
