export class IDGenerator {
  private counter: number;

  constructor(start: number = 0) {
    this.counter = start;
  }

  public next(): number {
    return this.counter++;
  }
}
