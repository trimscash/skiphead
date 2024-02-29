# skiphead
Forensic tool. Skip header and analyze it. Software that can separate files into n-byte sections, analyze them, and output them.

Useful when files are hidden and inserted every n bytes.

![image](https://github.com/trimscash/skiphead/assets/42578480/6e1f3be4-e066-4038-b2b0-a2a1b40e91ae)


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
Parse the header of the file skipped by n bytes and display the file type. Forensic app

Usage: skiphead [OPTIONS] <FILE>

Arguments:
  <FILE>  

Options:
  -n <SKIP_NUMS>...
          Number of skips. Must be greater than 0 [default: 1 2 3]
  -l <PICK_LENGTH>
          Length to pick up from that location. Must be greater than 0 [default: 1]
  -o <PICK_OFFSET>
          Offset to start picking within that range. Must be greater than or equal to 0 [default: 0]
  -f <FILE_OFFSET>
          Offset to start parsing the entire file [default: 0]
      --output
          Whether to output the file
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

https://digitaltravesia.jp/CTF/picoCTF2023/picoCTF_2023_Writeup.html

ファイルオフセット`140 byte`から`4byte`ごとに`2byte`，別のファイルが挿入されている．

これを`skiphead`でやると以下のコマンドでできる．

```
skiphead 
```



# Todo
- Added a mode to search files by combining the given skip, pick_offset, and pick_length arrays.
  	(与えたskip, pick_offset, pick_lengthの配列を組み合わせてファイルを探索するモードの追加．)
