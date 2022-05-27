import {
  createTestEnvironment,
  destroyTestEnvironment,
  TestEnvironment
} from "../helpers/env";
import { $app } from "../helpers/app_helpers";
import { $antd } from "../helpers/antd_helpers";
import { $api } from "../helpers/api_helpers";
import { $c } from "../helpers/app_elements";

describe("test create resource procedure", () => {
  let env: TestEnvironment;

  beforeAll(async () => {
    jest.setTimeout(100000);
    await page.goto(process.env.APP!);
    env = await createTestEnvironment();
  });

  afterAll(async () => {
    await expect(destroyTestEnvironment(env.id)).resolves.toBeTruthy();
  });

  it("login and open resource tab", async () => {
    await $app.users.login(page, env);
    await $app.pages.resource.open(page);
  });

  it("fill the form to create resource the first time", async () => {
    // open a resource form
    await page.click($c.resource.createResourceBtn);
    await expect(
      page.waitForSelector($c.resource.upsert.resourceId)
    ).resolves.not.toBeNull();

    await expect($app.resources.size(page)).resolves.toBe(0);
    // clear resource id
    await page.$eval(
      $c.resource.upsert.resourceId,
      (el: HTMLInputElement) => (el.value = "")
    );
    await $antd.staticSelect.select(
      page,
      $c.resource.upsert.resourceType,
      "csv"
    );
    const uploadInput = await page.$($c.resource.upsert.resourceFile);
    expect(uploadInput).not.toBeNull();
    await uploadInput!.uploadFile(
      "<rootDir>/../../resources/s001_kaya_rice_import.csv"
    );

    // submit form, expect to have one resource
    await Promise.all([
      $api.resource.waitForCreation(page),
      page.click($c.resource.upsert.submitBtn)
    ]);
    await expect($app.resources.size(page)).resolves.toBe(1);
    await expect(page.$($c.resource.upsert.resourceId)).resolves.toBeNull();
  });

  it("fill the form to create resource again", async () => {
    // checking the state: have one resource, and upsert form is not open
    await expect($app.resources.size(page)).resolves.toBe(1);
    await expect(page.$($c.resource.upsert.resourceId)).resolves.toBeNull();

    // open upsert form
    await page.click($c.resource.createResourceBtn);
    await expect(
      page.waitForSelector($c.resource.upsert.resourceId)
    ).resolves.not.toBeNull();

    // adding information without resource id
    await page.type($c.resource.upsert.resourceId, "");
    await $antd.staticSelect.select(
      page,
      $c.resource.upsert.resourceType,
      "csv"
    );
    const uploadInput = await page.$($c.resource.upsert.resourceFile);
    expect(uploadInput).not.toBeNull();
    await uploadInput!.uploadFile(
      "<rootDir>/../../resources/s101_time_row.csv"
    );

    // click submit will throw a warning and not send request to server
    await page.click($c.resource.upsert.submitBtn);
    await expect(
      page.waitForXPath($c.resource.upsert.resourceIdError)
    ).resolves.not.toBeNull();

    // now providing resource id and we are able to create new resource
    await page.type($c.resource.upsert.resourceId, "resource2");

    await Promise.all([
      $api.resource.waitForCreation(page),
      page.click($c.resource.upsert.submitBtn)
    ]);
    await expect($app.resources.size(page)).resolves.toBe(2);
  });

  it("test create resource function", async () => {
    const nResources = await $app.resources.size(page);
    await expect(
      $app.resources.create(
        page,
        "resource-a",
        "<rootDir>/../../resources/s001_kaya_rice_import.csv"
      )
    ).resolves.toBeTruthy();
    await expect($app.resources.size(page)).resolves.toBe(nResources + 1);
  });
});

describe("test error handling", () => {
  let env: TestEnvironment;

  beforeAll(async () => {
    jest.setTimeout(100000);
    await page.goto(process.env.APP!);
    env = await createTestEnvironment();
  });

  afterAll(async () => {
    await expect(destroyTestEnvironment(env.id)).resolves.toBeTruthy();
  });

  it("login and open resource tab", async () => {
    await $app.users.login(page, env);
    await $app.pages.resource.open(page);

    // open a resource form
    await page.click($c.resource.createResourceBtn);
    await expect(
      page.waitForSelector($c.resource.upsert.resourceId)
    ).resolves.not.toBeNull();

    // submit the form without resource type will result as an error
    await page.click($c.resource.upsert.submitBtn);
    await expect(
      page.waitForXPath($c.resource.upsert.resourceTypeError)
    ).resolves.not.toBeNull();
    await $antd.staticSelect.select(
      page,
      $c.resource.upsert.resourceType,
      "csv"
    );

    // submit the form without remote file or upload file will be error
    await page.click($c.resource.upsert.submitBtn);
    await expect(
      page.waitForXPath($c.resource.upsert.resourceURLError)
    ).resolves.not.toBeNull();

    // enter url should dismiss the error
    await page.type($c.resource.upsert.resourceURL, "http://example.org");
    await expect(
      page.waitForXPath($c.resource.upsert.resourceURLError, { hidden: true })
    ).resolves.toBeNull();

    // upload a file should dismiss the error (reproduce the previous error first)
    await $antd.input.clear(page, $c.resource.upsert.resourceURL);
    await page.click($c.resource.upsert.submitBtn);
    await expect(
      page.waitForXPath($c.resource.upsert.resourceURLError)
    ).resolves.not.toBeNull();

    const uploadInput = await page.$($c.resource.upsert.resourceFile);
    expect(uploadInput).not.toBeNull();
    await uploadInput!.uploadFile(
      "<rootDir>/../../resources/s001_kaya_rice_import.csv"
    );

    await expect(
      page.waitForXPath($c.resource.upsert.resourceURLError, { hidden: true })
    ).resolves.toBeNull();
  });
});
