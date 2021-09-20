# Manta Signer

## Singer Spec

https://hackmd.io/0ZnJMf9jRwug2_ZBE0F4fA?both

## Compile

# 执行`build/bin`下面相关平台的二进制文件

### MacOS Arm

### MacOS x86-64

### Ubuntu 20.04 LTS (x86_64)

need `brew install FiloSottile/musl-cross/musl-cross`
(To be fix: Compilation issues)

### Windows (x86_64)

1. Install toolchain

```bash
rustup toolchain install stable-x86_64-pc-windows-gnu
```

2. Install x86_64-w64-mingw32-gcc linker:

```bash
brew install mingw-w64
```

(To be fixed: compilation issue)

## Running

### MacOS

```bash
dist/darwin/manta-daemon
```

## Interaction with DApp (web based)

via RESTful API on `localhost:9988`.
A customized port could be specified at `--addr=0.0.0.0:<port number>`:

## There are 3 RESTful APIs:

1. /heartbeat

Method: GET

http_status_code = 200 on success, indicate that the signer is running.

Example:

```
$ curl http://localhost:9988/heartbeat
```

2. /generateTransferZKP

Method: POST

Params:

    Query_param:
        app_version:  the current App version

Body:

    ZKP payload (in binary).

Response:

    {
        "transfer_zkp":   "0x22023891",
        "daemon_version": "0.1.1",
        "app_version":    "0.1.1"
    }

Example:

```
$ curl --request POST --data-binary "@dist/darwin/manta-signer" http://localhost:9988/generateTransferZKP
```

3. /generateReclaimZKP

Method: POST

Params:

    Query_param:
        app_version: the current App version

Body:

    ZKP payload (in binary).

Response:

    {
        "reclaim_zkp":   "0x22023891",
        "daemon_version": "0.1.1",
        "app_version":    "0.1.1"
    }

Example:

```
$ curl --request POST --data-binary "@dist/darwin/manta-signer" http://localhost:9988/generateReclaimZKP
```

### Generating MacOS `.dmg`

(TBD)
