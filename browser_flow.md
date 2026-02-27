# ブラウザ処理フロー：コードと紐づけた解説

---

## 全体フロー図

```
ユーザーがURLを入力（アドレスバー）
         │
         ▼
 [1] URL解析
   saba_core/src/url.rs
   Url::parse()
         │
         ▼
 [2] HTTPリクエスト送信
   net/wasabi/src/http.rs
   HttpClient::get()
         │
         ▼
 [3] HTTPレスポンス受信・解析
   saba_core/src/http.rs
   HttpResponse::new()
         │
         ▼ ←─────────────────────────────────────────┐
 Page::receive_response()  ← ここからがPage主導      │
   saba_core/src/renderer/page.rs                    │
         │                                            │
         ↓                                            │
        [4] HTML字句解析（トークナイズ）               │
         renderer/html/token.rs                       │
         HtmlTokenizer                                │
         │                                            │
         ↓                                            │
        [5] HTML構文解析（DOMツリー構築）               │
         renderer/html/parser.rs                      │
         HtmlParser::construct_tree()                 │
         │                                            │
         ↓                                            │
        [6] CSS解析（CSSOMツリー構築）                 │
         renderer/css/cssom.rs                        │
         CssParser::parse_stylesheet()                │
         │                                            │
         ↓                                            │
        [7] JavaScript実行（DOM変更）                  │
         renderer/js/runtime.rs                       │
         JsRuntime::execute()                         │
         │                                            │
         ↓                                            │
        [8] レイアウトツリー構築                       │
         renderer/layout/layout_view.rs               │
         LayoutView::new()                            │
         ├─ スタイル計算（カスケード・継承）             │
         ├─ サイズ計算                                │
         └─ 座標（位置）計算                           │
         │                                            │
         ↓                                            │
        [9] ペインティング（DisplayItem生成）           │
         display_item.rs                              │
         LayoutView::paint()                          │
                    │                                 │
                    ▼                                 │
         [10] 画面描画（OSへの描画命令）                │
           ui/wasabi/src/app.rs                       │
           WasabiUI::update_ui()                      │
                    │                                 │
                    ▼                                 │
               画面表示                               │
                    │                                 │
          リンクをクリック → URLを取得 ───────────────┘
```

---

## 具体例：データが各段階でどう変わるか

`<p class="foo">hello</p>` と `.foo { color: red }` を例に追う。

**[4] → トークン列**
```
StartTag{ tag: "p", attributes: [{class, foo}] }
Char('h'), Char('e'), Char('l'), Char('l'), Char('o')
EndTag{ tag: "p" }
Eof
```

**[5] → DOMツリー**
```
Document
  └─ Element(html)
       └─ Element(body)
            └─ Element(p) [attributes: class="foo"]
                 └─ Text("hello")
```

**[6] → CSSOM**
```
StyleSheet
  └─ QualifiedRule
       ├─ selector: ClassSelector("foo")
       └─ declarations: [{ property: "color", value: Ident("red") }]
```

**[8] → レイアウトツリー**
```
LayoutObject { kind: Block, style: { color: red, display: Block }, point: (0,0), size: (800,20) }
  └─ LayoutObject { kind: Text("hello"), style: { color: red }, point: (0,0) }
```

**[9] → DisplayItemリスト**
```
[
  Rect  { point: (0,0), size: (800,20), bg_color: white },
  Text  { text: "hello", color: red, point: (0,0) },
]
```

**[10] → OSへの描画命令**
```
window.fill_rect(white, x+PADDING, y+PADDING+TOOLBAR_HEIGHT, 800, 20)
window.draw_string(red, x+PADDING, y+PADDING+TOOLBAR_HEIGHT, "hello", ...)
window.flush()
```

---

## 各段階の詳細

---

### [1] URL解析

**ファイル**：`saba_core/src/url.rs`

**処理内容**：
- `http://example.com:8080/path?query` という文字列を分解する
- `host = "example.com"`, `port = "8080"`, `path = "/path"`, `searchpart = "query"`
- HTTPスキームのみ対応（httpsは非対応）
- portが省略された場合はデフォルト `"80"` を使用

**呼び出し元**：`src/main.rs` の `handle_url()` 関数

