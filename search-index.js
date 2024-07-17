var searchIndex = JSON.parse('{\
"arithmetic_series":{"doc":"","t":"IFKKKKKKK","n":["Int","arithmetic_series","checked_add","checked_div","checked_mul","decrement","is_positive","two","zero"],"q":[[0,"arithmetic_series"]],"d":["","初項 <code>a</code>, 項数 <code>n</code>, 公差 <code>d</code> …","","","","","","",""],"i":[0,0,1,1,1,1,1,1,1],"f":[0,[[1,1,1],[[2,[1]]]],[[],2],[[],2],[[],2],[[]],[[],3],[[]],[[]]],"c":[],"p":[[8,"Int"],[4,"Option"],[15,"bool"]]},\
"binary_search_range":{"doc":"","t":"IK","n":["BinarySearchRange","range"],"q":[[0,"binary_search_range"]],"d":["ソート済みの列を検索します。",""],"i":[0,3],"f":[0,[1,[[1,[2]]]]],"c":[],"p":[[3,"Range"],[15,"usize"],[8,"BinarySearchRange"]]},\
"ceil_log2":{"doc":"","t":"IK","n":["CeilLog2","ceil_log2"],"q":[[0,"ceil_log2"]],"d":["log2 の切り上げです。","2^x &gt;= self となる最小の x を返します。"],"i":[0,1],"f":[0,[[]]],"c":[],"p":[[8,"CeilLog2"]]},\
"cumulative_sum_2d":{"doc":"","t":"DLLLLLLLLL","n":["CumulativeSum2D","borrow","borrow_mut","from","into","new","sum","try_from","try_into","type_id"],"q":[[0,"cumulative_sum_2d"]],"d":["二次元累積和です。","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","",""],"i":[0,6,6,6,6,6,6,6,6,6],"f":[0,[[]],[[]],[[]],[[]],[[],[[6,[[0,[1,2,3,4,5]]]]]],[[[6,[[0,[1,2,3,4,5]]]],[8,[7]],[8,[7]]],[[0,[1,2,3,4,5]]]],[[],9],[[],9],[[],10]],"c":[],"p":[[8,"Clone"],[8,"Copy"],[8,"Default"],[8,"Add"],[8,"Sub"],[3,"CumulativeSum2D"],[15,"usize"],[3,"Range"],[4,"Result"],[3,"TypeId"]]},\
"detect_cycle":{"doc":"","t":"FF","n":["detect_cycle_directed","detect_cycle_undirected"],"q":[[0,"detect_cycle"]],"d":["有向グラフの閉路を求めます。","無向グラフの閉路を求めます。"],"i":[0,0],"f":[[1,[[3,[[2,[1]]]]]],[1,[[3,[[2,[1]]]]]]],"c":[],"p":[[15,"usize"],[3,"Vec"],[4,"Option"]]},\
"dijkstra":{"doc":"","t":"DILLLLFKLKLLLLKLLLLL","n":["ConstEdge","Edge","borrow","borrow_mut","clone","clone_into","dijkstra","dist","dist","from","from","from","into","new","to","to","to_owned","try_from","try_into","type_id"],"q":[[0,"dijkstra"]],"d":["長さが定数の辺です。","グラフの辺を表すトレイトです。","","","","","<code>dijkstra</code> …","始点から <code>from</code> までの距離 <code>d</code> …","","","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","","","","","","",""],"i":[0,0,2,2,2,2,0,8,2,8,2,2,2,2,8,2,2,2,2,2],"f":[0,0,[[]],[[]],[[[2,[1]]],[[2,[1]]]],[[]],[[3,3]],[[]],[[[2,[[0,[4,5]]]],[0,[4,5]]],[[0,[4,5]]]],[[],3],[[]],[[[2,[[0,[4,5]]]]],3],[[]],[[3,3],2],[[],3],[[[2,[[0,[4,5]]]]],3],[[]],[[],6],[[],6],[[],7]],"c":[],"p":[[8,"Clone"],[3,"ConstEdge"],[15,"usize"],[8,"Copy"],[8,"Add"],[4,"Result"],[3,"TypeId"],[8,"Edge"]]},\
"divisors":{"doc":"","t":"IK","n":["Divisors","divisors"],"q":[[0,"divisors"]],"d":["非負整数の約数全体です。","非負整数の約数を昇順で返します。<code>0</code> …"],"i":[0,2],"f":[0,[[],1]],"c":[],"p":[[3,"Vec"],[8,"Divisors"]]},\
"ext_gcd":{"doc":"","t":"F","n":["ext_gcd"],"q":[[0,"ext_gcd"]],"d":["g = gcd(a, b), ax + by = g を満たす (x, y, g) …"],"i":[0],"f":[[[1,1]]],"c":[],"p":[[15,"i64"]]},\
"factorials":{"doc":"","t":"DLLLLLLLLLLLLL","n":["Factorial","binomial","binomial_or_zero","borrow","borrow_mut","factorial","from","into","inversion","new","new_checking_modulo_prime","try_from","try_into","type_id"],"q":[[0,"factorials"]],"d":["…","二項係数を返します。","<code>binomial</code> とほとんど同じですが <code>n</code> が <code>k</code> …","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","<code>1</code> 以上 <code>size</code> 未満の <code>n</code> について、<code>n</code> の階乗 (mod …","<code>modulo</code> …","","",""],"i":[0,1,1,1,1,1,1,1,1,1,1,1,1,1],"f":[0,[[1,2,2],3],[[1,2,2],3],[[]],[[]],[[1,2],3],[[]],[[]],[[1,2],3],[[2,3],1],[[2,3],1],[[],4],[[],4],[[],5]],"c":[],"p":[[3,"Factorial"],[15,"usize"],[15,"u64"],[4,"Result"],[3,"TypeId"]]},\
"fenwick_tree":{"doc":"","t":"DLLLLLLLLLLLLLL","n":["FenwickTree","add","borrow","borrow_mut","clone","clone_into","fmt","from","into","new","sum","to_owned","try_from","try_into","type_id"],"q":[[0,"fenwick_tree"]],"d":["Fenwick Tree (Binary Indexed Tree) …","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","",""],"i":[0,4,4,4,4,4,4,4,4,4,4,4,4,4,4],"f":[0,[[[4,[[0,[1,2,3]]]],5,[0,[1,2,3]]]],[[]],[[]],[[[4,[6]]],[[4,[6]]]],[[]],[[[4,[7]],8],9],[[]],[[]],[[5,[0,[1,2,3]]],[[4,[[0,[1,2,3]]]]]],[[[4,[[0,[1,2,3]]]],[10,[5]]],[[0,[1,2,3]]]],[[]],[[],11],[[],11],[[],12]],"c":[],"p":[[8,"Copy"],[8,"AddAssign"],[8,"SubAssign"],[3,"FenwickTree"],[15,"usize"],[8,"Clone"],[8,"Debug"],[3,"Formatter"],[6,"Result"],[3,"Range"],[4,"Result"],[3,"TypeId"]]},\
"floor_sqrt":{"doc":"","t":"F","n":["floor_sqrt"],"q":[[0,"floor_sqrt"]],"d":["<code>floor(sqrt(n))</code> を返す。"],"i":[0],"f":[[1,1]],"c":[],"p":[[15,"u64"]]},\
"graph":{"doc":"","t":"FF","n":["connectivity","is_tree"],"q":[[0,"graph"]],"d":["",""],"i":[0,0],"f":[[1,2],[1,2]],"c":[],"p":[[15,"usize"],[15,"bool"]]},\
"grid_search":{"doc":"","t":"DFLLLLLLLLLLLL","n":["Around","around","borrow","borrow_mut","directions","from","into","into_iter","next","try_from","try_into","type_id","x_range","y_range"],"q":[[0,"grid_search"]],"d":["This <code>struct</code> is created by the <code>around</code> methods. See its …","<code>(y, x)</code> を基点とした周辺座標を yield …","","","…","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","…","…"],"i":[0,0,2,2,2,2,2,2,2,2,2,2,2,2],"f":[0,[[1,1],2],[[]],[[]],[2,2],[[]],[[]],[[]],[2,3],[[],4],[[],4],[[],5],[[2,[6,[1]]],2],[[2,[6,[1]]],2]],"c":[],"p":[[15,"usize"],[3,"Around"],[4,"Option"],[4,"Result"],[3,"TypeId"],[8,"RangeBounds"]]},\
"join":{"doc":"","t":"IK","n":["Join","join"],"q":[[0,"join"]],"d":["",""],"i":[0,3],"f":[0,[1,2]],"c":[],"p":[[15,"str"],[3,"String"],[8,"Join"]]},\
"least_prime_factors":{"doc":"","t":"F","n":["least_prime_factors"],"q":[[0,"least_prime_factors"]],"d":["「<code>k</code> …"],"i":[0],"f":[[1,[[2,[1]]]]],"c":[],"p":[[15,"usize"],[3,"Vec"]]},\
"lowest_common_ancestor":{"doc":"","t":"DLLLLLLLLLLLL","n":["LowestCommonAncestor","borrow","borrow_mut","depth","from","get","get_dist","into","kth_parent","new","try_from","try_into","type_id"],"q":[[0,"lowest_common_ancestor"]],"d":["根付き木の LCA です。","","","頂点 <code>u</code> の深さを返します。","Returns the argument unchanged.","<code>u</code> と <code>v</code> の LCA を返します。","<code>u</code> と <code>v</code> の距離 (頂点間にある辺の数) …","Calls <code>U::from(self)</code>.","頂点 <code>u</code> から根の方向に <code>k</code> …","頂点数 <code>n</code>, 根 <code>root</code>, 木をなす無向辺の集合 <code>edges</code>…","","",""],"i":[0,1,1,1,1,1,1,1,1,1,1,1,1],"f":[0,[[]],[[]],[[1,2],2],[[]],[[1,2,2],2],[[1,2,2],2],[[]],[[1,2,2],[[3,[2]]]],[[2,2],1],[[],4],[[],4],[[],5]],"c":[],"p":[[3,"LowestCommonAncestor"],[15,"usize"],[4,"Option"],[4,"Result"],[3,"TypeId"]]},\
"mod_int":{"doc":"<code>ModInt</code> は整数の四則演算を mod <code>p</code> …","t":"DDGGIDDLLLLLLLLLLLLLLLLLLOLLLLLLLLLLLLLLLLLLLLLLLLLKLLLLLLLLLLLLLLLLLLLLLLLLLLLL","n":["DynamicModulo","ModInt","ModInt1000000007","ModInt998244353","Modulo","Modulo1000000007","Modulo998244353","add","add_assign","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","define_modulo","div","div_assign","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","from","from","from","from","from","from","from","into","into","into","into","inv","modulo","modulo","modulo","modulo","modulo","mul","mul_assign","new","pow","set","sub","sub_assign","to_owned","to_owned","to_owned","to_owned","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","val"],"q":[[0,"mod_int"]],"d":["","","","","","","","","","","","","","","","","","","","","","","","","","好きな法の <code>Modulo</code> を定義します。","","","","","","","","","","","","","Returns the argument unchanged.","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","<code>x * y % p = 1</code> となる <code>y</code> を返します。","","法を返します。","","","","","","整数を <code>0 &lt;= x &lt; modulo</code> …","二分累乗法で <code>x^exp % p</code> を計算します。","","","","","","","","","","","","","","","","","","","","<code>ModInt</code> に格納されている値を返します。"],"i":[0,0,0,0,0,0,0,2,2,2,5,6,7,2,5,6,7,2,5,6,7,2,5,6,7,0,2,2,2,5,6,7,2,2,2,2,2,2,2,2,2,2,2,5,6,7,2,5,6,7,2,1,2,5,6,7,2,2,2,2,7,2,2,2,5,6,7,2,5,6,7,2,5,6,7,2,5,6,7,2],"f":[0,0,0,0,0,0,0,[[[2,[1]],[3,[[2,[1]]]]]],[[[2,[1]],[3,[[2,[1]]]]]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[[2,[4]]],[[2,[4]]]],[5,5],[6,6],[7,7],[[]],[[]],[[]],[[]],0,[[[2,[1]],[3,[[2,[1]]]]]],[[[2,[1]],[3,[[2,[1]]]]]],[[[2,[8]],9],10],[[5,9],10],[[6,9],10],[[7,9],10],[11,[[2,[1]]]],[12,[[2,[1]]]],[13,[[2,[1]]]],[14,[[2,[1]]]],[15,[[2,[1]]]],[16,[[2,[1]]]],[[]],[17,[[2,[1]]]],[18,[[2,[1]]]],[19,[[2,[1]]]],[20,[[2,[1]]]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[[2,[1]]],[[2,[1]]]],[[],11],[[],11],[[],11],[[],11],[[],11],[[[2,[1]],[3,[[2,[1]]]]]],[[[2,[1]],[3,[[2,[1]]]]]],[11,[[2,[1]]]],[[[2,[1]],15],[[2,[1]]]],[11],[[[2,[1]],[3,[[2,[1]]]]]],[[[2,[1]],[3,[[2,[1]]]]]],[[]],[[]],[[]],[[]],[[],21],[[],21],[[],21],[[],21],[[],21],[[],21],[[],21],[[],21],[[],22],[[],22],[[],22],[[],22],[[[2,[1]]],11]],"c":[],"p":[[8,"Modulo"],[3,"ModInt"],[8,"Into"],[8,"Clone"],[3,"Modulo1000000007"],[3,"Modulo998244353"],[3,"DynamicModulo"],[8,"Debug"],[3,"Formatter"],[6,"Result"],[15,"i64"],[15,"isize"],[15,"i8"],[15,"usize"],[15,"u32"],[15,"i32"],[15,"i16"],[15,"u8"],[15,"u64"],[15,"u16"],[4,"Result"],[3,"TypeId"]]},\
"next_permutation":{"doc":"","t":"IK","n":["NextPermutation","next_permutation"],"q":[[0,"next_permutation"]],"d":["next permutation です。",""],"i":[0,2],"f":[0,[[],1]],"c":[],"p":[[15,"bool"],[8,"NextPermutation"]]},\
"oj_test":{"doc":"","t":"DLLFLLLLLLLLLLL","n":["ProblemSolver","borrow","borrow_mut","download_online_judge_testcase","fmt","from","into","new","problem_url","run","solver_path","to_string","try_from","try_into","type_id"],"q":[[0,"oj_test"]],"d":["","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","","",""],"i":[0,5,5,0,5,5,5,5,5,5,5,5,5,5,5],"f":[0,[[]],[[]],[[1,2],[[4,[3]]]],[[5,6],7],[[]],[[]],[2,5],[5,[[8,[1]]]],[[5,2],4],[5,2],[[],9],[[],10],[[],10],[[],11]],"c":[],"p":[[15,"str"],[3,"Path"],[3,"PathBuf"],[6,"Result"],[3,"ProblemSolver"],[3,"Formatter"],[6,"Result"],[4,"Option"],[3,"String"],[4,"Result"],[3,"TypeId"]]},\
"pascal_triangle":{"doc":"","t":"F","n":["pascal_triangle"],"q":[[0,"pascal_triangle"]],"d":["0 以上 <code>n</code> 未満の全ての <code>i</code>, <code>j</code> …"],"i":[0],"f":[[[1,2],[[3,[[3,[2]]]]]]],"c":[],"p":[[15,"usize"],[15,"u64"],[3,"Vec"]]},\
"prime_factorization":{"doc":"","t":"IK","n":["PrimeFactorization","prime_factorization"],"q":[[0,"prime_factorization"]],"d":["非負整数を素因数分解です。","(素因数, べき) のベクタを返します。"],"i":[0,2],"f":[0,[[],1]],"c":[],"p":[[3,"Vec"],[8,"PrimeFactorization"]]},\
"rolling_hash":{"doc":"","t":"DLLLLLLLLLLLLLLLLLL","n":["RollingHash","at","borrow","borrow_mut","clone","clone_into","fmt","from","from_iter","hash","into","is_empty","is_substring","len","new","to_owned","try_from","try_into","type_id"],"q":[[0,"rolling_hash"]],"d":["Rolling Hash です。O(文字列長) …","","","","","","","Returns the argument unchanged.","","部分文字列のハッシュ値を返します。","Calls <code>U::from(self)</code>.","","self が other …","","","","","",""],"i":[0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],"f":[0,[[1,2],3],[[]],[[]],[1,1],[[]],[[1,4],5],[[]],[6,1],[[1,[7,[2]]],3],[[]],[1,8],[[1,1],8],[1,2],[[],1],[[]],[[],9],[[],9],[[],10]],"c":[],"p":[[3,"RollingHash"],[15,"usize"],[15,"u64"],[3,"Formatter"],[6,"Result"],[8,"IntoIterator"],[3,"Range"],[15,"bool"],[4,"Result"],[3,"TypeId"]]},\
"run_length":{"doc":"","t":"DLLLLLLLLLL","n":["RunLength","borrow","borrow_mut","from","into","into_iter","new","next","try_from","try_into","type_id"],"q":[[0,"run_length"]],"d":["run length encoding です。","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","",""],"i":[0,2,2,2,2,2,2,2,2,2,2],"f":[0,[[]],[[]],[[]],[[]],[[]],[1,[[2,[1]]]],[[[2,[1]]],3],[[],4],[[],4],[[],5]],"c":[],"p":[[8,"Iterator"],[3,"RunLength"],[4,"Option"],[4,"Result"],[3,"TypeId"]]},\
"segment_tree":{"doc":"","t":"DLLLLLLLLLLLLLLL","n":["SegmentTree","borrow","borrow_mut","clone","clone_into","fmt","fold","from","get","into","new","to_owned","try_from","try_into","type_id","update"],"q":[[0,"segment_tree"]],"d":["<strong>注意⚠</strong> この実装は遅いので time limit …","","","","","","<code>range</code> が <code>l..r</code> として、…","Returns the argument unchanged.","列の <code>i</code> 番目の要素を取得します。","Calls <code>U::from(self)</code>.","長さ <code>n</code> の列を初期値 <code>e</code> で初期化します。","","","","","列の <code>i</code> 番目の要素を <code>x</code> で更新します。"],"i":[0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2],"f":[0,[[]],[[]],[[[2,[1,1]]],[[2,[1,1]]]],[[]],[[[2,[3,3]],4],5],[[[2,[1,6]],[8,[7]]],1],[[]],[[[2,[1,6]],7],1],[[]],[[7,1,6],[[2,[1,6]]]],[[]],[[],9],[[],9],[[],10],[[[2,[1,6]],7,1]]],"c":[],"p":[[8,"Clone"],[3,"SegmentTree"],[8,"Debug"],[3,"Formatter"],[6,"Result"],[8,"Fn"],[15,"usize"],[3,"Range"],[4,"Result"],[3,"TypeId"]]},\
"sliding_window":{"doc":"","t":"FF","n":["sliding_window_maximum","sliding_window_minimum"],"q":[[0,"sliding_window"]],"d":["<code>sliding_window_minimum</code> の最大値バージョンです。","幅 <code>window_width</code> …"],"i":[0,0],"f":[[1,[[4,[[0,[2,3]]]]]],[1,[[4,[[0,[2,3]]]]]]],"c":[],"p":[[15,"usize"],[8,"Ord"],[8,"Clone"],[3,"Vec"]]},\
"strongly_connected_components":{"doc":"","t":"F","n":["strongly_connected_components"],"q":[[0,"strongly_connected_components"]],"d":["強連結成分分解です。参考"],"i":[0],"f":[[1,[[2,[[2,[1]]]]]]],"c":[],"p":[[15,"usize"],[3,"Vec"]]},\
"suffix_array":{"doc":"","t":"FF","n":["lcp_array","suffix_array"],"q":[[0,"suffix_array"]],"d":["LCP 配列を O(|s|) で求めます。","文字列 <code>s</code> の suffix array を O(|s|log|s|) …"],"i":[0,0],"f":[[[],[[2,[1]]]],[[],[[2,[1]]]]],"c":[],"p":[[15,"usize"],[3,"Vec"]]},\
"topological_sort":{"doc":"","t":"F","n":["topological_sort"],"q":[[0,"topological_sort"]],"d":["…"],"i":[0],"f":[[1,[[3,[[2,[1]]]]]]],"c":[],"p":[[15,"usize"],[3,"Vec"],[4,"Option"]]},\
"tree_diameter":{"doc":"","t":"F","n":["tree_diameter"],"q":[[0,"tree_diameter"]],"d":["…"],"i":[0],"f":[[1]],"c":[],"p":[[15,"usize"]]},\
"union_find":{"doc":"","t":"DLLLLLLLLLLLL","n":["UnionFind","borrow","borrow_mut","find","from","get_size","into","new","same","try_from","try_into","type_id","unite"],"q":[[0,"union_find"]],"d":["Union Find はグラフの連結成分を管理します。","","","頂点 <code>i</code> …","Returns the argument unchanged.","頂点 <code>i</code> の属する連結成分のサイズ (頂点数) …","Calls <code>U::from(self)</code>.","グラフの頂点数 <code>n</code> を渡します。","頂点 <code>i</code> と頂点 <code>j</code> …","","","","頂点 <code>i</code> の属する連結成分と頂点 <code>j</code> …"],"i":[0,1,1,1,1,1,1,1,1,1,1,1,1],"f":[0,[[]],[[]],[[1,2],2],[[]],[[1,2],2],[[]],[2,1],[[1,2,2],3],[[],4],[[],4],[[],5],[[1,2,2]]],"c":[],"p":[[3,"UnionFind"],[15,"usize"],[15,"bool"],[4,"Result"],[3,"TypeId"]]},\
"z_algorithm":{"doc":"","t":"F","n":["z_algorithm"],"q":[[0,"z_algorithm"]],"d":["<code>z[i]</code>: <code>a[i..]</code> と <code>a</code> …"],"i":[0],"f":[[[],[[2,[1]]]]],"c":[],"p":[[15,"usize"],[3,"Vec"]]},\
"zarts":{"doc":"","t":"DLLLLLLLLLLLL","n":["SortedSeq","at","borrow","borrow_mut","fmt","from","from_iter","into","ord","size","try_from","try_into","type_id"],"q":[[0,"zarts"]],"d":["座標圧縮です。","index 番目の値を返します","","","","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","集合内で小さいほうから何番目か (0-indexed) …","集合のサイズを返します","","",""],"i":[0,3,3,3,3,3,3,3,3,3,3,3,3],"f":[0,[[[3,[[0,[1,2]]]],4],[[0,[1,2]]]],[[]],[[]],[[[3,[2]],5],6],[[]],[7,[[3,[1]]]],[[]],[[[3,[[0,[1,2]]]],[0,[1,2]]],4],[3,4],[[],8],[[],8],[[],9]],"c":[],"p":[[8,"Ord"],[8,"Debug"],[3,"SortedSeq"],[15,"usize"],[3,"Formatter"],[6,"Result"],[8,"IntoIterator"],[4,"Result"],[3,"TypeId"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
