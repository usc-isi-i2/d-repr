import {
  createTestEnvironment,
  TestEnvironment,
  destroyTestEnvironment
} from "../helpers/env";
import { $app } from "../helpers/app_helpers";
import { $c } from "../helpers/app_elements";
import { $antd } from "../helpers/antd_helpers";
import { $api } from "../helpers/api_helpers";

describe("test delete resource", () => {
  let env: TestEnvironment;

  beforeAll(async () => {
    jest.setTimeout(1000000);
    await page.goto(process.env.APP!);
    env = await createTestEnvironment();
  });

  afterAll(async () => {
    await expect(destroyTestEnvironment(env.id)).resolves.toBeTruthy();
  });

  it("login and open resource tab", async () => {
    await expect($app.users.login(page, env)).resolves.toBeTruthy();
    await $app.pages.resource.open(page);
  });

  it("can delete existing resoruce", async () => {
    const rid = "resource-a";
    const nResources = await $app.resources.size(page);
    // create resource
    await expect(
      $app.resources.create(
        page,
        rid,
        "<rootDir>/../../resources/s001_kaya_rice_import.csv"
      )
    ).resolves.toBeTruthy();
    await expect($app.resources.size(page)).resolves.toEqual(nResources + 1);

    // view the created resource first
    await page.click($c.resource.minimizedList.resource(rid));
    await expect(
      page.waitForSelector($c.resource.header.this(rid))
    ).resolves.not.toBeNull();

    // click delete button, and confirm => no more resources
    await page.click($c.resource.header.closeButton(rid));
    await Promise.all([
      $api.resource.waitForDeletion(page),
      $antd.modal.yes(page)
    ]);
    await $antd.modal.waitUntilDisappear(page);
    await expect($app.resources.size(page)).resolves.toEqual(nResources);
  });

  it("delete method works correctly ", async () => {
    const rid = "resource-a";
    await expect($app.resources.size(page)).resolves.toEqual(0);
    // create resource
    await $app.resources.create(
      page,
      rid,
      "<rootDir>/../../resources/s001_kaya_rice_import.csv"
    );
    await expect($app.resources.size(page)).resolves.toEqual(1);
    await $app.resources.delete(page, rid);
    await expect($app.resources.size(page)).resolves.toEqual(0);
  });
});
