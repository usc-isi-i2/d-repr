import { Page } from "puppeteer";

export const $antd = {
  modal: {
    yes: async (p: Page) => {
      return await $antd.modal.clickBtn(p, "Yes");
    },
    no: async (p: Page) => {
      return await $antd.modal.clickBtn(p, "No");
    },
    clickBtn: async (p: Page, btnLabel: string) => {
      await $antd.modal.waitUntilAppear(p);
      const btns = await p.$x(
        `//*[contains(@class, 'ant-modal')]//*[contains(@class, 'ant-modal-confirm-btns')]//*[contains(text(), '${btnLabel}')]`
      );
      expect(btns.length).toEqual(1);
      await btns[0].click();
      return true;
    },
    waitUntilAppear: async (p: Page) => {
      await p.waitForSelector("div.ant-modal-confirm-btns", { visible: true });
      await p.waitForFunction(
        (selector: string) => {
          const el = document.querySelector(selector) as HTMLElement;
          const isVisible = !!(
            el.offsetWidth ||
            el.offsetHeight ||
            el.getClientRects().length
          );
          return isVisible;
        },
        {},
        "div.ant-modal-confirm-btns"
      );
      // wait for 100ms, sometime puppeteer can click the element, but doesn't actually trigger the click
      await p.waitFor(200);
    },
    waitUntilDisappear: async (p: Page) => {
      await p.waitForSelector("div.ant-modal", { hidden: true });
      return true;
    }
  },
  staticSelect: {
    get: async (p: Page, selector: string) => {
      return await p.$eval(
        `${selector} .ant-select-selection-selected-value`,
        el => el.textContent
      );
    },
    select: async (p: Page, selector: string, value: string) => {
      await p.click(selector);
      await p.click(
        `li[data-testid=antd-select-options][data-testvalue="${value}"]`
      );
      return true;
    }
  },
  input: {
    clear: async (p: Page, selector: string) => {
      await p.click(selector);
      await p.click(selector, { clickCount: 3 });
      await p.keyboard.press("Backspace");
    }
  },
  table: {
    nextPage: async (p: Page, tblSelector: string): Promise<boolean> => {
      // forward to next page and return true if we can
      return await $antd.table.clickNavBtn(
        p,
        `${tblSelector} ul.ant-pagination li.ant-pagination-next`
      );
    },
    prevPage: async (p: Page, tblSelector: string): Promise<boolean> => {
      // backward to previous page and return true if we can
      return await $antd.table.clickNavBtn(
        p,
        `${tblSelector} ul.ant-pagination li.ant-pagination-prev`
      );
    },
    clickNavBtn: async (p: Page, btnSelector: string): Promise<boolean> => {
      const isDisabled = await p.$eval(btnSelector, (el: HTMLElement) => {
        return el.getAttribute("aria-disabled") === "true";
      });

      if (isDisabled) {
        return false;
      }

      await p.click(btnSelector);
      return true;
    },
    moveNextNPage: async (
      p: Page,
      tblSelector: string,
      n: number
    ): Promise<boolean> => {
      const currentItem = `${tblSelector} ul.ant-pagination li.ant-pagination-item-active`;
      const pageNo = await p.$eval(currentItem, (el: HTMLElement) => {
        return parseInt(el.getAttribute("title")!, 10);
      });

      const nextItem = `${tblSelector} ul.ant-pagination li.ant-pagination-item-${pageNo +
        n}`;
      const nextSelector = await page.$(nextItem);

      if (nextSelector === null) {
        return false;
      }

      await page.click(nextItem);
      return true;
    },
    goToPage: async (
      p: Page,
      tblSelector: string,
      pageNo: number,
      waitForPage: boolean = false
    ): Promise<boolean> => {
      const pageSelector = `${tblSelector} ul.ant-pagination li.ant-pagination-item-${pageNo}`;
      const pageElement = await (waitForPage
        ? page.waitForSelector(pageSelector)
        : page.$(pageSelector));

      if (pageElement === null) {
        return false;
      }

      await page.click(pageSelector);
      return true;
    }
  }
};
