## 目標
- Actix-webで作ったサーバをDockerに載せて、AWSにデプロイして確認までを爆速で行いたい

## やること
- 必要なものをローカル開発PCにansibleで導入する(git, python3, ansibleを入れる立ち上げ用のshellscriptも書いておく)
- infra
  - terraformを用いてAWSのEC2などを立ち上げる
  - ansibleを使ってリモートサーバに対して設定を流す
- backend
  - Rust(Actix-web) -> ok
- リバースプロキシ
  - nginx(Docker)
- HTTPS
  - 証明書の手順もある程度自動化したい
  - 独自ドメイン割り振りもやりたい
- todoアプリを作る -> だいたいok

## やらないこと
- フロントエンド(あとでやってもいいかも？Elmを使いたい)
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
- 

## trouble shoot
- 配信されてるっぽいのに真っ白 `Uncaught (in promise) TypeError: Response has unsupported MIME type`
  - これはサーバ(静的ファイルホスト側)がwasmを送れない(Header対応してない)ことが原因
  - wasmについてtypesを入れると解決。`default_type`もplainに変更

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
  - 次は