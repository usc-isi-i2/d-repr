export default class UIConfiguration {
  public displayMax1Resource: boolean;

  constructor(displayMax1Resource: boolean) {
    this.displayMax1Resource = displayMax1Resource;
  }

  public setDisplayMax1Resource(displayMax1Resource: boolean) {
    const instance = this.shallowClone();
    instance.displayMax1Resource = displayMax1Resource;
    return instance;
  }

  private clone() {
    return new UIConfiguration(this.displayMax1Resource);
  }

  private shallowClone() {
    return new UIConfiguration(this.displayMax1Resource);
  }
}