```rust
// src/main.rs（概略）
fn handle_url(url: String) -> Result<HttpResponse, Error> {
    let parsed_url = Url::new(url).parse()?;
    // host, port, path を取り出してHTTPクライアントに渡す
}
```

**ポイント**：
> URL は「どのサーバーの（host+port）」「何を（path）」取りに行くかの指示書。ブラウザはまずこれを解析して、次のHTTP通信の宛先と対象を決める。

---

### [2] HTTPリクエスト送信

**ファイル**：`net/wasabi/src/http.rs`

**処理内容**（`HttpClient::get()` の流れ）：
1. `lookup_host(host)` でDNS解決 → IPアドレス取得
2. `TcpStream::connect(ip, port)` でTCP接続確立
3. GETリクエスト文字列を送信（`Host:`, `Accept:`, `Connection: close` ヘッダ付き）
4. 4KBチャンクずつレスポンスを受信して文字列に結合

**ポイント**：
> DNS でドメイン名をIPアドレスに変換し、そのIPにTCPでコネクションを張って、HTTP の GET リクエストを送る。HTTP は TCP の上に乗っているテキストプロトコル。

---

### [3] HTTPレスポンス受信・解析

**ファイル**：`saba_core/src/http.rs`

**処理内容**（`HttpResponse::new()` でパース）：
- 生テキスト（バイト列→UTF-8文字列）を受け取り構造化する
- 1行目：`HTTP/1.1 200 OK` → version / status_code / reason に分解
- 続く行：`Name: Value` → `Vec<Header>` に格納
- 空行以降：body として保存

**リダイレクト処理**（`src/main.rs`）：
```
status_code == 302 → header_value("Location") を取得 → 再度 handle_url() を呼ぶ
```

**ポイント**：
> サーバーからの返答はテキストで来る。最初の行にステータスコード（200=成功、302=リダイレクト等）、次にヘッダ情報、空行を挟んでボディ（HTMLなど）が続く構造。

---

### [4] HTML字句解析（トークナイズ）

**ファイル**：`saba_core/src/renderer/html/token.rs`

**`HtmlToken` の種類**（`HtmlTokenizer` が `State` ステートマシンで生成）：
```rust
enum HtmlToken {
    StartTag { tag, self_closing, attributes },  // <div class="foo">
    EndTag { tag },                               // </div>
    Char(char),                                   // テキストの1文字
    Eof,                                          // 終端
}
```

**ステートマシンの主な状態（`State`）**：
- `Data` → 通常テキスト読み取り中
- `TagOpen` → `<` を読んだ直後
- `TagName` → タグ名を読み取り中
- `BeforeAttributeName` / `AttributeName` / `AttributeValueDoubleQuoted` → 属性解析
- `ScriptData` → `<script>` 内の特殊処理

**ポイント**：
> HTMLパーサの最初のステップは「字句解析」。`<`, タグ名, 属性, `>`, テキスト... というように、HTML文字列を意味のかたまり（トークン）に切り分ける。ステートマシンで実装するのが標準的。

---

### [5] HTML構文解析（DOMツリー構築）

**ファイル**：`saba_core/src/renderer/html/parser.rs`、DOM定義は `saba_core/src/renderer/dom/node.rs`

**DOMツリーの構造**（`<p>hello</p>` の場合）：
```
Document
  └─ Element(html)
       └─ Element(body)
            └─ Element(p)
                 └─ Text("hello")
```

**`InsertionMode`（挿入モード）**：
```
Initial → BeforeHtml → BeforeHead → InHead → AfterHead
       → InBody → Text → AfterBody → AfterAfterBody
```

**ツリー構築の仕組み**：
1. トークンを1つずつ受け取る
2. `InsertionMode` に応じて処理を切り替え
3. `StartTag` → `stack_of_open_elements` にプッシュしてノードを子として追加
4. `EndTag` → スタックからポップ
5. `Char` → テキストノードとして現在のノードに追加

**DOMノードの構造**（`node.rs`）：
```rust
struct Node {
    kind: NodeKind,              // Document / Element(Element) / Text(String)
    parent: Weak<RefCell<Node>>,
    first_child: Option<Rc<RefCell<Node>>>,
    next_sibling: Option<Rc<RefCell<Node>>>,
    // ... （双方向リンクリスト構造）
}
```

