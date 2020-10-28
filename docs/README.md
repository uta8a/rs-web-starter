## 目標
- Actix-webで作ったサーバをDockerに載せて、AWSにデプロイして確認までを爆速で行いたい

## やること
- 必要なものをローカル開発PCにansibleで導入する(git, python3, ansibleを入れる立ち上げ用のshellscriptも書いておく)
- infra
  - terraformを用いてAWSのEC2などを立ち上げる
  - ansibleを使ってリモートサーバに対して設定を流す
- backend
  - Rust(Actix-web)
- リバースプロキシ
  - nginx(Docker)
- HTTPS
  - 証明書の手順もある程度自動化したい
  - 独自ドメイン割り振りもやりたい
- todoアプリを作る

## やらないこと
- フロントエンド(あとでやってもいいかも？Elmを使いたい)
- デザイン
- 認証(あとでやってもいいかも？cookie-session方式でやる)

## dev
```
$ make db # Docker composeでDB立ち上げて、初期sql投入

$ cd backend
$ cargo check # localが早いので、checkだけは毎回docker-compose前に行う。envではlocalhostを指定している(hostからlocalhostに接続したDBに向かってつなぐ)
$ make db_down

# buildするときにコンパイルエラーになりそう。どうしようかな
```

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

## log
- 2020/10/27
  - ディレクトリ構成など
  - dependency入れた(Cargo.toml, actix-web 3.1.0)
  - develop docker-compose, Makefileで開発環境整えた。

ビルド時もdb必要ぽいです
query!などでコンパイル時チェック使うならビルド時にDATABASE_URLを指定してね、そいつは実行時と同じスキーマにしてねとあるので