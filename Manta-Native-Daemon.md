# Manta Native Daemon Specification

## Overview

Manta Native App is a deamon that runs in the background, its major function is running zero-knowledge proof* generation natively for the Manta Web App running in the browser. 

1. Manta Web App (a.k.a. App): developed by Manta Team and hosted by Manta, e.g. dapp-test.manta.network. The user may open Manta Web App directly from her browser (like Zoom) or click on Manta Daemon (like Dropbox).

2. Manta Daemon (a.k.a. Daemon): service that runs in the background (OS can be Mac or Win) to generate zero-knowledge proofs* for Manta Web App. The communication between App and Daemon could be REST API or others. This is the part to be implemented by the contractor.

*Manta Team will provide zero-knowledge proof code in Rust, as a black box.

## Manta Daemon Core Functions

There are two functions in the Daemon that the Web App calls. These functions are implemented by manta and take the following arguments (pending minor updates):

1. `GenerateTransferZKP`(payload, app_version), where payload is a  serialized binary file, this function returns the following to the web app:
```json
{
   transfer_zkp: zkp_binary,
   daemon_version: version_number_of_daemon,
   app_version: version_number_of_app_of_input,
}
```
  
2. `GenerateReclaimZKP`(payload, app_version), where payload is a  serialized binary file, this function returns the following to the web app:
```json
{
    reclaim_zkp: zkp_binary,
    daemon_version: version_number_of_daemon,
    app_version: version_number_of_app_of_input,
}
```

## Manta Daemon UI and Packaging

The manta daemon should be packaged into an install file in Mac/Win.  The daemon can be opened in two ways:

1. from click the icon
2. from Manta Web App with a link (like Zoom), this means Manta Web App should be able detect whether there is a daemon is running, if there isn't, Manta Web App need to show to the user the link to install/open Manta Daemon.

## Communications between the Manta Web App and Daemon
This part should be figured out by the out sourcing company, e.g. Daemon runs a webserver locally with a REST API or any other reasonable design.

## Security
Since the payload contains secret imformation, the payload will only live inside the the rust code that Manta team provide, and should not write/output to local disk or transferred via internet to a remote server.

## Delivery

The out-sourcing company should deliver the following to Manta:

1. The source code of the Mac/Win App
2. The build instructions to build the App from the source code to installation files in Mac/Win 
3. The test cases that have reasonable test coverage

Manta team will also coordinate with the out-sourcing company with the integration test of Web App and Daemon.