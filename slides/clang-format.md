author Linda_pp
date   13 Nov 2013
place  kabukiza tech#2
font   Ricty:h45

# clang-format

13 Nov 2013 歌舞伎座 tech#2

        @Linda_pp
        https://twitter.com/Linda_pp
        https://github.com/rhysd

### コーディングスタイルにまつわるお悩み

- チーム内でコーディングスタイルが決まっていて
  書きづらい
  - Google コーディングスタイルの80桁制限とか

    自分のスタイルで書いて後から修正したい

- 他人が書いたコードがぐちゃぐちゃで読みづらい

    手元で整形してから読みたい

- そもそも手で整形するのが面倒くさい

## それ clang-format で解決できるのでは

### clang-format とは
- C, C++, Objective-C のコードフォーマットツール
  - 指定したコードスタイルで自動整形してくれる
  - Clang の LibFormat を利用

  http://clang.llvm.org/docs/ClangFormat.html

- コマンドラインで利用

```sh
clang-format -style={style} [-i] {file}
```

    {style} = LLVM, Google, Chromium,
              Mozilla, WebKit

### // フォーマット例

```cpp
for(int i=0;i<4;++i){
    if(i%2==0) std::cout << hoge[i] << std::endl;
}
```

```cpp
for (int i = 0; i < 4; ++i) {
    if (i % 2 == 0) std::cout << hoge[i] << std::endl;
}
```

### // フォーマット例

```cpp
template<class ArrayL, class ArrayR>
constexpr bool operator()(ArrayL const& lhs, ArrayR const& rhs, size_t i) const
{
    return i == ls ? false :
           i == rs ? true :
           lhs[i] < rhs[i] ? false :
           lhs[i] > rhs[i] ? true :
           operator()(lhs, rhs, i+1);
}
```

```cpp
template <class ArrayL, class ArrayR>
constexpr bool operator()(ArrayL const& lhs,
                          ArrayR const& rhs, size_t i) const
{
    return i == ls
               ? false
               : i == rs ? true
                         : lhs[i] < rhs[i]
                               ? false
                               : lhs[i] > rhs[i]
                                     ? true
                                     : operator()(lhs, rhs,
                                                  i + 1);
}
```
### // フォーマット例

```cpp
std::cout << "Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the \"Software\"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:";
```

```cpp
std::cout
    << "Permission is hereby granted, free of charge, to "
       "any person obtaining a copy of this software and "
       "associated documentation files (the \"Software\"), "
       "to deal in the Software without restriction, "
       "including without limitation the rights to use, "
       "copy, modify, merge, publish, distribute, "
       "sublicense, and/or sell copies of the Software, "
       "and to permit persons to whom the Software is "
       "furnished to do so, subject to the following "
       "conditions:";
```

### スタイルを細かく設定する
- `-style` オプションに辞書を渡す

```sh
# Linux カーネルスタイル
clang-format -style='{ \
        BasedOnStyle: LLVM \
        IndentWidth: 8 \
        UseTab: Always \
        BreakBeforeBraces: Linux \
        AllowShortIfStatementsOnASingleLine: false \
        IndentCaseLabels: false }' \
        hoge.cpp
```

- ドキュメント

    http://clang.llvm.org/docs/ClangFormatStyleOptions.html

### 設定ファイルにスタイルの設定を保存する

```sh
# -dump-config の出力をファイルに保存
clang-format [...] -dump-config > .clang-format

# -style オプションにファイル名を渡す
clang-format -style=.clang-format {file}
```

### エディタから利用する

- Emacs

```lisp
(load "path/to/clang-format.el")
(global-set-key [C-M-tab] 'clang-format-region)
```

- VisualStudio
  - clang-format-plugin

    http://llvm.org/builds/

### エディタから利用する

- Vim
  - 公式の python スクリプトを使う

```vim
map <C-K> :pyf path/to/clang-format.py<CR>
imap <C-K> <ESC>:pyf path/to/clang-format.py<CR>i
```

  - vim-clang-format を使う
    - コード整形用のオペレータマッピングが使える
    - .vimrc で細かくスタイルを指定できる

        https://github.com/rhysd/vim-clang-format
        http://rhysd.hatenablog.com/entry/2013/08/26/231858

### まとめ
- clang-format で C や C++, Objective-C のコードを
  特定のスタイルに簡単に整形できる

- `-style` オプションに色々渡すことでスタイルを細かく
  カスタマイズできる

- 各エディタから利用できる 
  （Vim は vim-clang-format がおすすめ）
