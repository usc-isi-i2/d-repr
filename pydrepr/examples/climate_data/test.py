from drepr import DRepr, NDArrayGraph
from netCDF4 import Dataset
from raster import Raster


# drepr = DRepr.parse_from_file("")
# NDArrayTables.from_drepr()

curr_dir = "./"

infile = curr_dir + "gldas/2008/GLDAS_NOAH025_3H.A20080101.0000.021.nc4"
drepr_file = curr_dir + "gldas.yml"
# raster = Raster.from_netcdf4(infile, "Rainf_tavg")
# raster.to_geotiff(curr_dir + "Rainf_tavg.gdal.tif")
# infile = curr_dir + "flood_data.nc"
# raster = Raster.from_netcdf4(infile, "flood")

ndarray = NDArrayGraph.from_drepr(DRepr.parse_from_file(drepr_file), infile)
class_id = next(ndarray.iter_class_ids("mint:Variable"))
variable = ndarray.get_property_as_ndarray(class_id, "rdf:value", ["mint:Variable:1:mint-geo:lat", "mint-geo:long"])
print('>>> done')