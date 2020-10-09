# pxls2

[heiwa4126/pxls1: あるパスに置かれたJSONファイルを解析して Excelファイルを作るツール](https://github.com/heiwa4126/pxls1)
を、勉強用にRustで書き直したもの。

# もくじ

- [pxls2](#pxls2)
- [もくじ](#もくじ)
- [インストール](#インストール)
- [使い方](#使い方)
- [TODO](#todo)


# インストール

```sh
make release
sudo cp ./target/release/pxls2 /usr/local/bin
# ↓おこのみで実行。ディスクの空きが増える
make clean
```

# 使い方

```
pxls2 <JSONファイルのあるパス> <出力するExcelファイル>
pxls2 -y <JSONファイルのあるパス> <出力するYAMLファイル>
pxls2 [-h|-v]
```

options:
- -y    YAMLモード
- -h    ヘルプの表示
- -v    バージョンの表示

# TODO

- Makefileが雑すぎるのを直していく。ほんとにtask runnerにしかなってない。
- Rustのコードの中身もなんとかする。main.rsが特に汚い。
  cli parserを切り離してかつ拡張しやすくするのが結構めんどうRustの場合。他人のコード読んで学ぶ。
