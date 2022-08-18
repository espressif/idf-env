# idf-env

Tool for maintaining ESP-IDF environment.

## Quick start

### Quick start with `cargo`

```shell
cargo install idf-env --git https://github.com/espressif/idf-env
```

### Quick start with `PowerShell`

Install serial drivers for ESP boards on Windows. Execute following command in PowerShell:

```
Invoke-WebRequest 'https://dl.espressif.com/dl/idf-env/idf-env.exe' -OutFile .\idf-env.exe; .\idf-env.exe driver install --espressif --ftdi --silabs --wch
```

# Commands

## Working with configuration

File stored in esp_idf.json
```
idf-env config get
idf-env config get --property gitPath
idf-env config get --property python --idf-id esp-idf-618cf3b908db7b2ed74540bde5ba6605
idf-env config get --property python --idf-path "C:/esp/"
idf-env config add --idf-version "v4.2" --idf-path "C:/esp/" --python "C:/python/python.exe"
idf-env config add --name idf --idf-version "v4.2" --idf-path "C:/esp/" --python "C:/python/python.exe"
idf-env config edit
idf-env config rm id
```

### Working with launchers of ESP-IDF
```
idf-env launcher add --shell powershell --to windows-terminal --title "ESP-IDF 4.4" --idf-path "C:/esp/"
```

### Working with installations of ESP-IDF
```
idf-env idf install
idf-env idf install --idf-version "master" --installer "G:\idf-installer\build\esp-idf-tools-setup-online-unsigned.exe"
idf-env idf uninstall
idf-env idf reset --path "G:\esp-idf"
idf-env idf shell
idf-env idf build
```

### Working with Antivirus

```
idf-env antivirus get
idf-env antivirus get --property displayName
idf-env antivirus exclusion add --path "C:\....exe"
idf-env antivirus exclusion add --tool cmake
idf-env antivirus exclusion add --all
idf-env antivirus exclusion add --all --chunk 5
idf-env antivirus exclusion remove --path "C:\....exe"
idf-env antivirus exclusion remove --tool cmake
idf-env antivirus exclusion remove --all
idf-env antivirus exclusion list
idf-env antivirus exclusion manage
```


### Working with drivers

```
idf-env driver get
idf-env driver get --property DeviceID
idf-env driver get --property DeviceID --missing
```

Run in elevated shell - requires Administrator privileges.
Tools will request elevated privileges by UAC if necessary.

```
idf-env driver install --espressif --ftdi --silabs --wch
```

Download drivers without installation:

```
idf-env driver download --espressif --ftdi --silabs --wch
```

#### Links for manual download of drivers

- Espressif System - https://dl.espressif.com/dl/idf-driver/idf-driver-esp32-usb-jtag-2021-07-15.zip
- FTDI - https://www.ftdichip.com/Drivers/CDM/CDM%20v2.12.28%20WHQL%20Certified.zip
- Silabs - https://www.silabs.com/documents/public/software/CP210x_Universal_Windows_Driver.zip
- WHC - https://www.wch.cn/downloads/CH341SER_ZIP.html

### Working with Rust language for Xtensa

Boostrap whole Rust installation.


#### Installation for Rust toolchain Windows with GNU

```
idf-env rust install --default-host x86_64-pc-windows-gnu --extra-tools=mingw --extra-crates="ldproxy"
```

Installation with system version of MinGW:

```
idf-env rust install --default-host x86_64-pc-windows-gnu
```

Install specific version of the toolchain

```
idf-env rust install --default-host x86_64-pc-windows-gnu --toolchain-version 1.63.0.0 --extra-tools=mingw
```

#### Installation of Rust toolchain for Windows with MSVC

```
idf-env rust install
idf-env rust install --default-host x86_64-pc-windows-msvc --extra-tools=vctools
```

#### Other operations
```
idf-env rust reinstall
idf-env rust uninstall
```

### Web IDE Companion

```
idf-env companion start
idf-env companion start --port COM7
idf-env companion update
```

### Working with shell

```
idf-env shell append --variable path --path c:/Espressif/tools
```

### Working with certificates

Verify whether remote site is reachable and has https certificate recognized by the system:

```shell
idf-env certificate verify --url https://dl.espressif.com/dl/
```

Return codes:
- 0 - site is reachable
- 1 - site is not-reachable
