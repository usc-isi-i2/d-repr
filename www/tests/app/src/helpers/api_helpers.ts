import { Page } from "puppeteer";

const waitForResponse = async (
  p: Page,
  method: string,
  urlRegex: RegExp,
  shouldSuccess: boolean = false,
  enableLogging: boolean = false
) => {
  const resp = await p.waitForResponse(response => {
    if (enableLogging) {
      global.console.log(
        `XHR ${response.request().method()} ${response.url()}`
      );
    }
    return (
      response.url().match(urlRegex) !== null &&
      response.request().method() === method
    );
  });

  if (!resp.ok() && shouldSuccess) {
    global.console.error("error while trying to create resource:", resp.text());
  }

  expect(resp.status()).toEqual(200);

  return resp.status();
};

export const $api = {
  resource: {
    waitForCreation: (
      p: Page,
      shouldSuccess: boolean = true,
      enableLogging: boolean = false
    ) =>
      waitForResponse(
        p,
        "POST",
        /\/datasets\/[^/]+\/resources\/?/,
        shouldSuccess,
        enableLogging
      ),
    waitForDeletion: (
      p: Page,
      shouldSuccess: boolean = true,
      enableLogging: boolean = false
    ) =>
      waitForResponse(
        p,
        "DELETE",
        /\/datasets\/[^/]+\/resources\/[^/]+\/?/,
        shouldSuccess,
        enableLogging
      )
  }
};
