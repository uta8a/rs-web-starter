# frontend
## yew test
- 今のところ、srcと同じfile内でテストがよさそう。
- でもエラー出る

```
---- app::tests::it_works stdout ----
thread 'app::tests::it_works' panicked at 'cannot call wasm-bindgen imported functions on non-wasm targets', /home/uta8a/.cargo/registry/src/github.com-1ecc6299db9ec823/js-sys-0.3.45/src/lib.rs:4742:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    app::tests::it_works

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

error: test failed, to rerun pass '--lib'
```
- これはwasmターゲットにしていないからエラー

```
$ RUST_BACKTRACE=1 cargo test --target wasm32-unknown-unknown --lib -- --nocapture 
Running target/wasm32-unknown-unknown/debug/deps/yew_wasm_pack_template-81a963551b4323c2.wasm
/home/uta8a/project/workspace/rs-web-starter/frontend/target/wasm32-unknown-unknown/debug/deps/yew_wasm_pack_template-81a963551b4323c2.wasm: 1: Syntax error: end of file unexpected
error: test failed, to rerun pass '--lib'
```

- これはwasmターゲットにしたときにwasmをELFとして実行しようとしているのかエラー
- ここで、``wasm_bindgen_test_configure!(run_in_browser);``が大事になってくるのか？ヘッドレスブラウザでテストを行うと。
- https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/browsers.html
  - これがheadless browserについての記述
- ``wasm-pack test --firefox``
- ``wasm-pack test --firefox --headless``
- のいずれかでよさそう
- このときは``tests/web.rs``が実行される
- wasmで生成されたものに対して、Rustの状態からテストをかけるのはナンセンスな気がしてきた。ブラックボックステストのように、wasmが描画した画面を直接スクレイピングして関係が保存されているか調べるツールを回したほうがいいのではないか？


## troubleshoot
- chromeを入れたらエラーが変化した
- 
```
Try find `webdriver.json` for configure browser's capabilities:
Ok
driver status: signal: 9                          
driver stdout:
    Starting ChromeDriver 86.0.4240.22 (398b0743353ff36fb1b82468f63a3a93b4e2e89e-refs/branch-heads/4240@{#378}) on port 44101
    Only local connections are allowed.
    Please see https://chromedriver.chromium.org/security-considerations for suggestions on keeping ChromeDriver safe.
    ChromeDriver was started successfully.

Error: failed to find element reference in response
error: test failed, to rerun pass '--test web'
Error: Running Wasm tests with wasm-bindgen-test failed
Caused by: failed to execute `cargo test`: exited with exit code: 1
  full command: "cargo" "test" "--target" "wasm32-unknown-unknown"
```

```
RUST_LOG=wasm_bindgen_test_runner wasm-pack test --headless --chrome
```

