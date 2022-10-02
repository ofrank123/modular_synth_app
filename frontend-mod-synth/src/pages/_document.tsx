import React from "react";
import { Html, Head, Main, NextScript } from "next/document";
import Script from "next/script";

export default function Document() {
  return (
    <Html>
      <Head />
      <body>
        <Main />
        <NextScript />
        <Script
          src="polyfill_worklet_import.js"
          type="module"
          strategy="beforeInteractive"
        ></Script>
      </body>
    </Html>
  );
}
