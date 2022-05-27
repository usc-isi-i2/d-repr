export class BugError extends Error {
  constructor(msg: string) {
    super(`Congrat! You found a bug! ${msg}`);
    this.name = "Bug";
  }
}