```
Set timeout to 20 seconds...
[2020-11-13T00:54:21Z DEBUG wasm_bindgen_test_runner::headless] executing "/home/uta8a/.cache/.wasm-pack/chromedriver-20b989a31d295eb7/chromedriver" "--port=37625"
Running headless tests in Chrome on `http://127.0.0.1:37625/`
Try find `webdriver.json` for configure browser's capabilities:
Ok
[2020-11-13T00:54:21Z DEBUG wasm_bindgen_test_runner::headless] POST /session {"desiredCapabilities":{"goog:chromeOptions":{"args":["headless","disable-dev-shm-usage","no-sandbox"]},"moz:firefoxOptions":{"args":[],"prefs":{"media.navigator.permission.disabled":true,"media.navigator.streams.fake":true}}},"requiredCapabilities":{}}
[2020-11-13T00:54:21Z DEBUG wasm_bindgen_test_runner::headless] got: {"sessionId":"4ddbab3d7a9b1b969ae192c47e6676c2","status":0,"value":{"acceptInsecureCerts":false,"acceptSslCerts":false,"applicationCacheEnabled":false,"browserConnectionEnabled":false,"browserName":"chrome","chrome":{"chromedriverVersion":"86.0.4240.22 (398b0743353ff36fb1b82468f63a3a93b4e2e89e-refs/branch-heads/4240@{#378})","userDataDir":"/tmp/.com.google.Chrome.Qu6oOq"},"cssSelectorsEnabled":true,"databaseEnabled":false,"goog:chromeOptions":{"debuggerAddress":"localhost:35665"},"handlesAlerts":true,"hasTouchScreen":false,"javascriptEnabled":true,"locationContextEnabled":true,"mobileEmulationEnabled":false,"nativeEvents":true,"networkConnectionEnabled":false,"pageLoadStrategy":"normal","platform":"Linux","proxy":{},"rotatable":false,"setWindowRect":true,"strictFileInteractability":false,"takesHeapSnapshot":true,"takesScreenshot":true,"timeouts":{"implicit":0,"pageLoad":300000,"script":30000},"unexpectedAlertBehaviour":"ignore","version":"86.0.4240.198","webStorageEnabled":true,"webauthn:virtualAuthenticators":true}}
[2020-11-13T00:54:21Z DEBUG wasm_bindgen_test_runner::headless] POST /session/4ddbab3d7a9b1b969ae192c47e6676c2/url {"url":"http://127.0.0.1:37893"}
[2020-11-13T00:54:22Z DEBUG wasm_bindgen_test_runner::headless] got: {"sessionId":"4ddbab3d7a9b1b969ae192c47e6676c2","status":0,"value":null}
[2020-11-13T00:54:22Z DEBUG wasm_bindgen_test_runner::headless] POST /session/4ddbab3d7a9b1b969ae192c47e6676c2/element {"using":"css selector","value":"#output"}
[2020-11-13T00:54:22Z DEBUG wasm_bindgen_test_runner::headless] got: {"sessionId":"4ddbab3d7a9b1b969ae192c47e6676c2","status":0,"value":{"ELEMENT":"0.0773028785166856-1"}}
[2020-11-13T00:54:22Z DEBUG wasm_bindgen_test_runner::headless] POST /session/4ddbab3d7a9b1b969ae192c47e6676c2/element {"using":"css selector","value":"#console_log"}
[2020-11-13T00:54:22Z DEBUG wasm_bindgen_test_runner::headless] got: {"sessionId":"4ddbab3d7a9b1b969ae192c47e6676c2","status":7,"value":{"message":"no such element: Unable to locate element: {\"method\":\"css selector\",\"selector\":\"#console_log\"}\n  (Session info: headless chrome=86.0.4240.198)\n  (Driver info: chromedriver=86.0.4240.22 (398b0743353ff36fb1b82468f63a3a93b4e2e89e-refs/branch-heads/4240@{#378}),platform=Linux 5.4.0-52-generic x86_64)"}}
[2020-11-13T00:54:22Z DEBUG wasm_bindgen_test_runner::headless] DELETE /session/4ddbab3d7a9b1b969ae192c47e6676c2/window
[2020-11-13T00:54:22Z DEBUG wasm_bindgen_test_runner::headless] got: {"sessionId":"4ddbab3d7a9b1b969ae192c47e6676c2","status":0,"value":[]}
driver status: signal: 9                          
driver stdout:
    Starting ChromeDriver 86.0.4240.22 (398b0743353ff36fb1b82468f63a3a93b4e2e89e-refs/branch-heads/4240@{#378}) on port 37625
    Only local connections are allowed.
    Please see https://chromedriver.chromium.org/security-considerations for suggestions on keeping ChromeDriver safe.
    ChromeDriver was started successfully.

Error: failed to find element reference in response
error: test failed, to rerun pass '--test web'
Error: Running Wasm tests with wasm-bindgen-test failed
Caused by: failed to execute `cargo test`: exited with exit code: 1
  full command: "cargo" "test" "--target" "wasm32-unknown-unknown"
```
- chrome ver 86であってそう
- documentを宣言してAppModelに組み込むことはできる(mount)
- ではその逆は？
- イマイチ``web_sys::document``と``Model``の関係性が分かっていない
- html::Scopeのnewが0.15->0.16のタイミングで消えている
  - https://github.com/yewstack/yew/commit/9c0951513d04da973d4695a48f6426aad43b99be
  - 該当コミットを見ると``get_component``で代用できる的な雰囲気が出ている
- でも最初に得るにはどうしたらいいんだい
- Scopeを得たい
- ``let doc = yew::App::<Model>::new().mount_as_body().get_component().unwrap().view();``でいけそうな感じがしたけどうまくいかない。VNodeの取得には成功しているはず
  - その先が無理だっだ。VNodeからテキスト取り出せないし、何かをVNodeに変換する方法も分からない。

```
error[E0308]: mismatched types
  --> tests/web.rs:37:5
   |
37 |     assert_eq!(doc.view(), "hoge");
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `yew::virtual_dom::VNode`, found `&str`
```
- そもそもブラウザを立ち上げてのテストで今回のようにやるときは、原因が知りたいときに有効というだけなのでは。
- ブラックボックステストでよいなら、 https://qiita.com/riku-shiru/items/fb3677780802ea90bd5e のようなことをすればよさそう。これで行こう。
- playwright
  - ``sudo apt-get install libenchant1c2a``
  - chromeのみ対応か...
```
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
- chrome以外では真っ白だった。wasm対応かなあ。
- chromiumでテストする、他の環境は後回しにする。
- シュッとやりたいことができてしまったので、フロントHTMLのテスト駆動でViewを書き込んでいこうと思う。