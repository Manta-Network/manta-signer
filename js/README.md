# Manta JS

This workspace includes the following packages:

|    name          |       path          |        description                      |     status    |
|------------------|---------------------|-----------------------------------------|---------------|
| dophin-api       | `./dolphin-api`     |  api for dolphin nodes.                 |  experimental |
| workflows        | `./workflows`       |  logical workflows with extrinsics.     |  experimental |
| e2e              | `./e2e`             |  end-to-end test suites.                |  experimental |
| singer-interface | `/signer-interface` |  typescript interface for manta-signer. |  experimental |

## Build
Tested environment:
* node: `v16.x`
* yarn: `1.22.x`

```yarn install && yarn build```

## Set up

1. Make sure you have a node running that you want to connect to.

```bash
    git clone -b dolphin git@github.com:Manta-Network/Manta.git
    cd Manta
    cargo build --release
    ./target/release/manta --dev --tmp --alice --unsafe-ws-external
```

Which will be live at `ws://127.0.0.1:9944`.

2. [Run manta-signer test server](https://github.com/Manta-Network/manta-signer/tree/master/examples)


## Tests
You can run the entire testsuite with

```yarn run e2e test```


### Create a new test
The test suite is set up using [mocha](https://mochajs.org/) as the test runner and [chai](https://www.chaijs.com/) for the assertion library.
Both projects have good documentation on how to use them.

Add a new test by adding a `describe()` call in `e2e/test/test.ts`.


## Workflows
To create a new workflow to be uses in `dev-cli` or `e2e` tests,
start by choosing a name for your workflow. Then create a copy of
`/manta-workflows/src/workflows/init_asset_workflow.ts` with the new name, and
replace all instances of `initAssetWorkflow` in the file with the new name.

Both repositories already have `manta-workflows` as a dependency, so to use a
workflow recompile the project:

```yarn run workflows build```

And then import your workflow like

``` import { initAssetWorkflow } from 'manta-workflows';```
