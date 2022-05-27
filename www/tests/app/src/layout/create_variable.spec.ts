import {
  TestEnvironment,
  createTestEnvironment,
  destroyTestEnvironment
} from "../helpers/env";
import { $app } from "../helpers/app_helpers";
import { $c } from "../helpers/app_elements";
import { $antd } from "../helpers/antd_helpers";

describe("test create variable", () => {
  let env: TestEnvironment;
  const resourceA = "resource-a";
  const resourceB = "resource-b";

  beforeAll(async () => {
    jest.setTimeout(100000);
    await page.goto(process.env.APP!);
    env = await createTestEnvironment();
  });

  afterAll(async () => {
    await expect(destroyTestEnvironment(env.id)).resolves.toBeTruthy();
  });

  it("setup test data", async () => {
    await $app.users.login(page, env);
    // open resource page & create resource
    await $app.pages.resource.open(page);
    await $app.resources.create(
      page,
      resourceA,
      "<rootDir>/../../resources/s001_kaya_rice_import.csv"
    );
    await $app.resources.create(
      page,
      resourceB,
      "<rootDir>/../../resources/s101_time_row.csv"
    );
    await expect($app.resources.size(page)).resolves.toEqual(2);

    // open layout page
    await $app.pages.layout.open(page);
  });

  it("create variable", async () => {
    await page.click($c.layout.createVariableBtn);

    // showing resource A: both resources and variables
    await $app.resources.waitForResourceToDisplay(page, resourceA);
    const x = await $app.resources.getDisplayingResources(page);
    expect(x).toEqual([resourceA]);
    await expect(
      $antd.staticSelect.get(page, $c.layout.upsert.resourceId)
    ).resolves.toEqual(resourceA);

    // change from resource-a to resource-b
    await $antd.staticSelect.select(
      page,
      $c.layout.upsert.resourceId,
      resourceB
    );
    await $app.resources.waitForResourceToDisplay(page, resourceB);
    await expect($app.resources.getDisplayingResources(page)).resolves.toEqual([
      resourceB
    ]);

    // set variable id
    await page.$eval($c.layout.upsert.variableId, (el: HTMLInputElement) => {
      el.value = "date";
    });

    // click submit will show an error because we haven't set the layout

    await page;
    await jestPuppeteer.debug();
  });
});
