import {
  createTestEnvironment,
  destroyTestEnvironment,
  TestEnvironment
} from "../helpers/env";
import * as _ from "lodash";
import { $app } from "../helpers/app_helpers";
import { $antd } from "../helpers/antd_helpers";
import { $c } from "../helpers/app_elements";

describe("test navigate csv resource", () => {
  let env: TestEnvironment;
  const resources = {
    "s101_time_row.csv": 42
  };

  beforeEach(async () => {
    jest.setTimeout(100000);
    await page.goto(process.env.APP!);
    env = await createTestEnvironment();

    // prepare scenario
    await $app.users.login(page, env);
    await $app.pages.resource.open(page);

    for (const [i, name] of Object.keys(resources).entries()) {
      await expect(
        $app.resources.create(
          page,
          name.split(".")[0],
          `<rootDir>/../../resources/${name}`
        )
      ).resolves.toBeTruthy();
    }

    await expect($app.resources.size(page)).resolves.toBe(_.size(resources));
  });

  afterEach(async () => {
    await expect(destroyTestEnvironment(env.id)).resolves.toBeTruthy();
  });

  it("sequential-pattern", async () => {
    for (const name of Object.keys(resources)) {
      const resourceId = name.split(".")[0];
      const resourceSelector = $c.resource.header;

      // open a resource
      await $app.resources.openResource(page, resourceId);
      const $tblSelector = `${$c.resource.resourcePanel(resourceId)} ${
        $c.data.matrix.self
      }`;

      // test navigate sequentially
      let canMove;
      let maxOps = 1000;
      for (let i = 0; i < 2; i++) {
        do {
          canMove = await $antd.table.nextPage(page, $tblSelector);
        } while (canMove && maxOps-- > 0);
        do {
          canMove = await $antd.table.prevPage(page, $tblSelector);
        } while (canMove && maxOps-- > 0);
      }

      expect(maxOps).toBeGreaterThan(0);
    }
  });

  it("custom-pattern", async () => {
    const resourceId = "s101_time_row";
    await $app.resources.openResource(page, resourceId);
    const $tblSelector = `${$c.resource.resourcePanel(resourceId)} ${
      $c.data.matrix.self
    }`;

    for (const pageNo of [1, 3, 2, 5, 4, 3, 2, 1, 5, 4, 5, 4, 5, 1]) {
      await expect(
        $antd.table.goToPage(page, $tblSelector, pageNo, true)
      ).resolves.toBeTruthy();
      await page.waitFor(100);
    }
  });
});
