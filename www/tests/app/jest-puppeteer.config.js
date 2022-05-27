module.exports = {
  launch: {
    dumpio: false,
    headless: process.env.HEADLESS !== "false",
    defaultViewport: {
      width: 1280,
      height: 720
    },
    sloMo: 200,
    args: [`--window-size=1280,720`]
  },
  browserContext: process.env.BROWSER_CONTEXT
};
