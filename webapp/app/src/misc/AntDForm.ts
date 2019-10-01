export type AntDFormValidationStatus =
  | "error"
  | "success"
  | "warning"
  | "validating";

export class AntDFormField<T> {
  public readonly validationStatus: AntDFormValidationStatus;
  public readonly validationMessage: string;
  public readonly value: T;

  constructor(
    value: T,
    validationStatus: AntDFormValidationStatus = "success",
    validationMessage: string = ""
  ) {
    this.value = value;
    this.validationStatus = validationStatus;
    this.validationMessage = validationMessage;
  }

  public isValid() {
    return this.validationStatus === "success";
  }
}
