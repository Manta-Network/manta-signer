### 编译

#### MacOS

#### Windows

##### 安装依赖

[Nodejs](https://nodejs.org/en/) 环境 
执行命令 `npm install -g yarn`

[golang](https://golang.org) 环境

[rust](https://www.rust-lang.org/) 环境

关于c语言的编译器，需要安装
[msys2](https://www.msys2.org/) 
安装完成后执行 

`pacman -Syu`

`pacman -Su`

`pacman -S --needed base-devel mingw-w64-x86_64-toolchain`

#### 编译

```
# 安装wails cli
go get github.com/wailsapp/wails/v2/cmd/wails
# 标记版本
git describe --tags > .version 
wails build
```

#### 打包

##### MacOS
```
chmod +x package.sh && \
export MANTA_SIGNER_SIGNING_IDENTITY=$identity \
AC_USERNAME=$username \
AC_PASSWORD=$password \
AC_PROVIDER=$provider && \
./package.sh
```

##### Windows


#### 运行

执行`build/bin`下面相关平台的二进制文件

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
$ curl --request POST --data-binary "@dist/darwin/manta-signer" http://localhost:9988/generateTransferZKP
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
$ curl --request POST --data-binary "@dist/darwin/manta-signer" http://localhost:9988/generateReclaimZKP
```

### 生成macos app DMG
