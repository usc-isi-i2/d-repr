import { Index } from "./data";
import * as _ from "lodash";

export abstract class Alignment {
  public static deserialize(o: any): Alignment {
    if (o.type === "dimension") {
      return new DimensionAlignment(
        o.source,
        o.target,
        _.map(o.aligned_dims, m => ({
          source: m.source,
          target: m.target
        }))
      );
    } else {
      return new ValueAlignment(o.source, o.target);
    }
  }

  public source: string;
  public target: string;

  abstract get id(): string;

  constructor(source: string, target: string) {
    this.source = source;
    this.target = target;
  }

  public abstract isEqual(another: Alignment): boolean;
  public abstract serialize(): any;
}

export type AlignmentImpl = ValueAlignment | DimensionAlignment;

export class ValueAlignment extends Alignment {
  get id() {
    return `value:${this.source}---${this.target}`;
  }

  public isEqual(m: Alignment): boolean {
    if (!(m instanceof ValueAlignment) || m === null) {
      return false;
    }

    return m.source === this.source && m.target === this.target;
  }

  public serialize() {
    return {
      type: "value",
      source: this.source,
      target: this.target
    };
  }
}

export class DimensionAlignment extends Alignment {
  public alignedDimensions: Array<{ source: number; target: number }>;

  get id() {
    return `index:${this.source}---${this.target}---${_.map(
      this.alignedDimensions,
      m => `${m.source}-${m.target}`
    ).join(",")}`;
  }

  constructor(
    var1id: string,
    var2id: string,
    alignedDimensions: Array<{ source: number; target: number }>
  ) {
    super(var1id, var2id);
    this.alignedDimensions = alignedDimensions;
  }

  public addAlignedDimension(source: number, target: number) {
    this.alignedDimensions.push({ source, target });
  }

  public isEqual(m: Alignment): boolean {
    if (!(m instanceof DimensionAlignment) || m === null) {
      return false;
    }

    return (
      m.source === this.source &&
      m.target === this.target &&
      _.isEqual(this.alignedDimensions, m.alignedDimensions)
    );
  }

  public serialize() {
    return {
      type: "dimension",
      source: this.source,
      target: this.target,
      aligned_dims: _.map(this.alignedDimensions, m => ({
        source: m.source,
        target: m.target
      }))
    };
  }
}
