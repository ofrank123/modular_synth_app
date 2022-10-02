/*
Polyfills the addModule function of the AudioWorklet module to automatically rollup the module on a failure.  Removes the need for manually bundling AudioProcessor.
*/

const wrappedFunc = AudioWorklet.prototype.addModule;

AudioWorklet.prototype.addModule = async function (url) {
  try {
    return await wrappedFunc.call(this, url);
  } catch (e) {
    if (e.name != "AbortError") {
      throw e;
    }
    // assume error is caused by https://bugzilla.mozilla.org/show_bug.cgi?id=1572644
    console.warn("direct addModule call failed, resorting to bundling");
    const { rollup } = await import(
      "https://unpkg.com/rollup@2.78.0/dist/es/rollup.browser.js"
    );
    const generated = await (
      await rollup({
        input: url,
        onwarn: console.warn,
        plugins: [
          {
            resolveId(importee, importer) {
              return new URL(
                importee,
                new URL(importer || window.location.href)
              ).toString();
            },
            load(id) {
              return fetch(id).then((response) => response.text());
            },
          },
        ],
      })
    ).generate({});
    const blob = new Blob([generated.output[0].code], {
      type: "text/javascript",
    });
    const objectUrl = URL.createObjectURL(blob);
    try {
      return await wrappedFunc.call(this, objectUrl);
    } finally {
      URL.revokeObjectURL(objectUrl);
    }
  }
};
