from drepr import DRepr, NDArrayGraph
from netCDF4 import Dataset
from raster import Raster, GeoTransform, EPSG, BoundingBox, ReSample
import numpy as np
from netCDF4 import Dataset

curr_dir = "./"

infile = curr_dir + "GLDAS_NOAH025_3H.A20080101.0000.021.nc4"
varname = "Rainf_f_tavg"
drepr_file = curr_dir + "gldas.yml"
# infile = curr_dir + "3B-MO.MS.MRG.3IMERG.20080101-S000000-E235959.01.V06B.HDF5.nc4"
# varname = "precipitation"
# drepr_file = curr_dir + "gpm.yml"

ndarray = NDArrayGraph.from_drepr(DRepr.parse_from_file(drepr_file), infile)
class_id = next(ndarray.iter_class_ids("mint:Variable"))

class_info = ndarray.sm.class2dict(class_id)
edge_data = ndarray.edge_data_as_ndarray(class_info['rdf:value'],
                                         [class_info['mint-geo:lat'], class_info['mint-geo:long']])
edge_data.data = edge_data.data[::-1]   # create north-up images
geo2d = ndarray._deprecated_get1rowtbl("mint-geo:Raster")
gt = GeoTransform(x_min=geo2d['mint-geo:x_min'],
                  y_max=geo2d['mint-geo:y_min'] + geo2d['mint-geo:dy'] * edge_data.data.shape[0],
                  dx=geo2d['mint-geo:dx'], dy=-geo2d['mint-geo:dy'])

raster2 = Raster(edge_data.data, gt, int(geo2d['mint-geo:epsg']),
                edge_data.nodata.value if edge_data.nodata is not None else None)
raster2.to_geotiff(curr_dir + f"{varname}.drepr.tif")
print('>>> done')
