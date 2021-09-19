var searchIndex={};
searchIndex["arithmetic_series"] = {"doc":"","i":[[5,"arithmetic_series","arithmetic_series","初項 `a`, 項数 `n`, 公差 `d` の等差数列の和を求めます。",null,[[["int"]],[["int"],["option"]]]],[8,"Int","","",null,null],[10,"is_positive","","",0,[[],["bool"]]],[10,"decrement","","",0,[[],["self"]]],[10,"checked_add","","",0,[[],["option"]]],[10,"checked_mul","","",0,[[],["option"]]],[10,"checked_div","","",0,[[],["option"]]],[10,"two","","",0,[[],["self"]]]],"p":[[8,"Int"]]};
searchIndex["binary_search"] = {"doc":"","i":[[8,"BinarySearch","binary_search","ソート済の列に対して二分法で\"境目\"を探します。",null,null],[10,"lower_bound","","",0,[[["t"],["self"]],["usize"]]],[10,"upper_bound","","",0,[[["t"],["self"]],["usize"]]],[10,"split_by","","",0,[[["t"],["self"]]]]],"p":[[8,"BinarySearch"]]};
searchIndex["ceil_log2"] = {"doc":"","i":[[8,"CeilLog2","ceil_log2","log2 の切り上げです。",null,null],[10,"ceil_log2","","2^x >= self となる最小の x を返します。",0,[[],["self"]]]],"p":[[8,"CeilLog2"]]};
searchIndex["detect_cycle"] = {"doc":"","i":[[5,"detect_cycle_undirected","detect_cycle","無向グラフの閉路を求めます。",null,[[["usize"]],[["option",["vec"]],["vec",["usize"]]]]],[5,"detect_cycle_directed","","有向グラフの閉路を求めます。",null,[[["usize"]],[["option",["vec"]],["vec",["usize"]]]]]],"p":[]};
searchIndex["dijkstra"] = {"doc":"","i":[[3,"Edge","dijkstra","",null,null],[12,"to","","行き先の頂点です。",0,null],[12,"cost","","移動にかかるコストです。",0,null],[5,"dijkstra","","`dijkstra` はあるひとつの頂点から全ての頂点への最短距離を計算します。",null,[[["usize"]]]],[11,"new","","",0,[[["usize"],["u64"]],["self"]]],[11,"from","","",0,[[["t"]],["t"]]],[11,"into","","",0,[[],["u"]]],[11,"to_owned","","",0,[[["self"]],["t"]]],[11,"clone_into","","",0,[[["self"],["t"]]]],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"try_into","","",0,[[],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]],[11,"type_id","","",0,[[["self"]],["typeid"]]],[11,"clone","","",0,[[["self"]],["edge"]]],[11,"fmt","","",0,[[["formatter"],["self"]],["result"]]]],"p":[[3,"Edge"]]};
searchIndex["divisors"] = {"doc":"","i":[[8,"Divisors","divisors","非負整数の約数全体です。",null,null],[10,"divisors","","非負整数の約数を昇順で返します。`0` に対しては空のベクタ `vec![]` を返します。",0,[[],["vec"]]]],"p":[[8,"Divisors"]]};
searchIndex["ext_gcd"] = {"doc":"","i":[[5,"ext_gcd","ext_gcd","g = gcd(a, b), ax + by = g を満たす (x, y, g) を返します。",null,[[["i64"]]]]],"p":[]};
searchIndex["factorials"] = {"doc":"","i":[[5,"factorials","factorials","`1` 以上 `size` 未満の `n` について、`n` の階乗 (mod `mo`) と、その乗法逆元を…",null,[[["usize"],["u64"]]]]],"p":[]};
searchIndex["fenwick_tree"] = {"doc":"","i":[[3,"FenwickTree","fenwick_tree","",null,null],[11,"new","","長さ `n` の列を作り、初期値 `e` で埋めます。雰囲気は `let mut a = vec![e; n];`…",0,[[["usize"],["t"]],["self"]]],[11,"add","","列の `k` 番目に `x` を足します。`k` は 0-indexed です。`a[k] += x;`",0,[[["self"],["usize"],["t"]]]],[11,"sum","","区間和を計算します。`range` が `l..r` だとして…",0,[[["range",["usize"]],["self"],["usize"]],["t"]]],[11,"from","","",0,[[["t"]],["t"]]],[11,"into","","",0,[[],["u"]]],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"try_into","","",0,[[],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]],[11,"type_id","","",0,[[["self"]],["typeid"]]]],"p":[[3,"FenwickTree"]]};
searchIndex["floor_sqrt"] = {"doc":"","i":[[5,"floor_sqrt","floor_sqrt","`floor(sqrt(n))` を返す。",null,[[["u64"]],["u64"]]]],"p":[]};
searchIndex["grid_search"] = {"doc":"","i":[[3,"Around","grid_search","This `struct` is created by the [`around`] methods. See…",null,null],[5,"around","","`(y, x)` を基点とした周辺座標を yield するイテレータを作ります。",null,[[["usize"]],["around"]]],[11,"y_range","","上下方向の範囲をセットします。デフォルトは `0..usize::MAX` です。",0,[[],["self"]]],[11,"x_range","","左右方向の範囲をセットします。デフォルトは `0..usize::MAX` です。",0,[[],["self"]]],[11,"directions","","基点からの相対座標たちをセットします。デフォルトは空のスライスです。",0,[[],["self"]]],[11,"from","","",0,[[["t"]],["t"]]],[11,"into","","",0,[[],["u"]]],[11,"into_iter","","",0,[[],["i"]]],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"try_into","","",0,[[],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]],[11,"type_id","","",0,[[["self"]],["typeid"]]],[11,"next","","",0,[[["self"]],["option"]]]],"p":[[3,"Around"]]};
searchIndex["join"] = {"doc":"","i":[[8,"Join","join","",null,null],[10,"join","","",0,[[["str"],["self"]],["string"]]]],"p":[[8,"Join"]]};
searchIndex["least_prime_factors"] = {"doc":"","i":[[5,"least_prime_factors","least_prime_factors","「`k` を割る最小の素数」をエラトステネスのふるいの要領で `2` 以上 `n` 未満の全ての `k`…",null,[[["usize"]],[["usize"],["vec",["usize"]]]]]],"p":[]};
searchIndex["lowest_common_ancestor"] = {"doc":"","i":[[3,"LowestCommonAncestor","lowest_common_ancestor","頂点 `0` を根とする根付き木の LCA を求めます。",null,null],[11,"new","","木を隣接グラフ形式で渡します。",0,[[],["self"]]],[11,"get","","`u` と `v` の LCA を返します。",0,[[["self"],["usize"]],["usize"]]],[11,"get_dist","","`u` と `v` の距離 (頂点間にある辺の数) を返します。",0,[[["self"],["usize"]],["usize"]]],[11,"depth","","",0,[[["self"]],["vec"]]],[11,"ancestor","","",0,[[["self"]],["vec"]]],[11,"from","","",0,[[["t"]],["t"]]],[11,"into","","",0,[[],["u"]]],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"try_into","","",0,[[],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]],[11,"type_id","","",0,[[["self"]],["typeid"]]]],"p":[[3,"LowestCommonAncestor"]]};
searchIndex["mod_int"] = {"doc":"`ModInt` は整数の四則演算を mod `p` で行う構造体です。","i":[[3,"ModInt","mod_int","",null,null],[3,"Mod1000000007","","",null,null],[3,"Mod998244353","","",null,null],[6,"ModInt1000000007","","",null,null],[6,"ModInt998244353","","",null,null],[8,"Modulo","","",null,null],[18,"P","","",0,null],[11,"new","","整数を `0 <= x < p` に正規化してインスタンスを作ります。",1,[[["i64"]],["self"]]],[11,"val","","`ModInt` に格納されている値 `x` を返します。",1,[[],["i64"]]],[11,"p","","法 `p` を返します。",1,[[],["i64"]]],[11,"pow","","二分累乗法で `x^exp % p` を計算します。",1,[[["t"]],["self"]]],[11,"inv","","`x * y % p = 1` となる `y` を返します。",1,[[],["self"]]],[14,"define_mod_int_p","","好きな法の `ModInt` を定義します。",null,null],[11,"from","","",1,[[["t"]],["t"]]],[11,"into","","",1,[[],["u"]]],[11,"to_owned","","",1,[[["self"]],["t"]]],[11,"clone_into","","",1,[[["self"],["t"]]]],[11,"try_from","","",1,[[["u"]],["result"]]],[11,"try_into","","",1,[[],["result"]]],[11,"borrow","","",1,[[["self"]],["t"]]],[11,"borrow_mut","","",1,[[["self"]],["t"]]],[11,"type_id","","",1,[[["self"]],["typeid"]]],[11,"from","","",2,[[["t"]],["t"]]],[11,"into","","",2,[[],["u"]]],[11,"to_owned","","",2,[[["self"]],["t"]]],[11,"clone_into","","",2,[[["self"],["t"]]]],[11,"try_from","","",2,[[["u"]],["result"]]],[11,"try_into","","",2,[[],["result"]]],[11,"borrow","","",2,[[["self"]],["t"]]],[11,"borrow_mut","","",2,[[["self"]],["t"]]],[11,"type_id","","",2,[[["self"]],["typeid"]]],[11,"from","","",3,[[["t"]],["t"]]],[11,"into","","",3,[[],["u"]]],[11,"to_owned","","",3,[[["self"]],["t"]]],[11,"clone_into","","",3,[[["self"],["t"]]]],[11,"try_from","","",3,[[["u"]],["result"]]],[11,"try_into","","",3,[[],["result"]]],[11,"borrow","","",3,[[["self"]],["t"]]],[11,"borrow_mut","","",3,[[["self"]],["t"]]],[11,"type_id","","",3,[[["self"]],["typeid"]]],[11,"from","","",1,[[["i32"]],["self"]]],[11,"from","","",1,[[["i64"]],["self"]]],[11,"from","","",1,[[["u32"]],["self"]]],[11,"from","","",1,[[["u64"]],["self"]]],[11,"from","","",1,[[["usize"]],["self"]]],[11,"clone","","",1,[[["self"]],["modint"]]],[11,"clone","","",2,[[["self"]],["mod1000000007"]]],[11,"clone","","",3,[[["self"]],["mod998244353"]]],[11,"fmt","","",1,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",2,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",3,[[["formatter"],["self"]],["result"]]],[11,"div","","",1,[[["t"]]]],[11,"sub","","",1,[[["t"]]]],[11,"add","","",1,[[["t"]]]],[11,"mul","","",1,[[["t"]]]],[11,"add_assign","","",1,[[["self"],["t"]]]],[11,"sub_assign","","",1,[[["self"],["t"]]]],[11,"mul_assign","","",1,[[["self"],["t"]]]],[11,"div_assign","","",1,[[["self"],["t"]]]]],"p":[[8,"Modulo"],[3,"ModInt"],[3,"Mod1000000007"],[3,"Mod998244353"]]};
searchIndex["next_permutation"] = {"doc":"","i":[[8,"NextPermutation","next_permutation","",null,null],[10,"next_permutation","","",0,[[["self"]],["bool"]]]],"p":[[8,"NextPermutation"]]};
searchIndex["prime_factorization"] = {"doc":"","i":[[8,"PrimeFactorization","prime_factorization","非負整数を素因数分解です。",null,null],[10,"prime_factorization","","(素因数, べき) のベクタを返します。",0,[[],["vec"]]]],"p":[[8,"PrimeFactorization"]]};
searchIndex["procon_reader"] = {"doc":"","i":[[3,"ProconReader","procon_reader","競技プログラミングで、入力値を読むパートをラクにします。",null,null],[11,"new","","標準入力から読み込みたいときの例です。 ファイルからの読み込みは `BufRead` の Examples…",0,[[["r"]],["self"]]],[11,"get","","空白・改行で区切られている値を取得します。適宜、型アノテーションをつけてください。",0,[[["self"]],["t"]]],[11,"get_vec","","空白・改行区切りの値を `n` 個読みます。",0,[[["self"],["usize"]],["vec"]]],[11,"get_chars","","1 行の文字列を `char` のベクタとして読みます。",0,[[["self"]],[["vec",["char"]],["char"]]]],[11,"from","","",0,[[["t"]],["t"]]],[11,"into","","",0,[[],["u"]]],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"try_into","","",0,[[],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]],[11,"type_id","","",0,[[["self"]],["typeid"]]]],"p":[[3,"ProconReader"]]};
searchIndex["rolling_hash"] = {"doc":"","i":[[3,"RollingHash","rolling_hash","Rolling Hash です。O(文字列長) の前計算をしたうえで、部分文字列のハッシュ値を O(1) で計算します。",null,null],[11,"new","","",0,[[],["self"]]],[11,"get","","`range` が指す範囲の部分文字列のハッシュ値を返します。",0,[[["usize"],["self"],["range",["usize"]]],["u64"]]],[11,"connect","","2 つの文字列を連結したときのハッシュ値を返します。",0,[[["usize"],["self"],["u64"]],["u64"]]],[11,"from","","",0,[[["t"]],["t"]]],[11,"into","","",0,[[],["u"]]],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"try_into","","",0,[[],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]],[11,"type_id","","",0,[[["self"]],["typeid"]]]],"p":[[3,"RollingHash"]]};
searchIndex["suffix_array"] = {"doc":"","i":[[5,"suffix_array","suffix_array","文字列 `s` の suffix array を O(|s|log|s|) で求めます。",null,[[],[["usize"],["vec",["usize"]]]]],[5,"lcp_array","","LCP 配列を O(|s|) で求めます。",null,[[],[["usize"],["vec",["usize"]]]]]],"p":[]};
searchIndex["topological_sort"] = {"doc":"","i":[[5,"topological_sort","topological_sort","有向グラフの頂点をトポロジカル順に並べて返します。グラフが DAG でなければ None を返します。",null,[[["usize"]],[["option",["vec"]],["vec",["usize"]]]]]],"p":[]};
searchIndex["union_find"] = {"doc":"","i":[[3,"UnionFind","union_find","Union Find はグラフの連結成分を管理します。",null,null],[11,"new","","グラフの頂点数 `n` を渡します。",0,[[["usize"]],["unionfind"]]],[11,"find","","頂点 `i` の属する連結成分の代表元を返します。",0,[[["self"],["usize"]],["usize"]]],[11,"unite","","頂点 `i` の属する連結成分と頂点 `j` の属する連結成分をつなげます。",0,[[["self"],["usize"]]]],[11,"get_size","","頂点 `i` の属する連結成分のサイズ (頂点数) を返します。",0,[[["self"],["usize"]],["usize"]]],[11,"same","","頂点 `i` と頂点 `j` が同じ連結成分に属するかどうかを返します。",0,[[["self"],["usize"]],["bool"]]],[11,"from","","",0,[[["t"]],["t"]]],[11,"into","","",0,[[],["u"]]],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"try_into","","",0,[[],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]],[11,"type_id","","",0,[[["self"]],["typeid"]]]],"p":[[3,"UnionFind"]]};
searchIndex["z_algorithm"] = {"doc":"","i":[[5,"z_algorithm","z_algorithm","`z[i]`: `a[i..]` と `a` との最長共通接頭辞の長さ、を返します。",null,[[],[["usize"],["vec",["usize"]]]]]],"p":[]};
addSearchOptions(searchIndex);initSearch(searchIndex);