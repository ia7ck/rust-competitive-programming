var searchIndex={};
searchIndex["binary_search"] = {"doc":"","i":[[8,"BinarySearch","binary_search","ソート済の列に対して二分法で\"境目\"を探します。",null,null],[10,"lower_bound","","",0,[[["t"],["self"]],["usize"]]],[10,"upper_bound","","",0,[[["t"],["self"]],["usize"]]],[10,"split_by","","",0,[[["t"],["self"]]]]],"p":[[8,"BinarySearch"]]};
searchIndex["factorization"] = {"doc":"","i":[[5,"min_factors","factorization","「k を割る最小の素数」をエラトステネスのふるいの要領で 2 以上 n 未満の全ての k について計算します。 #…",null,[[["usize"]],[["usize"],["vec",["usize"]]]]]],"p":[]};
searchIndex["fenwick_tree"] = {"doc":"","i":[[3,"FenwickTree","fenwick_tree","",null,null],[11,"new","","長さ `n` の列を作り、初期値 `e` で埋めます。雰囲気は `let mut a = vec![e; n];`…",0,[[["usize"],["t"]],["self"]]],[11,"add","","`k` 番目の値に `x` を足します。`k` は 0-indexed です。`a[k] += x;`",0,[[["self"],["usize"],["t"]]]],[11,"sum","","区間和を計算します。`range` が `l..r` だとして `a[l..r].iter().sum();`…",0,[[["usize"],["self"],["range",["usize"]]],["t"]]],[11,"from","","",0,[[["t"]],["t"]]],[11,"into","","",0,[[],["u"]]],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"try_into","","",0,[[],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]],[11,"type_id","","",0,[[["self"]],["typeid"]]],[11,"vzip","","",0,[[],["v"]]]],"p":[[3,"FenwickTree"]]};
searchIndex["grid"] = {"doc":"","i":[[3,"Adjacent","grid","グリッドグラフで現在位置の周辺を走査したいときに使えます。",null,null],[11,"new","","隣接 4 方向を走査する例です。 # Examples `use grid::Adjacent; const…",0,[[["i"],["usize"]],["self"]]],[11,"from","","",0,[[["t"]],["t"]]],[11,"into","","",0,[[],["u"]]],[11,"into_iter","","",0,[[],["i"]]],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"try_into","","",0,[[],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]],[11,"type_id","","",0,[[["self"]],["typeid"]]],[11,"next","","",0,[[["self"]],["option"]]]],"p":[[3,"Adjacent"]]};
searchIndex["next_permutation"] = {"doc":"","i":[[8,"NextPermutation","next_permutation","",null,null],[10,"next_permutation","","",0,[[["self"]],["bool"]]]],"p":[[8,"NextPermutation"]]};
searchIndex["procon_reader"] = {"doc":"","i":[[3,"ProconReader","procon_reader","",null,null],[11,"new","","標準入力から読み込みたいときの例です。 ファイルからの読み込みは `BufRead` の Examples…",0,[[["r"]],["self"]]],[11,"get","","空白・改行で区切られている値を取得します。適宜、型アノテーションをつけてください。 # Examples ```…",0,[[["self"]],["t"]]],[11,"get_vec","","よくある「空白区切りの数値を `n` 個」取得したいときなどに使えます。 # Examples ``` use…",0,[[["self"],["usize"]],["vec"]]],[11,"from","","",0,[[["t"]],["t"]]],[11,"into","","",0,[[],["u"]]],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"try_into","","",0,[[],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]],[11,"type_id","","",0,[[["self"]],["typeid"]]]],"p":[[3,"ProconReader"]]};
searchIndex["rolling_hash"] = {"doc":"","i":[[3,"RollingHash","rolling_hash","Rolling Hash です。O(文字列長) の前計算をしたうえで、部分文字列のハッシュ値を O(1) で計算します。",null,null],[11,"new","","",0,[[],["self"]]],[11,"get","","`range` が指す範囲の部分文字列のハッシュ値を返します。",0,[[["usize"],["self"],["range",["usize"]]],["u64"]]],[11,"connect","","2 つの文字列を連結したときのハッシュ値を返します。",0,[[["usize"],["self"],["u64"]],["u64"]]],[11,"from","","",0,[[["t"]],["t"]]],[11,"into","","",0,[[],["u"]]],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"try_into","","",0,[[],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]],[11,"type_id","","",0,[[["self"]],["typeid"]]],[11,"vzip","","",0,[[],["v"]]]],"p":[[3,"RollingHash"]]};
searchIndex["system_test_tool"] = {"doc":"","i":[[5,"system_test","system_test_tool","",null,[[["s"]]]],[8,"Solution","","",null,null],[10,"solve","","",0,[[["str"],["self"]],["string"]]],[10,"problem_url","","",0,[[["self"]],["str"]]]],"p":[[8,"Solution"]]};
searchIndex["union_find"] = {"doc":"","i":[[3,"UnionFind","union_find","Union Find はグラフの連結成分を管理します。",null,null],[11,"new","","グラフの頂点数 `n` を渡します。",0,[[["usize"]],["unionfind"]]],[11,"find","","頂点 `i` の属する連結成分の代表元を返します。",0,[[["self"],["usize"]],["usize"]]],[11,"unite","","頂点 `i` の属する連結成分と頂点 `j` の属する連結成分をつなげます。",0,[[["self"],["usize"]]]],[11,"get_size","","頂点 `i` の属する連結成分のサイズ (頂点数) を返します。",0,[[["self"],["usize"]],["usize"]]],[11,"same","","頂点 `i` と頂点 `j` が同じ連結成分に属するかどうかを返します。",0,[[["self"],["usize"]],["bool"]]],[11,"components","","「連結成分に属する頂点のベクタ」のベクタを返します。",0,[[["self"]],[["vec",["vec"]],["vec",["usize"]]]]],[11,"leaders","","各連結成分の代表元をベクタで返します。`uf.components().iter().map(|c|…",0,[[["self"]],[["usize"],["vec",["usize"]]]]],[11,"from","","",0,[[["t"]],["t"]]],[11,"into","","",0,[[],["u"]]],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"try_into","","",0,[[],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]],[11,"type_id","","",0,[[["self"]],["typeid"]]]],"p":[[3,"UnionFind"]]};
searchIndex["util_macro"] = {"doc":"","i":[],"p":[]};
searchIndex["z_algorithm"] = {"doc":"","i":[[5,"z_algorithm","z_algorithm","",null,[[],[["usize"],["vec",["usize"]]]]]],"p":[]};
addSearchOptions(searchIndex);initSearch(searchIndex);