const playwright = require('playwright');

(async () => {
  for (const browserType of ['chromium']) {
// for (const browserType of ['chromium', 'firefox', 'webkit']) {
    // ここのbrowserTypeを変えることで、対象のブラウザを変更することができます。
    const browser = await playwright[browserType].launch();
    const context = await browser.newContext();
    const page = await context.newPage();

    await page.goto('http://localhost:8000/');
    // await page.screenshot({ path: `example-${browserType}.png` });
    const result = await page.evaluate(() => {
        const root = document.querySelector('#root').innerText;
        if (root === "hello wasm") {
            return true;
        }
        return false;
    });
    if (result) {
        console.log("test_pass: innerText")
    }else{
        console.log("test_FAIL: innerText")
    }

    await browser.close();
  }
})();