# hxdown
[![hxdown](https://github.com/PineAG/hxdown/actions/workflows/rust.yml/badge.svg)](https://github.com/PineAG/hxdown/actions/workflows/rust.yml)

hxdown(HXD Downloader) is a CLI gallery downloader made with love and Rust.

hxdown是一款在命令行运行的(你懂的)画像下载软件。基于Rust编写。

[下载/Download](https://github.com/PineAG/hxdown/releases/tag/latest)

## Usage

```
hxdown https://e-hentai.org/g/1741679/e33add3ab7/
```

## Supported Websites
* E-Hentai
* NHentai

## 使用代理访问
可通过环境变量`https_proxy`或`all_proxy`设置

### Windows

在`C:\Users\PineAG\Documents\WindowsPowerShell\Microsoft.PowerShell_profile.ps1`中加入下列语句

```
$env:all_proxy='http://host:port'
```

### macOS/Linux

在~/.bashrc或~/.zshrc中加入下列语句

```
export all_proxy='http://host:port'
```
