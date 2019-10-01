export class Ontology {
  private _prefix: string;
  private _ns: string;

  constructor(prefix: string, namespace: string) {
    this._prefix = prefix;
    this._ns = namespace;
  }

  get prefix() {
    return this._prefix;
  }

  get namespace() {
    return this._ns;
  }
}
