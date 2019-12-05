from drepr import DRepr, NDArrayTables
from netCDF4 import Dataset
from raster import Raster


# drepr = DRepr.parse_from_file("")
# NDArrayTables.from_drepr()

curr_dir = "/home/rook/workspace/d-repr/pydrepr/examples/climate_data/"

infile = curr_dir + "gldas/2008/GLDAS_NOAH025_3H.A20080101.0000.021.nc4"
drepr_file = curr_dir + "gldas.yml"
# raster = Raster.from_netcdf4(infile, "Rainf_tavg")
# raster.to_geotiff(curr_dir + "Rainf_tavg.gdal.tif")
# infile = curr_dir + "flood_data.nc"
# raster = Raster.from_netcdf4(infile, "flood")

NDArrayTables.from_drepr(DRepr.parse_from_file(drepr_file), infile)