**サポートする要素**：`html, head, body, p, h1, h2, a, style, script`

**ポイント**：
> トークンを使って「木構造（DOMツリー）」を作る。`<div><p>text</p></div>` ならdivが親、pが子、textがpの子になる。スタックを使って「今どの要素の中にいるか」を追跡しながら構築する。

---

### [6] CSS解析（CSSOMツリー構築）

**ファイル**：字句解析 `saba_core/src/renderer/css/token.rs`、構文解析 `saba_core/src/renderer/css/cssom.rs`、CSS取得 `saba_core/src/renderer/dom/api.rs`

**CSSOMの構造**（`get_style_content()` で `<style>` タグから取得後、`CssParser::parse_stylesheet()` でパース）：
```
StyleSheet
  └─ Vec<QualifiedRule>
       ├─ selector: Selector
       │    ├─ TypeSelector("p")    // p { ... }
       │    ├─ ClassSelector("foo") // .foo { ... }
       │    └─ IdSelector("bar")    // #bar { ... }
       └─ declarations: Vec<Declaration>
            ├─ property: "color"
            └─ value: CssToken (Ident("red") など)
```

**ポイント**：
> CSSもHTMLと同様に字句解析→構文解析の2段階で処理される。結果として「セレクタ→スタイル宣言」のリスト（CSSOM）が得られる。これを後でDOMノードにマッチングさせてスタイルを適用する。

---

### [7] JavaScript実行（DOM変更）

**ファイル**：`saba_core/src/renderer/js/`（`token.rs` → `JsLexer`、`ast.rs` → `JsParser`、`runtime.rs` → `JsRuntime`）

**呼び出し元**：`Page::execute_js()`（`renderer/page.rs`）

**処理の流れ**：
```
get_js_content(dom) で <script> タグ内容を取得
    ↓
JsLexer::new(js) → 字句解析
    ↓
JsParser::new(lexer).parse_ast() → AST構築
    ↓
JsRuntime::new(dom).execute(&ast) → DOM変更
```

**`JsRuntime` が持つDOMアクセスAPI**：
- `document.getElementById(id)` → DOM検索
- テキストノードへの代入 → DOM内容変更

**ポイント**：
> JavaScriptもHTMLやCSSと同様に字句解析→構文解析→実行の流れ。実行エンジン（ランタイム）はDOMへの参照を持っており、JavaScriptがDOMを書き換えることができる。これがJSで動的なページが作れる理由。

---

### [8] レイアウトツリー構築

**ファイル**：`saba_core/src/renderer/layout/computed_style.rs`、`layout_object.rs`、`layout_view.rs`

**① スタイル計算**：`build_layout_tree()` 内で各DOMノードに対して実行
- `LayoutObject::is_node_selected(selector)` でセレクタマッチング
- `LayoutObject::cascading_style(declarations)` でCSSOMのルールを適用
- `ComputedStyle::defaulting(node, parent_style)` で未指定プロパティを親から継承
- `ComputedStyle` が持つプロパティ：`color`、`background_color`、`display`、`font_size`、`text_decoration`、`width`、`height`

**デフォルト値の例**：
- `<h1>` → `font_size = XXLarge`, `display = Block`
- `<a>` → `text_decoration = Underline`
- `<p>` → `display = Block`

**② サイズ計算**：`calculate_node_size(node, parent_size)` で再帰的に確定
- `LayoutObject::compute_size(parent_size)` → 親サイズを基に幅・高さを決定

**③ 座標計算**：`calculate_node_position(node, parent_point, prev_sibling_*)` で再帰的に確定
- `LayoutObject::compute_position(parent_point, prev_sibling_*)` → 前の兄弟要素に基づいて配置

**Blockとインラインの違い**：
- `Block`：幅いっぱいに広がり、前後で改行（`<p>`, `<h1>`, `<h2>`）
- `Inline`：文章の流れに沿って横に並ぶ（`<a>`）
- `Text`：テキストノード（インラインに準じた扱い）

