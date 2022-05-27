import { Resource, data } from "src/models";

export class ResourceRecord {
  public resource: Resource;
  public data: data.NDimData;

  constructor(resource: Resource, data2: data.NDimData) {
    this.resource = resource;
    this.data = data2;
  }
}
