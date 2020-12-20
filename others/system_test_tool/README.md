システムテストを実施するには [online-judge-tools](https://github.com/online-judge-tools/oj) が必要です。

```shell
pip3 install online-judge-tools
```

1. verify 用の問題を解く関数 `solve(input: &str, res: &mut String)` を実装します。
    - 標準入力の代わりに `input` から入力を読みこんで、`res` に答えを書きこんでください。
2. 上の関数を `system_test_tool::system_test(solve: F, problem_url: &str)` に問題 URL と一緒に渡してシステムテストを実行します。
    - 対象は `oj download --system problem_url` で取得できるテストケースです。

使用例

- [UnionFind](/algo/union_find/tests/unionfind.rs)
- [ProconReader](/others/procon_reader/tests/many_aplusb.rs)
