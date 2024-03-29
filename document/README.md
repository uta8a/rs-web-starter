## 目標
- Actix-webで作ったサーバをDockerに載せて、フロントエンドをyewで書いてDockerに載せて、AWSにデプロイして確認までを爆速で行いたい

## やること
- 必要なものをローカル開発PCにansibleで導入する(git, python3, ansibleを入れる立ち上げ用のshellscriptも書いておく)
- infra
  - terraformを用いてAWSのEC2などを立ち上げる
  - ansibleを使ってリモートサーバに対して設定を流す
- backend
  - Rust(Actix-web) -> ok
- リバースプロキシ
  - nginx(Docker)
  - frontendもここに載せる
- HTTPS
  - 証明書の手順もある程度自動化したい
  - 独自ドメイン割り振りもやりたい
- todoアプリを作る -> だいたいok

## やらないこと
- フロントエンド(あとでやってもいいかも？Elmを使いたい)
  - yewを使うことにしました。フロントエンドやります。
- デザイン
- 認証(あとでやってもいいかも？cookie-session方式でやる)

## dev
```
$ make db # Docker composeでDB立ち上げて、localでの接続やdiesel_cliに使う
$ make db_down
```

## yew
- https://yew.rs/docs/en/getting-started/project-setup
- wasm関連のツールをインストールする必要がありそう。
- cargo-web/wasm-bindgen/wasm-pack を入れる
- yarnを入れる(node, npmはもとから入っていた)

## note
- infra部分は分割できる。
- 上に載せるやつはDocker導入したあとで、そこからtarでかためたDockerImageを送り込んで向こうで展開してloadという手順を考えている(private registryは予算的に厳しいかも、でもやったほうがいいかもしれないので2通り用意したい)
- nginxでHTTPSを担保してリバースプロキシする
- どこまでサーバを作り込む？DB接続と初期化はしておきたい気がする
- develop/とproduction/を作っておいた方がいいかも。設定ファイルはそこに入れておくみたいな
- はじめからDockerを想定した開発にしてみる？でもcargo結局使いそうなのであれな気もするけど
- sqlxを断念
  - Docker multi-stage buildを使いたいが、ビルドステージでバイナリを作るときにdbコンテナへの接続が必要になり、この段階でCMDでdb waitを行うことはできないので無理
  - dieselを使おう
- prodでHTTPSするときはこれやればよさそう https://qiita.com/tnoce/items/ded6d3d298da5972ab63
  - ひとまず、VirtualHostはしない方針で(1サーバで完結)
- 構成
```
nginx(Host, VirtualHostと監視を行う) : 80 --> nginx(docker, reverse proxy) : 8000
                                               |--> nginx(frontend, static file deliver) : 8001
                                               |--> backend(docker server) : 8002
```
- 開発段階では、ReverseProxyまで(Docker)をまとめてcomposeにして使う。
- 権限的には、Host nginxが80番で動くのでRoot、DockerはRoot、composeはNonRoot
- 最小限のAPI構成を作る。
  - / 使い方示す
  - /health 単純な文字列を返す -> /api/health
  - /init 最初にDBに初期データ投入して、それをGETで引っ張ってくる -> /api/check GET
  - /init ボタン押すとDBにPOSTでデータ入れる -> /api/check POST
- yewの使い方が分からない。
  - Router
    - https://github.com/yewstack/yew/tree/master/examples/router
    - でもこのexampleはwasm-packを使ってない？
    - trunk https://github.com/thedodd/trunk というビルドシステムを使っている
      - えーでもこれ以上複雑にするのはどうなんだろう。とりあえずwebpackでやっていく
  - どこまでできるのか
    - Elm形式だったら、portがめっちゃ必要になるはず？だけどwasm使っているならある程度Rustで書けるはず(web-sysとか)
- webpack-dev-serverはインメモリで展開するやつで、開発時だけ気にすればよい
- Cargoでwasm吐き出すのはどこで指定しているんだ...
  - WasmPackPluginで指定している、webpackの設定ファイルと同じディレクトリに``pkg``ディレクトリを作る。これをimportすると、Rust側のもろもろがjsから使えるようになる。
  - このとき、WasmPackPluginにはCargo.tomlのあるディレクトリを渡す。今回は同じ階層なので、``.``を渡している。
- yewのawesomeを発見した https://github.com/jetli/awesome-yew
- yewのテンプレートをElm architectureに沿って理解チャレンジした記録
  - https://github.com/uta8a/rs-web-starter/blob/7b8d35ab46d5864c4b125892144b29a0df61f6b3/frontend/src/app.rs
- 

## trouble shoot
- 配信されてるっぽいのに真っ白 `Uncaught (in promise) TypeError: Response has unsupported MIME type`
  - これはサーバ(静的ファイルホスト側)がwasmを送れない(Header対応してない)ことが原因
  - wasmについてtypesを入れると解決。`default_type`もplainに変更

## infra
- AWSでEC2をterraformで立てることができた。ssh接続はまだなので次はssh接続から。
- awsは月の使用量Free枠があるみたいで、一瞬で立てて消すだけではお金はかからない(ただ、Route53はお金かかるので注意)

