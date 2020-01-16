from drepr.executors.readers.geotiff import GeoTIFFReader
from drepr.executors.readers.netcdf import NetCDF4Reader
from drepr.executors.readers.np_dict import NPDictReader
from drepr.models import DRepr, ResourceType, RangeAlignment, PMap
import numpy as np

from drepr.executors.preprocessing.context import Context
from drepr.executors.preprocessing.py_exec import PyExec
from drepr.executors.readers.netcdf import NetCDF4Reader
from drepr.models import DRepr, ResourceType, PreprocessingType, IndexExpr, ClassNode, DataNode, LiteralNode, RangeExpr
from drepr.outputs.array_backend.array_attr import ArrayAttr, ScalarAttr, NoData


class CFConventionNDArrayMap:

    @classmethod
    def analyze(cls, ds_model: DRepr) -> bool:
        # only one resource
        if len(ds_model.resources) > 1:
            return False

        # ensure it is geotiff, netcdf
        if ds_model.resources[0].type not in {ResourceType.GeoTIFF, ResourceType.NetCDF3, ResourceType.NetCDF4, ResourceType.NPDict}:
            return False

        # only have dimension alignment
        if not all(isinstance(align, RangeAlignment) for align in ds_model.aligns):
            return False

        # only have map preprocessing, which mutate the current data
        for prepro in ds_model.preprocessing:
            if not isinstance(prepro.value, PMap) \
                    or prepro.value.output is not None \
                    or prepro.value.change_structure:
                return False

        return True

    @classmethod
    def execute(cls, ds_model: DRepr, resource_file: str):
        """
        Execution steps:

        1. create resource reader (only support one resource)
        2. execute preprocessing function (which is map function and only mutate the value)
        3. gather attributes
        4. Create table:
            a. If there are duplicated predicates (duplicated column_id), then the resulted array will have an extra dimension at
               the end, which size is equal to the number of duplicated columns of `column_id`. (see the ndarray documentation)
        """
        # 1st: create resource reader
        resource = ds_model.resources[0]
        if resource.type == ResourceType.NetCDF4:
            reader = NetCDF4Reader.from_file(resource_file)
        elif resource.type == ResourceType.GeoTIFF:
            reader = GeoTIFFReader.from_file(resource_file)
        elif resource.type == ResourceType.NPDict:
            reader = NPDictReader.from_file(resource_file)
        else:
            raise NotImplementedError()

        # 2nd: execute preprocessing function
        context = Context(reader)
        for preprocess_fn in ds_model.preprocessing:
            if preprocess_fn.type == PreprocessingType.pmap:
                steps = preprocess_fn.value.path.steps
                assert all(isinstance(step, IndexExpr) for step in steps), "Range selection should use numpy map"

                index = [step.val for step in steps]
                fn = PyExec.compile(preprocess_fn.value.code)
                value = fn.exec(reader.get_value(index), index, context)
                reader.set_value(index, value)
            else:
                raise Exception("You found a bug")

        # 3rd: gather attributes
        attrs = {}
        new_attrs = {}
        for attr in ds_model.attrs:
            attrs[attr.id] = reader.select(attr.path.steps)
            if isinstance(attrs[attr.id], np.ndarray):
                step2dim = []
                _attr = ds_model.get_attr_by_id(attr.id)
                path = _attr.path
                if len(_attr.missing_values) == 0:
                    nodata = None
                elif len(_attr.missing_values) == 1:
                    nodata = NoData(_attr.missing_values[0])
                else:
                    # need to convert other values back to just one value and use it!
                    raise NotImplementedError()

                count = 0
                for step in path.steps:
                    if not isinstance(step, IndexExpr):
                        # in this function all the shape of ndarray will be determined by range only,
                        # and they will follow the order. Will we have other cases?
                        assert isinstance(step, RangeExpr)
                        step2dim.append(count)
                        count += 1
                    else:
                        step2dim.append(None)
                new_attrs[f"dnode:{attr.id}"] = ArrayAttr(f"dnode:{attr.id}", attrs[attr.id], path, step2dim, nodata)
            else:
                new_attrs[f"dnode:{attr.id}"] = ScalarAttr(f"dnode:{attr.id}", attrs[attr.id])
            # assert all(isinstance(step, IndexExpr) for step in attr.path.steps)
            # index = [step.val for step in steps]
            # attrs[attr.id] = reader.get_value(index)

        # 4th: create tables from the semantic model
        sm = ds_model.sm
        tables = {}
        alignments = {}

        for nid, node in sm.nodes.items():
            if isinstance(node, ClassNode):
                outgoing_edges = list(sm.iter_outgoing_edges(nid))
                tables[nid] = {}
                for e in outgoing_edges:
                    c = sm.nodes[e.target_id]
                    if isinstance(c, DataNode):
                        if isinstance(attrs[c.attr_id], np.ndarray):
                            step2dim = []
                            _attr = ds_model.get_attr_by_id(c.attr_id)
                            path = _attr.path
                            if len(_attr.missing_values) == 0:
                                nodata = None
                            elif len(_attr.missing_values) == 1:
                                nodata = NoData(_attr.missing_values[0])
                            else:
                                # need to convert other values back to just one value and use it!
                                raise NotImplementedError()

                            count = 0
                            for step in path.steps:
                                if not isinstance(step, IndexExpr):
                                    # in this function all the shape of ndarray will be determined by range only,
                                    # and they will follow the order. Will we have other cases?
                                    assert isinstance(step, RangeExpr)
                                    step2dim.append(count)
                                    count += 1
                                else:
                                    step2dim.append(None)
                            # tables[nid][e.target_id] = ColArray(e.target_id, attrs[c.attr_id], path, step2dim, nodata)
                        else:
                            pass
                            # tables[nid][e.target_id] = ColSingle(attrs[c.attr_id])
                    elif isinstance(c, LiteralNode):
                        pass
                        # tables[nid][e.target_id] = ColSingle(c.value)
                    elif not isinstance(c, ClassNode):
                        raise NotImplementedError()
            elif isinstance(node, LiteralNode):
                new_attrs[nid] = ScalarAttr(nid, node.value)

        # inferring alignments
        for align in ds_model.aligns:
            source = f"dnode:" + align.source
            target = f"dnode:" + align.target
            if source not in alignments:
                alignments[source] = {}
            alignments[source][target] = align
        return None, new_attrs
        # return ndarray.NDArrayGraph(sm, tables, alignments), new_attrs
