import { Page } from "puppeteer";
import { $c } from "./app_elements";
import { $antd } from "./antd_helpers";
import { $api } from "./api_helpers";
import { TestEnvironment } from "./env";

export const $app = {
  pages: {
    resource: {
      open: async (p: Page) => {
        return await $app.pages.openPage(p, $c.menu.resource, $c.resource.page);
      },
      isOpened: async (p: Page) => {
        return await $app.pages.isPageOpened(p, $c.resource.page);
      }
    },
    layout: {
      open: async (p: Page) => {
        return await $app.pages.openPage(p, $c.menu.layout, $c.layout.page);
      },
      isOpened: async (p: Page) => {
        return await $app.pages.isPageOpened(p, $c.layout.page);
      }
    },
    openPage: async (
      p: Page,
      pageAnchorSelector: string,
      pageSelector: string
    ) => {
      // only open if is not inside this page
      if (!(await $app.pages.isPageOpened(p, pageSelector))) {
        await p.click(pageAnchorSelector);
        await p.waitForSelector(pageSelector);
      }

      return true;
    },
    isPageOpened: async (p: Page, pageSelector: string) => {
      try {
        return await p.$eval(pageSelector, (el: HTMLElement) => {
          return !!(
            el.offsetWidth ||
            el.offsetHeight ||
            el.getClientRects().length
          );
        });
      } catch (e) {
        if (e.message.indexOf("failed to find element matching selector")) {
          return false;
        }
        throw e;
      }
    }
  },
  users: {
    login: async (p: Page, env: TestEnvironment) => {
      await expect(p.$($c.user.login.form)).not.toBeNull();
      await p.type($c.user.login.email, env.username);
      await p.type($c.user.login.password, env.password);
      await p.click($c.user.login.submitBtn);
      await p.waitForSelector($c.user.login.form, {
        hidden: true,
        timeout: 3000
      });

      return true;
    }
  },
  resources: {
    size: async (p: Page) => {
      // return number of resources
      await expect($app.pages.resource.isOpened(page)).resolves.toBeTruthy();
      const elems = await p.$$($c.resource.minimizedList.resources);
      return elems.length;
    },
    waitForResourceToDisplay: async (p: Page, rid: string) => {
      // wait for a resource to display
      let rpath;
      let rhpath;

      if (await $app.pages.resource.isOpened(p)) {
        rpath = $c.resource.minimizedList.displayingResource(rid);
        rhpath = $c.resource.header.this(rid);
      } else if (await $app.pages.layout.isOpened(p)) {
        rpath = $c.layout.resources.displayingResource(rid);
        rhpath = $c.layout.resources.header.this(rid);
      } else {
        throw new Error("Layout or resources page need to be opened");
      }

      await p.waitForSelector(rpath);
      await p.waitForSelector(rhpath);
    },
    getDisplayingResources: async (p: Page) => {
      // return list of resources are showing on the page
      let drpath;
      let drhpath;

      if (await $app.pages.resource.isOpened(p)) {
        // resource page
        drpath = $c.resource.minimizedList.displayingResources;
        drhpath = $c.resource.header.all;
      } else if (await $app.pages.layout.isOpened(p)) {
        // layout page
        drpath = $c.layout.resources.displayingResources;
        drhpath = $c.layout.resources.header.all;
      } else {
        throw new Error(
          "Cannot get displaying resources when not in layout or resources page"
        );
      }

      const displayingResources = await p.$$eval(drpath, resources =>
        resources.map((r: HTMLElement) => r.dataset.testvalue)
      );

      // make sure the displaying and the list are consistent
      const displayingResourceHeaders = await p.$$eval(
        drhpath,
        headers => headers.length
      );
      expect(displayingResourceHeaders).toEqual(displayingResources.length);

      return displayingResources;
    },
    delete: async (p: Page, resourceId: string) => {
      // must be in resource page
      await expect($app.pages.resource.isOpened(page)).resolves.toBeTruthy();

      // view the created resource
      await page.click($c.resource.minimizedList.resource(resourceId));
      await page.waitForSelector($c.resource.header.this(resourceId));

      // click and confirm
      await page.click($c.resource.header.closeButton(resourceId));
      // wait for request to finish
      await Promise.all([
        $api.resource.waitForDeletion(p),
        $antd.modal.yes(page)
      ]);
      await $antd.modal.waitUntilDisappear(p);
      return true;
    },
    create: async (p: Page, resourceId: string, resourceFile: string) => {
      // must be in resource page
      await expect($app.pages.resource.isOpened(page)).resolves.toBeTruthy();

      let resourceType;
      if (resourceFile.endsWith(".csv")) {
        resourceType = "csv";
      } else {
        throw new Error("Cannot infer resource type");
      }

      await p.click($c.resource.createResourceBtn);
      await p.waitForSelector($c.resource.upsert.resourceId);
      await p.$eval(
        $c.resource.upsert.resourceId,
        (el: HTMLInputElement) => (el.value = "")
      );
      await p.type($c.resource.upsert.resourceId, resourceId);
      await $antd.staticSelect.select(
        p,
        $c.resource.upsert.resourceType,
        resourceType
      );
      const uploadInput = await p.$($c.resource.upsert.resourceFile);
      expect(uploadInput).not.toBeNull();
      await uploadInput!.uploadFile(resourceFile);

      await Promise.all([
        $api.resource.waitForCreation(p, true),
        p.click($c.resource.upsert.submitBtn)
      ]);

      return true;
    },
    openResource: async (p: Page, rid: string) => {
      // must be in resource or layout page
      let resourceElement;
      if (await $app.pages.resource.isOpened(p)) {
        resourceElement = $c.resource.minimizedList.resource(rid);
      } else if (await $app.pages.layout.isOpened(p)) {
        resourceElement = $c.layout.resources.resource(rid);
      } else {
        throw new Error("Layout or resources page need to be opened");
      }

      await page.click(resourceElement);
      await $app.resources.waitForResourceToDisplay(p, rid);
    }
  }
};
