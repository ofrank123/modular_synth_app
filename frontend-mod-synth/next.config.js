const path = require("path");

/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
};

module.exports = {
  ...nextConfig,
  sassOptions: {
    includePaths: ["**/*.scss"],
    prependData: `@import "./src/styles/variables.scss";`,
  },
  webpack: (config, options) => {
    config.experiments = {
      ...(config.experiments || {}),
      asyncWebAssembly: true,
    };

    return config;
  },
};
