from tests.drepr.conftest import DSInput


def test_mapping_graph(ds_input: DSInput):
    mg = ds_input.get_repr().get_mapping_graph()
    for m in ds_input.get_supplementary_mapping_funcs():
        assert mg.get_mapping_func(m.get_source_variable_id(), m.get_target_variable_id()) == m
