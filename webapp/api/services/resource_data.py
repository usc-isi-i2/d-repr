#!/usr/bin/python
# -*- coding: utf-8 -*-

from api.models.resource import Resource


class ResourceDataService:

    instance = None

    def __init__(self):
        pass

    @staticmethod
    def get_instance():
        if ResourceDataService.instance is None:
            ResourceDataService.instance = ResourceDataService()
        return ResourceDataService.instance

    # noinspection PyMethodMayBeStatic
    def get_resource_data(self, resource: Resource, slices):
        data, nslice = get_data_real(resource.get_data(), slices)
        return data, nslice


def get_data_real(data, slices):
    if slices['type'] == 'range':
        n_data = len(data)

        nv_data = []
        ns_range = [slices['range'][0]]
        ns_value = []

        for i in range(len(slices['values'])):
            start = slices['range'][i]
            end = slices['range'][i + 1]
            value = slices['values'][i]

            if value is None:
                nv_data.extend(data[start:end])
                ns_value.append(value)
            else:
                ns = None
                for v in data[start:end]:
                    nv, ns = get_data_real(v, value)
                    nv_data.append(nv)
                ns_value.append(ns)

            ns_range.append(min(n_data, end))
            if end >= n_data:
                break

        return nv_data, {'type': 'range', "range": ns_range, "values": ns_value}
    else:
        index2slice = []
        new_slice = {'type': 'index', 'index2slice': index2slice}
        obj = {}
        for k, v in slices['index2slice']:
            if v is None:
                obj[k] = data[k]
                index2slice.append((k, None))
            else:
                nv, ns = get_data_real(data[k], v)
                obj[k] = nv
                index2slice.append((k, ns))

        return obj, new_slice
