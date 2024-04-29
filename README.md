![Build](https://github.com/MiSawa/ralgo/workflows/Build/badge.svg) ![Verify](https://github.com/MiSawa/ralgo/workflows/Verify/badge.svg)

# Ralgo
Rust implementation of ALGOrithms.
Try to be Presto but may be Largo.


## メモ

- [dependency_util](https://github.com/MiSawa/ralgo/tree/d469b65e174aa6b9962790488132978ebb8e5086/dependency_util) を使ってファイル間の依存関係を抽出していたが, [oj-verify](https://online-judge-tools.github.io/verification-helper/document.ja.html) が Rust に公式対応した関係でカスタム言語としての任意コード実行が動かなくなり, ひとまず依存関係を解決せず富豪的に全部 verify することにした.