## frontend(yew)
- reverse proxyとフロントエンド一体化させた
- yewについて、テスト駆動開発をしてElmのようにView, Model, Updateを書いていくスタイルをしたかったのだが、これが無理だと思ったのでテストを書くのを断念
  - 理由 ``wasm_bindgen_test`` attributeがついているので ``wasm-pack test --firefox`` ``wasm-pack test --firefox --headless`` でテストする。このとき ``tests/web.rs``のテストが主に実行されるのだけど、headlessはえらーで落ちる。
  - まずはchromeをいれてwebdriverとのメインバージョンを合わせる。
  - 次に、 ``RUST_LOG=wasm_bindgen_test_runner wasm-pack test --headless --chrome``としてログを見る
  - ``driver status: signal: 9 `` ``Error: failed to find element reference in response`` 系のログはおそらくdocumentが２つ存在していることが理由？これを解決する方法は見つけられなかった。
- yew側でDOM Nodeをとってきたいが、立ち上げの``yew::App::<Model>::new()``から直接``web_sys::document``相当のものを取ってくる方法が見つからない。
  - Scopeのnewが0.15->0.16の変更で潰されて、代わりに``get_component``が導入された(documentにはnewは``visible for testing``とあったので、テスト用だったと思われる)
  - ``let doc = yew::App::<Model>::new().mount_as_body().get_component().unwrap().view();``でVNodeを取得できるが、VNodeからクラスやid, innerTextを得る方法が分からなかった。
- 結局blackbox testではあるけれど、headless browserを扱えるplaywrightというmicrosoft製のjsライブラリを使ってテスト。
```js
const playwright = require('playwright');

(async () => {
  for (const browserType of ['chromium', 'firefox', 'webkit']) {
    // ここのbrowserTypeを変えることで、対象のブラウザを変更することができます。
    const browser = await playwright[browserType].launch();
    const context = await browser.newContext();
    const page = await context.newPage();

    await page.goto('http://localhost:8000/');
    await page.screenshot({ path: `example-${browserType}.png` });
    await browser.close();
  }
})();
```
- スクリーンショットをとると、firefox, webkitでは真っ白になったので、不具合かheadless browser側でwasmがONになっていない可能性がある。
- 結局chromiumでテストすることに

## yewの限界とフロントエンドフレームワークへの気持ち
- そもそもyewでCanvasとかやろうとすると結局``web_sys``などを使うことになる。``wasm_bindgen``を隠蔽するのがフレームワークの仕事だけど、まだ追いついていないように思える。Rustはunsafeなパターンを解決するためにメソッドがたくさん用意されていて、安全に賢い人が考えたパターンを使えると思っていて、wasmへのバインドも似たような方法で解決されてほしいと思っているので、フレームワークはたくさんのメソッドと最小限のexampleを用意すべきという考えに至る。
- 他のpercyなどのライブラリが結局何をしているかというと、``wasm_bindgen``のwrapperとして、特に``html!``マクロ``Virtual DOM``の提供になると思う。js周りは触っていないのでまだ分からない。
- つまり、html周りに関してはこの2つを使い勝手の良い形、型検査を入れる形でやればライブラリが作れるのでは？という感じ。

## そもそもハッカソンに向けて何を整備すべきか？
- そもそも、ハッカソンはチーム戦なので、Rust好きで固めないとこのリポジトリを役立てるのは難しそう。
- ブラウザ対応の問題もある、Safariってどうなんだろう
- ハッカソンでは高速に作り上げることが大事なので、NuxtjsやNextjsで、Vercelなどを使うべきなのかも...？
- フロントエンドRustはまだまだなので、TypeScript(Nextjs)+Server(Actix)みたいな感じにするとして、このバックエンド側をめちゃくちゃ高速に開発できるようにする方が準備としては正しいのかもしれない。

## log
- 2020/10/27
  - ディレクトリ構成など
  - dependency入れた(Cargo.toml, actix-web 3.1.0)
  - develop docker-compose, Makefileで開発環境整えた。
- 2020/10/28
  - dieselでサンプルを構成([Auth0の記事](https://auth0.com/blog/build-an-api-in-rust-with-jwt-authentication-using-actix-web/)を参考にした)
  - うわものはとりあえずこれでいい気がするので、次はnginxでリバースプロキシを噛ませる
  - プロダクションでやるときの.envも新しくする
- 2020/11/03
  - CTF ScoreServer v6を作りたいが、それはこのリポジトリを完成させてからの方がいい気がしてきた。
  - REST/Postgres/yewで構成したいので、これらを加える。
  - templateを使ってlocalでyewの立ち上げ完了。これを書き換えていく
  - yewとactixの連携をしたい。単純にAPIサーバとして分離した形をとるので、あまりexampleみなくてもできそう。ただ、Dockerに2つとも別々に載せてcomposeで確認する手間はあるかも
  - devの方を確定させたい。devでcomposeでfrontとbackの連携をとってnginx reverse proxyまでいきたい。
- 2020/11/04
  - nginx複数やっていいのかと思ったけど、ファイルをホストするとことIPbanを行うところ一緒だといろいろめんどくさいというか、単一nginxに盛る理由はなさそう。速度低下が気になるけど、今回はそういうところを考えて完成までいきつかないリスクが大きいので許容する。
  - CDNとかのパターンも経験してみたいけど今回は後回しで。
  - 次はbackend側のAPIサーバを作り込みするか。最小限だしすぐつくれるやろ。
- 2020/11/12
  - infra以外を作ってしまって、infraのみにする
  - よく考えたらDockerfileもPwnみたいにしたほうがいいのかな？
  - frontendについては、nginx(reverse proxy, docker)にのせる(prod)
  - 開発段階ではまだ独立したDockerfileでやってよい？でもそのうちbackendもいれないとなあ
  - reverse proxyとfrontendを合体させた
  - webpack分からない
  - webpack少し分かった
  - actix-web-starterからrs-web-starterに変更
  - Elm architectureに沿って、カウンタアプリを作ってみる？その後それにrouter導入していくみたいな