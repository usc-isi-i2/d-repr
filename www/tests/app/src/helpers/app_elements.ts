export const $c = {
  menu: {
    dashboard: "[data-testid=top-menu] .ant-tabs-tab:nth-child(1)",
    resource: "[data-testid=top-menu] .ant-tabs-tab:nth-child(2)",
    layout: "[data-testid=top-menu] .ant-tabs-tab:nth-child(3)",
    mapping: "[data-testid=top-menu] .ant-tabs-tab:nth-child(4)",
    semanticModel: "[data-testid=top-menu] .ant-tabs-tab:nth-child(5)"
  },
  // everything about user page
  user: {
    login: {
      form: "[data-testid=user-login-form]",
      email: "[data-testid=user-login-form-email]",
      password: "[data-testid=user-login-form-password]",
      submitBtn: "[data-testid=user-login-form-login-button]"
    }
  },
  // everything about data display
  data: {
    matrix: {
      self: "[data-testid=data-matrix]",
      pagination: {
        self: "[data-testid=data-matrix] ul.ant-pagination",
        prevBtn:
          "[data-testid=data-matrix] ul.ant-pagination li.ant-pagination-prev",
        nextBtn:
          "[data-testid=data-matrix] ul.ant-pagination li.ant-pagination-next"
      }
    }
  },
  // everything about resource page
  resource: {
    page: "[data-testid=resource-page]",
    resourcePanel: (rid: string) => {
      return `[data-testid=resource-page] [data-testid=resource-component][data-testvalue=${rid}]`;
    },
    createResourceBtn: "[data-testid=create-resource-button]",
    minimizedList: {
      resources:
        "[data-testid=resource-page] [data-testid=resource-panels-minimized-resources]",
      displayingResources:
        "[data-testid=resource-page] .ant-btn-primary[data-testid=resource-panels-minimized-resources]",
      displayingResource: (rid: string) =>
        `[data-testid=resource-page] .ant-btn-primary[data-testid=resource-panels-minimized-resources][data-testvalue=${rid}]`,
      resource: (rid: string) =>
        `[data-testid=resource-page] [data-testid=resource-panels-minimized-resources][data-testvalue=${rid}]`
    },
    header: {
      all: "[data-testid=resource-page] [data-testid=resource-panel-header]",
      this: (rid: string) =>
        `[data-testid=resource-page] [data-testid=resource-panel-header][data-testvalue=${rid}]`,
      closeButton: (rid: string) =>
        `[data-testid=resource-page] [data-testid=resource-panel-header][data-testvalue=${rid}] [data-testid=resource-panel-header-close-button]`
    },
    upsert: {
      resourceId: "[data-testid=upsert-resource-form-resource-id]",
      // wrapper of ant.design form doesn't work
      resourceIdError:
        "//*[*/*/@data-testid='upsert-resource-form-resource-id']/*[contains(@class, 'ant-form-explain')]",
      resourceType: "[data-testid=upsert-resource-form-resource-type]",
      resourceTypeError:
        "//*[*/*/*/@data-testid='upsert-resource-form-resource-type']/*[contains(@class, 'ant-form-explain')]",
      resourceFile: "[data-testid=upsert-resource-form] input[type=file]",
      resourceURL: "[data-testid=upsert-resource-form-resource-url]",
      resourceURLError:
        "//*[*/*/@data-testid='upsert-resource-form-resource-url']/*[contains(@class, 'ant-form-explain')]",
      submitBtn: "[data-testid=upsert-resource-form-submit-button]"
    }
  },
  // everything in layout page
  layout: {
    page: "[data-testid=layout-page]",
    createVariableBtn: "[data-testid=create-variable-button]",
    resources: {
      resources:
        "[data-testid=layout-page] [data-testid=resource-panels-minimized-resources]",
      displayingResources:
        "[data-testid=layout-page] .ant-btn-primary[data-testid=resource-panels-minimized-resources]",
      displayingResource: (rid: string) =>
        `[data-testid=layout-page] .ant-btn-primary[data-testid=resource-panels-minimized-resources][data-testvalue=${rid}]`,
      resource: (rid: string) =>
        `[data-testid=layout-page] [data-testid=resource-panels-minimized-resources][data-testvalue=${rid}]`,
      header: {
        all: "[data-testid=layout-page] [data-testid=resource-panel-header]",
        this: (rid: string) =>
          `[data-testid=layout-page] [data-testid=resource-panel-header][data-testvalue=${rid}]`,
        closeButton: (rid: string) =>
          `[data-testid=layout-page] [data-testid=resource-panel-header][data-testvalue=${rid}] [data-testid=resource-panel-header-close-button]`
      }
    },
    upsert: {
      resourceId: "[data-testid=upsert-variable-form-resource-id]",
      variableId: "[data-testid=upsert-variable-form-variable-id]",
      variableLayout: "[data-testid=upsert-variable-form-variable-layout]",
      variableSorted: "[data-testid=upsert-variable-form-variable-sorted]",
      variableUnique: "[data-testid=upsert-variable-form-variable-unique]",
      variableValue: "[data-testid=upsert-variable-form-variable-value]",
      variableMissingValues:
        "[data-testid=upsert-variable-form-variable-missing-values]",
      deleteBtn: "[data-testid=upsert-variable-form-delete-btn]",
      cancelBtn: "[data-testid=upsert-variable-form-cancel-btn]",
      submitBtn: "[data-testid=upsert-variable-form-submit-btn]",
      collapse: "[data-testid=upsert-variable-form-collapse]"
    }
  }
};
