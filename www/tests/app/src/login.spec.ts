import {
  createTestEnvironment,
  TestEnvironment,
  destroyTestEnvironment
} from "./helpers/env";
import { $app } from "./helpers/app_helpers";

describe("test login procedure", () => {
  let env: TestEnvironment;

  beforeAll(async () => {
    jest.setTimeout(100000);
    env = await createTestEnvironment();
  });

  afterAll(async () => {
    await expect(destroyTestEnvironment(env.id)).resolves.toBeTruthy();
  });

  it("test login procedure", async () => {
    // should show login form
    await page.goto(process.env.APP!);
    await expect(page.$("[data-testid=user-login-form]")).not.toBeNull();

    // submit username and password
    await page.type("[data-testid=user-login-form-email]", env.username);
    await page.type("[data-testid=user-login-form-password]", env.password);
    await page.click("[data-testid=user-login-form-login-button]");
    await page.waitForSelector("[data-testid=user-login-form]", {
      hidden: true,
      timeout: 3000
    });

    // I am still login when reseting the page
    await page.goto(process.env.APP!);
    await page.waitForSelector("[data-testid=user-login-form]", {
      hidden: true,
      timeout: 3000
    });
  });

  it("login function should run correctly", async () => {
    const context = await browser.createIncognitoBrowserContext();
    const page = await context.newPage();

    await page.goto(process.env.APP!);
    await $app.users.login(page, env);
    await context.close();
  });
});
