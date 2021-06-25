### 交叉编译
#### Windows
windows 需要安装x86_65-w64-mingw32-gcc的linker
安装方法brew install mingw-w64
但是编译还是有问题，暂时没解决

#### 运行

MacOS下执行在项目当前路径下执行dist/darwin/manta-daemon即可

### 接口规范
本daemon程序实现了符合restful规范的接口，运行在默认为localhost:9988地址
可以通过指定`--addr=0.0.0.0:12345`来实现在其他端口监听
提供3个接口供外部交互

---
1. /heartbeat

Method: GET

返回http_status_code = 200即为成功，表明本daemon程序正在运行

Example:

```
$ curl http://localhost:9988/heartbeat
```

2. /generateTransferZKP

Method: POST

Params:

    Query_param:
        app_version: 必须 app版本

Body:
    
    二进制payload

Response: 

    {
        "transfer_zkp":   "0x22023891",
        "daemon_version": "0.1.1",
        "app_version":    "0.1.1"
    }

Example:
```
$ curl --request POST --data-binary "@dist/darwin/manta-daemon" http://localhost:9988/generateTransferZKP
```

3. /generateReclaimZKP

Method: POST

Params:

    Query_param:
        app_version: 必须 app版本

Body:

    二进制payload

Response:

    {
        "reclaim_zkp":   "0x22023891",
        "daemon_version": "0.1.1",
        "app_version":    "0.1.1"
    }

Example:
```
$ curl --request POST --data-binary "@dist/darwin/manta-daemon" http://localhost:9988/generateReclaimZKP
```
