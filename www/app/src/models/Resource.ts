export const SUPPORT_RESOURCE_TYPES = {
  csv: "CSV format",
  json: "JSON format",
  netcdf4: "NetCDF4 format"
};

export class Resource {
  public resourceId: string;
  public resourceType: string;

  constructor(resourceID: string, resourceType: string) {
    this.resourceId = resourceID;
    this.resourceType = resourceType;
  }
}
