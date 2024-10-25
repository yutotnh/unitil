# unitil

EUC-JP の全角チルダ(8F A2 B7)を波ダッシュ(A1 C1) に変換するツール

## 変換の例

```console
$ cat a.txt
1～10
$ hexdump -C a.txt
00000000  31 8f a2 b7 31 30 0a                              |1...10.|
00000007
$ unitil a.txt     # 全角チルダを波ダッシュに変更
a.txt:
$ hexdump -C a.txt
00000000  31 a1 c1 31 30 0a                                 |1..10.|
00000006
$ cat a.txt
1〜10
```

## 波ダッシュと全角チルダの個数を数える例

```console
$ cat a.txt
1～10
10～20

$ hexdump -C a.txt
00000000  31 8f a2 b7 31 30 0a 31  30 8f a2 b7 32 30 0a     |1...10.10...20.|
0000000f

$ unitil a.txt --count
a.txt:
    Wave dash       (0xA1C1)   : 0
    Fullwidth Tilde (0x8FA2B7) : 2
```

## Install

### From crates.io

```console
cargo install unitil
```

### From source

```console
cargo install --git  https://github.com/yutotnh/unitil
```

## License

MIT License
