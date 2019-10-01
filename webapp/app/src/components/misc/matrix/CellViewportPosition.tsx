export class CellViewPortPosition {
  // (row, col) index in the viewport. E.g: (0, 0): top-left, (0, y > 0) mean top-right
  public vpIndex: [number, number];
  // size of the viewport (|rows|, |cols|)
  public vpSize: [number, number];

  constructor(vpIndex: [number, number], vpSize: [number, number]) {
    this.vpIndex = vpIndex;
    this.vpSize = vpSize;
  }

  // true means a cell is in the border line, otherwise it is inside
  public isInBorder(): boolean {
    return (
      this.vpIndex[0] === 0 ||
      this.vpIndex[1] === 0 ||
      this.vpIndex[0] === this.vpSize[0] - 1 ||
      this.vpIndex[1] === this.vpSize[1] - 1
    );
  }

  public isInRightBorder(): boolean {
    return this.vpIndex[1] === this.vpSize[1] - 1;
  }

  public isInLeftBorder(): boolean {
    return this.vpIndex[1] === 0;
  }

  public isInTopBorder(): boolean {
    return this.vpIndex[0] === 0;
  }

  public isInBottomBorder(): boolean {
    return this.vpIndex[0] === this.vpSize[0] - 1;
  }
}