**ポイント**：
> レイアウト段階ではDOMツリーとCSSOMを合わせて「どのノードがどのスタイルを持つか」を決め（カスケード）、さらに「画面上のどこに何ピクセルで配置するか」を計算する。ブロック要素は縦に積み重なり、インライン要素は横に並ぶというのがCSSレイアウトの基本。

---

### [9] ペインティング（DisplayItem生成）

**ファイル**：`saba_core/src/display_item.rs`
**呼び出し元**：`LayoutView::paint()` → `LayoutObject::paint()` （`layout_view.rs`, `layout_object.rs`）

**`DisplayItem` の種類**：
```rust
enum DisplayItem {
    Rect {
        style: ComputedStyle,          // 背景色など
        layout_point: LayoutPoint,     // 左上座標 (x, y)
        layout_size: LayoutSize,       // 幅・高さ
    },
    Text {
        text: String,                  // 描画する文字列
        style: ComputedStyle,          // 文字色・フォントサイズ・下線など
        layout_point: LayoutPoint,     // 描画開始座標
    },
}
```

**処理内容**：
- レイアウトツリーを再帰的に走査
- 各 `LayoutObject` が `paint()` を呼ぶ → `DisplayItem` を生成してリストに追加
- 結果として `Vec<DisplayItem>` が得られる（描画順が保証されたフラットなリスト）

**ポイント**：
> レイアウト計算が終わったら、次は「実際に何を描くか」のリストを作る（ペインティング）。矩形（背景）とテキストの2種類の描画命令だけで構成されたフラットなリスト（DisplayItemリスト）に変換する。これによってレイアウト計算と実際の描画を分離できる。

---

### [10] 画面描画（OSへの描画命令）

**ファイル**：`ui/wasabi/src/app.rs`

**`update_ui()` の処理**（`WasabiUI` 構造体）：
```rust
for item in page.display_items() {
    match item {
        DisplayItem::Text { text, style, layout_point } =>
            window.draw_string(
                color, x + PADDING, y + PADDING + TOOLBAR_HEIGHT,
                &text, font_size, underline
            ),
        DisplayItem::Rect { style, layout_point, layout_size } =>
            window.fill_rect(
                bg_color, x + PADDING, y + PADDING + TOOLBAR_HEIGHT,
                width, height
            ),
    }
}
window.flush(); // フレームバッファに反映 → 画面に表示
```

**`WasabiUI` の全体的な役割**：
- `start(handle_url)` → メインループ起動
- `run_app()` → キー入力・マウス入力のイベントループ
- `handle_key_input()` → アドレスバーへのURL入力処理
- `handle_mouse_input()` → クリック検出 → `page.clicked(position)` でリンク取得 → `start_navigation()`
- `setup_toolbar()` → アドレスバーやボタンのUI描画

**ポイント**：
> 最終段階では、抽象的な「描画命令リスト（DisplayItem）」をOSのグラフィックAPIへの具体的な呼び出しに変換する。テキストならdraw_string、背景色の矩形ならfill_rect。最後にflush()でフレームバッファを更新することで画面に表示される。

---

## 全体のオーケストレーション構造

```
Browser（saba_core/src/browser.rs）
  └─ Vec<Page> を管理（マルチタブの概念）

WasabiUI（ui/wasabi/src/app.rs）
  └─ Browser を Rc<RefCell<>> で保持
  └─ start(handle_url) でメインループ開始
       ↓ URLが入力されると
  └─ start_navigation(url)
       ├─ handle_url(url) → HTTP通信 → HttpResponse
       └─ page.receive_response(response) → レンダリングパイプライン一式
            ├─ create_frame(html)    → [4][5][6] 実行
            ├─ execute_js()          → [7] 実行
            ├─ set_layout_view()     → [8] 実行
            └─ paint_tree()          → [9] 実行
       └─ update_ui()                → [10] 実行

Page（saba_core/src/renderer/page.rs）
  ├─ frame: Option<Window>           → DOMツリーの根
  ├─ style: Option<StyleSheet>       → CSSOMの根
  ├─ layout_view: Option<LayoutView> → レイアウトツリーの根
  └─ display_items: Vec<DisplayItem> → 最終的な描画命令リスト
```
