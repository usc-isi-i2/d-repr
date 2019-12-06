from drepr import DRepr, NDArrayGraph
from netCDF4 import Dataset
from raster import Raster, GeoTransform
import numpy as np
# def map_()


# drepr = DRepr.parse_from_file("")
# NDArrayTables.from_drepr()

curr_dir = "./"

infile = curr_dir + "gldas/2008/GLDAS_NOAH025_3H.A20080101.0000.021.nc4"
drepr_file = curr_dir + "gldas.yml"
raster = Raster.from_netcdf4(infile, "Rainf_f_tavg")
# raster = Raster.from_geotiff("NETCDF:{0}:{1}".format(infile, "Rainf_f_tavg"))
# print(raster.geotransform)
# raster.data = raster.data * 10
# raster.data[raster.data >= 0] = 1
# raster.data = np.ones(raster.data.shape, dtype=np.uint8)
# print(raster.data.shape)
raster.to_geotiff(curr_dir + "Rainf_f_tavg_gdal.tif")

# exit(0)
# raster = Raster.from_geotiff(curr_dir + "world.tif")
# print("+++", raster.geotransform)
# print("+++", raster.raster.GetGeoTransform())
# raster.to_geotiff(curr_dir + "world_tmp.tif")
# # # print(raster.data.shape, raster.raster.ReadAsArray().shape)
# # assert np.allclose(raster.data, raster.raster.ReadAsArray())
#
# raster2 = Raster.from_geotiff(curr_dir + "world_tmp.tif")
# assert np.allclose(raster.data, raster2.data)

# infile = curr_dir + "flood_data.nc"
# raster = Raster.from_netcdf4(infile, "flood")
# raster.to_geotiff(curr_dir + "flood.tif")
#
ndarray = NDArrayGraph.from_drepr(DRepr.parse_from_file(drepr_file), infile)
class_id = next(ndarray.iter_class_ids("mint:Variable"))

class_info = ndarray.sm.class2dict(class_id)
# print(class_info)

edge_data = ndarray.edge_data_as_ndarray(class_info['rdf:value'],
                                         [class_info['mint-geo:lat'], class_info['mint-geo:long']])

# print(edge_data.data.shape, edge_data.index_edges)
geo2d = ndarray._deprecated_get1rowtbl("mint-geo:Geo2D")
gt = GeoTransform(geo2d['mint:long_min'], geo2d['mint:long_delta'], geo2d['mint:long_angle'], geo2d['mint:lat_min'],
                  geo2d['mint:lat_delta'], geo2d['mint:lat_angle'])

print(raster.data.shape, edge_data.data.shape)
assert np.allclose(edge_data.data, raster.data)
raster2 = Raster(edge_data.data, gt, geo2d['mint:epsg'],
                edge_data.nodata.value if edge_data.nodata is not None else None)
raster2.to_geotiff(curr_dir + "Rainf_tavg_drepr.tif")
print('>>> done')
