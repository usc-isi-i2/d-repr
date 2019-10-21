from drepr.models import DRepr, ResourceType, yaml


def map_ndarray_cf_convention(ds_model: DRepr, resource_file: str):
    """
    Map high-dimensional single resource datasets to NDArray format
    """
    if ds_model.resources[0].type == ResourceType.GeoTIFF:
        return map_geotiff(ds_model, resource_file)
    elif ds_model.resources[0].type == ResourceType.NetCDF4:
        return map_netcdf(ds_model, resource_file, "4")
    # elif ds_model.resources[0].type == ResourceType.NetCDF3:
    #     return map_netcdf(ds_model, resource_file, "3")


def map_geotiff(ds_model: DRepr, resource_file: str):
    pass


def map_netcdf(ds_model: DRepr, resource_file: str, version: str):
    pass


if __name__ == '__main__':
    resource_file = "C:\\Users\\nancy\\Downloads\\test.tiff"
    from PIL import Image
    from PIL.TiffTags import TAGS
    with Image.open(resource_file) as img:
        print(img)
        print(img.tag)
        meta_dict = {TAGS[key]: img.tag[key] for key in img.tag.keys()}
        print("hello world")
    ds_model = """
version: '1'
resources: geotiff
attributes:
    band1: 
    """
    map_ndarray_cf_convention(DRepr.parse(yaml.load(ds_model)), resource_file)