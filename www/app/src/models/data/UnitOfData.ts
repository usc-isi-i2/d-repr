import { Index } from ".";

export class UnitOfData {
  // present a primitive value or a nested object that we cannot divide any further
  public indice: Index[];
  public value: any;

  public constructor(indice: Index[], value: any) {
    this.indice = indice;
    this.value = value;
  }
}
