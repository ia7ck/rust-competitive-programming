var searchIndex={};
searchIndex["binary_search"] = {"doc":"","i":[[8,"BinarySearch","binary_search","ソート済の列に対して二分法で\"境目\"を探します。",null,null],[10,"lower_bound","","",0,[[["t"],["self"]],["usize"]]],[10,"upper_bound","","",0,[[["t"],["self"]],["usize"]]],[10,"split_by","","",0,[[["t"],["self"]]]]],"p":[[8,"BinarySearch"]]};
searchIndex["combination"] = {"doc":"","i":[[3,"Binom","combination","",null,null],[3,"BinomWithModInt","","",null,null],[8,"BinomialCoefficient","","",null,null],[16,"Output","","",0,null],[10,"get","","二項係数「`n` 個の物から `k` 個を選ぶ通り数」を返します。",0,[[["self"],["usize"]]]],[11,"new","","`0` 以上 `size` 未満の `n` について",1,[[["i64"],["usize"]],["self"]]],[11,"new","","`0` 以上 `size` 未満の `n` について",2,[[["usize"]],["self"]]],[11,"from","","",1,[[["t"]],["t"]]],[11,"into","","",1,[[],["u"]]],[11,"try_from","","",1,[[["u"]],["result"]]],[11,"try_into","","",1,[[],["result"]]],[11,"borrow","","",1,[[["self"]],["t"]]],[11,"borrow_mut","","",1,[[["self"]],["t"]]],[11,"type_id","","",1,[[["self"]],["typeid"]]],[11,"from","","",2,[[["t"]],["t"]]],[11,"into","","",2,[[],["u"]]],[11,"try_from","","",2,[[["u"]],["result"]]],[11,"try_into","","",2,[[],["result"]]],[11,"borrow","","",2,[[["self"]],["t"]]],[11,"borrow_mut","","",2,[[["self"]],["t"]]],[11,"type_id","","",2,[[["self"]],["typeid"]]],[11,"get","","二項係数 `% mo` を定数時間で計算します。法 `mo` は構築時に与えたパラメータです。",1,[[["self"],["usize"]]]],[11,"get","","二項係数を `ModInt` で wrap して返します。割り算をするので計算量は `ModInt` の法 `p`…",2,[[["self"],["usize"]]]]],"p":[[8,"BinomialCoefficient"],[3,"Binom"],[3,"BinomWithModInt"]]};
searchIndex["dijkstra"] = {"doc":"","i":[[3,"Edge","dijkstra","",null,null],[12,"to","","行き先の頂点です。",0,null],[12,"cost","","移動にかかるコストです。",0,null],[5,"dijkstra","","`dijkstra` はあるひとつの頂点から全ての頂点への最短距離を計算します。",null,[[["usize"]]]],[11,"from","","",0,[[["t"]],["t"]]],[11,"into","","",0,[[],["u"]]],[11,"to_owned","","",0,[[["self"]],["t"]]],[11,"clone_into","","",0,[[["self"],["t"]]]],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"try_into","","",0,[[],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]],[11,"type_id","","",0,[[["self"]],["typeid"]]],[11,"clone","","",0,[[["self"]],["edge"]]],[11,"fmt","","",0,[[["formatter"],["self"]],["result"]]]],"p":[[3,"Edge"]]};
searchIndex["fenwick_tree"] = {"doc":"","i":[[3,"FenwickTree","fenwick_tree","",null,null],[11,"new","","長さ `n` の列を作り、初期値 `e` で埋めます。雰囲気は `let mut a = vec![e; n];`…",0,[[["usize"],["t"]],["self"]]],[11,"add","","`k` 番目の値に `x` を足します。`k` は 0-indexed です。`a[k] += x;`",0,[[["self"],["usize"],["t"]]]],[11,"sum","","区間和を計算します。`range` が `l..r` だとして `a[l..r].iter().sum();`…",0,[[["usize"],["self"],["range",["usize"]]],["t"]]],[11,"from","","",0,[[["t"]],["t"]]],[11,"into","","",0,[[],["u"]]],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"try_into","","",0,[[],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]],[11,"type_id","","",0,[[["self"]],["typeid"]]],[11,"vzip","","",0,[[],["v"]]]],"p":[[3,"FenwickTree"]]};
searchIndex["grid_search"] = {"doc":"","i":[[3,"Around","grid_search","This `struct` is created by the [`around`] methods. See…",null,null],[5,"around","","`(y, x)` を基点とした周辺座標を yield するイテレータを作ります。",null,[[["usize"]],["around"]]],[11,"y_range","","上下方向の範囲をセットします。デフォルトは `0..usize::MAX` です。",0,[[],["self"]]],[11,"x_range","","左右方向の範囲をセットします。デフォルトは `0..usize::MAX` です。",0,[[],["self"]]],[11,"directions","","基点からの相対座標たちをセットします。デフォルトは空のスライスです。",0,[[],["self"]]],[11,"from","","",0,[[["t"]],["t"]]],[11,"into","","",0,[[],["u"]]],[11,"into_iter","","",0,[[],["i"]]],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"try_into","","",0,[[],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]],[11,"type_id","","",0,[[["self"]],["typeid"]]],[11,"next","","",0,[[["self"]],["option"]]]],"p":[[3,"Around"]]};
searchIndex["min_factors"] = {"doc":"","i":[[5,"min_factors","min_factors","「`k` を割る最小の素数」をエラトステネスのふるいの要領で `2` 以上 `n` 未満の全ての `k`…",null,[[["usize"]],[["usize"],["vec",["usize"]]]]]],"p":[]};
searchIndex["mod_int"] = {"doc":"`ModInt` は「～を `p` で割った余りを出力してください」形式の問題で活躍します。","i":[[3,"ModInt","mod_int","",null,null],[3,"Mod1000000007","","",null,null],[3,"Mod998244353","","",null,null],[6,"ModInt1000000007","","",null,null],[6,"ModInt998244353","","",null,null],[8,"Modulo","","",null,null],[10,"p","","",0,[[],["i64"]]],[11,"new","","`0 <= x < p` に正規化してインスタンスを作ります。",1,[[["t"]],["self"]]],[11,"new_raw","","定数倍高速化用です。`0 <= x < p` であることを信用してインスタンスを作ります。",1,[[["i64"]],["self"]]],[11,"val","","`ModInt` に格納されている値 `x` を返します。",1,[[],["i64"]]],[11,"mo","","法 `p` を返します。",1,[[],["i64"]]],[11,"pow","","二分累乗法で `x^exp % p` を計算します。",1,[[["u64"]],["self"]]],[11,"inv","","`x * y % p = 1` となる `y` を返します。",1,[[],["self"]]],[11,"new_frac","","分数 `numer/denom % p` です。",1,[[["i64"]],["self"]]],[14,"define_mod_int_p","","好きな法の `ModInt` を定義します。",null,null],[11,"from","","",1,[[["t"]],["t"]]],[11,"into","","",1,[[],["u"]]],[11,"to_owned","","",1,[[["self"]],["t"]]],[11,"clone_into","","",1,[[["self"],["t"]]]],[11,"try_from","","",1,[[["u"]],["result"]]],[11,"try_into","","",1,[[],["result"]]],[11,"borrow","","",1,[[["self"]],["t"]]],[11,"borrow_mut","","",1,[[["self"]],["t"]]],[11,"type_id","","",1,[[["self"]],["typeid"]]],[11,"from","","",2,[[["t"]],["t"]]],[11,"into","","",2,[[],["u"]]],[11,"to_owned","","",2,[[["self"]],["t"]]],[11,"clone_into","","",2,[[["self"],["t"]]]],[11,"try_from","","",2,[[["u"]],["result"]]],[11,"try_into","","",2,[[],["result"]]],[11,"borrow","","",2,[[["self"]],["t"]]],[11,"borrow_mut","","",2,[[["self"]],["t"]]],[11,"type_id","","",2,[[["self"]],["typeid"]]],[11,"from","","",3,[[["t"]],["t"]]],[11,"into","","",3,[[],["u"]]],[11,"to_owned","","",3,[[["self"]],["t"]]],[11,"clone_into","","",3,[[["self"],["t"]]]],[11,"try_from","","",3,[[["u"]],["result"]]],[11,"try_into","","",3,[[],["result"]]],[11,"borrow","","",3,[[["self"]],["t"]]],[11,"borrow_mut","","",3,[[["self"]],["t"]]],[11,"type_id","","",3,[[["self"]],["typeid"]]],[11,"p","","",2,[[],["i64"]]],[11,"p","","",3,[[],["i64"]]],[11,"clone","","",1,[[["self"]],["modint"]]],[11,"clone","","",2,[[["self"]],["mod1000000007"]]],[11,"clone","","",3,[[["self"]],["mod998244353"]]],[11,"fmt","","",1,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",2,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",3,[[["formatter"],["self"]],["result"]]],[11,"div","","",1,[[["modint"]]]],[11,"sub","","",1,[[["modint"]]]],[11,"add","","",1,[[["modint"]]]],[11,"mul","","",1,[[["modint"]]]]],"p":[[8,"Modulo"],[3,"ModInt"],[3,"Mod1000000007"],[3,"Mod998244353"]]};
searchIndex["next_permutation"] = {"doc":"","i":[[8,"NextPermutation","next_permutation","",null,null],[10,"next_permutation","","",0,[[["self"]],["bool"]]]],"p":[[8,"NextPermutation"]]};
searchIndex["procon_reader"] = {"doc":"","i":[[3,"ProconReader","procon_reader","競技プログラミングで、入力値を読むパートをラクにします。",null,null],[11,"new","","標準入力から読み込みたいときの例です。 ファイルからの読み込みは `BufRead` の Examples…",0,[[["r"]],["self"]]],[11,"get","","空白・改行で区切られている値を取得します。適宜、型アノテーションをつけてください。",0,[[["self"]],["t"]]],[11,"get_vec","","空白・改行区切りの値を `n` 個読みます。",0,[[["self"],["usize"]],["vec"]]],[11,"get_chars","","1 行の文字列を `char` のベクタとして読みます。",0,[[["self"]],[["vec",["char"]],["char"]]]],[11,"from","","",0,[[["t"]],["t"]]],[11,"into","","",0,[[],["u"]]],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"try_into","","",0,[[],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]],[11,"type_id","","",0,[[["self"]],["typeid"]]]],"p":[[3,"ProconReader"]]};
searchIndex["rolling_hash"] = {"doc":"","i":[[3,"RollingHash","rolling_hash","Rolling Hash です。O(文字列長) の前計算をしたうえで、部分文字列のハッシュ値を O(1) で計算します。",null,null],[11,"new","","",0,[[],["self"]]],[11,"get","","`range` が指す範囲の部分文字列のハッシュ値を返します。",0,[[["usize"],["self"],["range",["usize"]]],["u64"]]],[11,"connect","","2 つの文字列を連結したときのハッシュ値を返します。",0,[[["usize"],["self"],["u64"]],["u64"]]],[11,"from","","",0,[[["t"]],["t"]]],[11,"into","","",0,[[],["u"]]],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"try_into","","",0,[[],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]],[11,"type_id","","",0,[[["self"]],["typeid"]]],[11,"vzip","","",0,[[],["v"]]]],"p":[[3,"RollingHash"]]};
searchIndex["system_test_tool"] = {"doc":"","i":[[5,"system_test","system_test_tool","",null,[[["str"],["f"]]]]],"p":[]};
searchIndex["union_find"] = {"doc":"","i":[[3,"UnionFind","union_find","Union Find はグラフの連結成分を管理します。",null,null],[11,"new","","グラフの頂点数 `n` を渡します。",0,[[["usize"]],["unionfind"]]],[11,"find","","頂点 `i` の属する連結成分の代表元を返します。",0,[[["self"],["usize"]],["usize"]]],[11,"unite","","頂点 `i` の属する連結成分と頂点 `j` の属する連結成分をつなげます。",0,[[["self"],["usize"]]]],[11,"get_size","","頂点 `i` の属する連結成分のサイズ (頂点数) を返します。",0,[[["self"],["usize"]],["usize"]]],[11,"same","","頂点 `i` と頂点 `j` が同じ連結成分に属するかどうかを返します。",0,[[["self"],["usize"]],["bool"]]],[11,"from","","",0,[[["t"]],["t"]]],[11,"into","","",0,[[],["u"]]],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"try_into","","",0,[[],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]],[11,"type_id","","",0,[[["self"]],["typeid"]]]],"p":[[3,"UnionFind"]]};
searchIndex["z_algorithm"] = {"doc":"","i":[[5,"z_algorithm","z_algorithm","`z[i]`: `a[i..]` と `a` との最長共通接頭辞の長さ、を返します。",null,[[],[["usize"],["vec",["usize"]]]]]],"p":[]};
addSearchOptions(searchIndex);initSearch(searchIndex);