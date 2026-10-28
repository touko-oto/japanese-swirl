# japanese swirl

日本語テキストを渦巻き並びに変換する。

## 仕様

渦巻き並びに変換する前に文字数をカウントし、10文字以上であれば、4*4以降の正方形で時計回りに文字位置から、  
一文字毎に区切り、置いていく。  
15文字であれば、残り1文字は全角スペースで埋める。  
17文字であれば、残り8文字は全角スペースで埋める。

## Input

日本語テキストが通常通りの順番で記載されているtxtファイル

### Input Sample Content

```plain
縦読みで文章を書いてもＡＩに見破られてインプレゾンビが湧くなら渦巻き状に並べればさすがに人間しか読めずインプレゾンビ湧かない説。
```

## Output

渦巻き並びに変換されたtxtファイル

### Output Sample Content

```plain
プレゾンビが湧縦
ンに人間しかく読
イがビ湧か読なみ
てすン。なめらで
れさゾ説いず渦文
らばレプンイ巻章
破れべ並に状きを
見にＩＡもてい書
```

## Usage

出力ファイルを指定しない場合、出力ファイル名は`input_result.txt`となる。  
```bash
jp-swl .\input.txt
```

出力ファイルを指定し、開始位置を指定しない場合、出力ファイル名は指定した名称となり、開始位置はrightup(右上)となる。  
```bash
jp-swl \tmp\input.txt -o \home\user1\outout.txt
```

開始位置を指定する場合(右上位置)  
```bash
jp-swl \tmp\input.txt -p rightup -o \home\user1\outout.txt
```

### 開始位置オプション

- 右上 `rightup` or `ru`
- 右下 `rightdown` or `rd`
- 左上 `leftup` or `lu`
- 左下 `leftdown` or `ld`
