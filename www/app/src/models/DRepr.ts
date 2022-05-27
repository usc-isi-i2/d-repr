import { Resource, Variable, SemanticModel, Ontology, Alignment } from ".";
import * as _ from "lodash";
import { ValueAlignment, DimensionAlignment } from "./Alignment";

export class DRepr {
  public resources: Resource[];
  public transformation: any;
  public variables: Variable[];
  public mappings: Alignment[];
  public semanticModel: SemanticModel;
  private onts: { [prefix: string]: Ontology };

  constructor(
    resources: Resource[],
    variables: Variable[],
    mappings: Alignment[],
    sm: SemanticModel,
    onts: { [prefix: string]: Ontology }
  ) {
    this.resources = resources;
    this.variables = variables;
    this.mappings = mappings;
    this.semanticModel = sm;
    this.onts = onts;
  }

  public serialize() {
    const resources = {};
    for (const r of this.resources) {
      resources[r.resourceId] = r.resourceType;
    }
    const layout = {};
    for (const v of this.variables) {
      layout[v.id] = {
        location: v.location.toString(false)
      };
    }
    // // create semantic model
    // const semanticTypes = {};
    // for (const n of this.semanticModel.iterDataNodes()) {
    //   const stype = this.semanticModel.getDataNodeSemanticType(n.id);
    //   if (stype !== undefined) {
    //     semanticTypes[n.id] = `${stype.ontClass!.serialize2str()}--${
    //       stype.ontPredicate
    //     }`;
    //   }
    // }
    // const semanticRelations = [];
    // for (const [s, p, o] of this.semanticModel.iterTriples()) {
    //   if (o.isClassNode()) {
    //     semanticRelations.push(
    //       `${s.classId!.serialize2str()}--${p}--${o.classId!.serialize2str()}`
    //     );
    //   }
    // }

    // create mappings
    const mappings = _.map(this.mappings, m => {
      const o: any = m.serialize();
      if (m instanceof ValueAlignment) {
        o.type = "value_mapping";
      } else if (m instanceof DimensionAlignment) {
        o.type = "dimension_mapping";
      } else {
        throw new Error("Congrat! You found a bug");
      }

      return o;
    });

    return {
      resources,
      transformation: [],
      layout,
      mappings,
      semantic_model: this.semanticModel.serialize(this.onts)
    };
  }
}
