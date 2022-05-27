export class ClassId {
  public static deserialize4str(s: string): ClassId {
    const idx = s.lastIndexOf(":");
    return new ClassId(s.substr(0, idx), parseInt(s.substr(idx + 1), 10));
  }

  private _shortURI: string;
  private _index: number;

  constructor(shortURI: string, index: number) {
    this._shortURI = shortURI;
    this._index = index;
  }

  get shortURI() {
    return this._shortURI;
  }

  get index() {
    return this._index;
  }

  get label() {
    return `${this._shortURI}${this._index}`;
  }

  get id() {
    return `${this._shortURI}:${this._index}`;
  }

  public serialize2str(): string {
    return `${this._shortURI}:${this._index}`;
  }

  public getNamespacePrefix(): string {
    return this.shortURI.substr(0, this.shortURI.indexOf(":"));
  }

  // create next class id
  public next(): ClassId {
    return new ClassId(this._shortURI, this._index + 1);
  }
}
