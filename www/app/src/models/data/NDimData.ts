import { PortionOfData, Dimension } from ".";

export class NDimData {
  public id: string;
  public dimension: Dimension;
  public pod: PortionOfData;

  constructor(id: string, dimension: Dimension, data: PortionOfData) {
    this.id = id;
    this.pod = data;
    this.dimension = dimension;
  }
}
