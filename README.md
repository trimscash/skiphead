# skiphead
Forensic tool. Software that can separate files into n-byte sections, analyze them, and output them.

Useful when files are hidden and inserted every n bytes.

フォレンジックツール．ファイルをnバイトごとにskipして解析することができます．ファイルをnバイトごとに区切って解析し，出力できるソフト．

nバイトごとに別のファイルが挿入されている場合に便利です．


![image](https://github.com/trimscash/skiphead/assets/42578480/6e1f3be4-e066-4038-b2b0-a2a1b40e91ae)


You can search for file types by combining parameters.

パラメータを組み合わせてファイルの種類を探索できます，

![image](https://github.com/trimscash/skiphead/assets/42578480/f7fc47fa-433f-4851-ad3a-3d801326ae44)



# Setup
```
git clone https://github.com/trimscash/skiphead ~
cd skiphead
cargo build -r
echo "export PATH=\$PATH:\$HOME/skiphead/target/release" >> ~/.zshrc
source ~/.zshrc
```
and use it. 
Replace .zshrc with the one you are using

# Usage
```
Parse the header of the file skipped by n bytes and display the file type.
 skiphead can search for file types by combining parameters.
 Forensic app

Usage: skiphead [OPTIONS] <FILE>

Arguments:
  <FILE>

Options:
  -s <SKIP_NUMS>...
          Number of skips. Must be greater than 0 [default: 1 2 3] [short aliases: n]
  -l <PICK_LENGTH>...
          Length to pick up from that location. Must be greater than 0 [default: 0]
  -o <PICK_OFFSET>...
          Offset to start picking within that range. Must be greater than or equal to 0 [default: 0]
  -f <FILE_OFFSET>
          Offset to start parsing the entire file [default: 0]
  -c, --combinate
          Combinate param mode. default mode is one on one
  -x, --export-file
          Whether to output the file [aliases: output, export, output-file]
  -z, --only
          Only non bin file
  -p, --print
          Print head of buffer
      --output-directory <OUTPUT_DIRECTORY>
          Output directory path [default: ./skiphead_out]
  -h, --help
          Print help
```

# Example
### picoCTF 2023 Invisible WORDs

https://play.picoctf.org/practice/challenge/354

この問題を以下のwriteupを参考にしながら`skiphead`で解く．

Solve this problem with `skiphead`, referring to the following writeup. 

https://digitaltravesia.jp/CTF/picoCTF2023/picoCTF_2023_Writeup.html


![image](https://github.com/trimscash/skiphead/assets/42578480/6ba928fe-7b42-4c92-a294-1f4cfa8fdbd1)


ファイルオフセット`140 byte`から`4byte`ごとに`2byte`，別のファイルが挿入されている．

Every `4 bytes` from file offset `140 bytes` to `2 bytes`, another file is inserted. 

これを`skiphead`でやると以下のコマンドでできる．

This can be done with `skiphead` by the following command. 

```
skiphead output.bmp -f 140 -n 4 -l 2
```

![image](https://github.com/trimscash/skiphead/assets/42578480/69fe68d1-17ac-4343-b0ac-e320895b0c51)

このようにZIPヘッダであることがわかる．さらに，`--output`オプションをつけることで`./skiphead_out`にファイルを抽出することができる．

As you can see, it is a ZIP header. In addition, the `--output` option can be used to extract the file to `. /skiphead_out`. 

```
skiphead output.bmp -f 140 -n 4 -l 2 --output 
```

![image](https://github.com/trimscash/skiphead/assets/42578480/c422d393-f6f9-45d4-afd9-eb332af04e33)

これを以下のコマンドで展開し，`"{"`で文字列を抽出するとフラグが得られる．

This is expanded with the following command, and the flag is obtained by extracting the string with `"{"`. 

```
7z e skip_4_pick_offset_0_pick_length_2_file_offset_140
```

![image](https://github.com/trimscash/skiphead/assets/42578480/a653fb11-76ec-43e7-9222-f290f22a037b)

```
picoCTF{w0rd_d4wg_y0u_f0und_5h3113ys_m4573rp13c3_a23dfbd4}
```

# todo
- Contributions are welcome!
- ほしい機能があればコントリビュータになってください！
