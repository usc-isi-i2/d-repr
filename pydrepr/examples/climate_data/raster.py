#!/usr/bin/python
# -*- coding: utf-8 -*-
import datetime
import os

import numpy as np
from pathlib import Path
from osgeo import gdal, osr, gdal_array
from typing import Tuple, Union
from enum import Enum, IntEnum
from dataclasses import dataclass, astuple
from netCDF4 import Dataset


@dataclass
class GeoTransform:
    # x = longitude, y = latitude
    # need to keep the order match with gdal: x_min,
    x_min: float = 0.0
    x_res: float = 1.0
    x_angle: float = 0.0
    y_min: float = 0.0
    y_angle: float = 0.0
    y_res: float = 1.0

    @staticmethod
    def from_gdal(t):
        return GeoTransform(x_min=t[0], x_res=t[1], x_angle=t[2], y_min=t[3], y_angle=t[4], y_res=t[5])

    def to_gdal(self):
        return self.x_min, self.x_res, self.x_angle, self.y_min, self.y_angle, self.y_res


class EPSG(IntEnum):
    WGS_84 = 4326
    WGS_84_PSEUDO_MERCATOR = 3857


@dataclass
class BoundingBox:
    # x = longitude, y = latitude
    x_min: float
    y_min: float
    x_max: float
    y_max: float

    def to_gdal(self) -> Tuple[float, float, float, float]:
        # min-long, min-lat, max-long, max-lat
        return self.x_min, self.y_min, self.x_max, self.y_max


class ReSample(Enum):
    NEAREST_NEIGHBOUR = 'nearest'
    BILINEAR = 'bilinear'


class Raster:
    def __init__(self, array: np.ndarray, geotransform: GeoTransform, epsg: EPSG, nodata: float=None):
        """
        @param nodata: which value should be no data
        """
        self.data = array
        self.geotransform = geotransform
        self.epsg = epsg
        self.nodata = nodata

        self.raster = gdal_array.OpenNumPyArray(array, True)
        self.raster.SetGeoTransform(astuple(geotransform))
        srs = osr.SpatialReference()
        srs.ImportFromEPSG(epsg)
        self.raster.SetSpatialRef(srs)

    def __del__(self):
        self.raster = None
        self.data = None

    @staticmethod
    def from_geotiff(infile: str) -> 'Raster':
        ds = gdal.Open(infile)
        proj = osr.SpatialReference(wkt=ds.GetProjection())
        epsg = int(proj.GetAttrValue('AUTHORITY', 1))
        data = ds.ReadAsArray()
        nodata = set(ds.GetRasterBand(i).GetNoDataValue() for i in range(1, data.shape[0] + 1))
        assert len(nodata) == 1, "Do not support multiple no data value by now"
        nodata = list(nodata)[0]
        return Raster(data, GeoTransform.from_gdal(ds.GetGeoTransform()), epsg, nodata)

    @staticmethod
    def from_netcdf4(infile: str, varname: str):
        ds = Dataset(infile)
        gdal_ds = gdal.Open("NETCDF:{0}:{1}".format(infile, varname), gdal.GA_ReadOnly)

        # TODO: check what [0] actually do
        variable = ds.variables[varname][0][::-1]
        data = np.asarray(variable)

        # the coordinate is totally mess up, don't know about other datasets
        # re-arrange the geotransform because gdal netcdf geo-transformation is wrong
        gt = GeoTransform.from_gdal(gdal_ds.GetGeoTransform())
        print("gdal geotransform=", gt)
        x_max = gt.x_min + gt.x_res * data.shape[1]
        y_max = gt.y_min + gt.y_res * data.shape[0]
        # gt = GeoTransform(y_min=x_max, y_angle=gt.x_angle, y_res=-gt.x_res, x_min=y_max, x_angle=gt.y_angle, x_res=-gt.y_res)
        # print("correct gdal geotransform", gt)
        # data = np.rot90(data)  # counter clockwise

        nodata = gdal_ds.GetRasterBand(1).GetNoDataValue()
        if gdal_ds.GetProjection() != '':
            proj = osr.SpatialReference(wkt=gdal_ds.GetProjection())
            print(proj)
            print(">>>")
            epsg = int(proj.GetAttrValue('AUTHORITY', 1))
        else:
            epsg = EPSG.WGS_84
        print(epsg)

        return Raster(data, gt, epsg, nodata)

    def crop(self, bounds: BoundingBox = None, vector_file: Union[Path, str] = None, use_vector_bounds: bool = True,
             x_res: float = None, y_res: float = None, resampling_algo: ReSample = None) -> 'Raster':
        """
        @param x_res, y_res None will use original resolution
        """
        tmp_file = f"/vsimem/{datetime.datetime.now().strftime('%Y%m%d-%H%M%S.%f')}.tif"
        warp_options = {'format': 'GTiff'}
        if vector_file is not None:
            warp_options['cutlineDSName'] = vector_file
            warp_options['cropToCutline'] = use_vector_bounds
            assert os.path.exists(vector_file)
        elif bounds is not None:
            warp_options['outputBounds'] = bounds.to_gdal()
        else:
            raise Exception('Please specify either bounds or vector_file to crop.')
        warp_options['xRes'] = x_res
        warp_options['yRes'] = y_res
        warp_options['srcNodata'] = self.nodata
        warp_options['resampleAlg'] = resampling_algo.value if resampling_algo is not None else None
        tmp_ds = gdal.Warp(tmp_file, self.raster, **warp_options)
        cropped_array, cropped_geotransform = tmp_ds.ReadAsArray(), GeoTransform.from_gdal(tmp_ds.GetGeoTransform())
        gdal.Unlink(tmp_file)

        return Raster(cropped_array, cropped_geotransform, self.epsg, self.nodata)

    def to_geotiff(self, outfile: str):
        driver = gdal.GetDriverByName("GTiff")
        if len(self.data.shape) == 2:
            data = self.data.reshape((1, *self.data.shape))
        elif len(self.data.shape) == 3:
            data = self.data
        else:
            raise Exception("Does not support writing non 2 or 3 dims array to geotiff file")

        bands, rows, cols = data.shape
        outdata = driver.Create(outfile, cols, rows, bands, self.dtype_np2gdal(data.dtype))
        outdata.SetGeoTransform(self.raster.GetGeoTransform())
        outdata.SetProjection(self.raster.GetProjection())
        for band in range(bands):
            outdata.GetRasterBand(band + 1).WriteArray(data[band])
            if self.nodata is not None:
                outdata.GetRasterBand(band + 1).SetNoDataValue(self.nodata)
        outdata.FlushCache()

    def serialize(self, outfile: str):
        np.savez_compressed(outfile,
                            data=self.data,
                            geotransform=self.geotransform.to_gdal(),
                            epsg=int(self.epsg),
                            nodata=self.nodata)

    @staticmethod
    def deserialize(infile: str):
        result = np.load(infile)
        data = result['data']
        geotransform = GeoTransform.from_gdal(result['geotransform'])
        return Raster(data, geotransform, EPSG(result['epsg']), result['nodata'])

    @staticmethod
    def dtype_np2gdal(np_dtype):
        if np_dtype == np.float32:
            return gdal.GDT_Float32
        elif np_dtype == np.uint8:
            return gdal.GDT_UInt16
        elif np_dtype == np.float64:
            return gdal.GDT_Float64
        else:
            raise NotImplementedError(np_dtype)


if __name__ == '__main__':
    raster = Raster.from_netcdf4("/data/mint/gpm/3B-HHR-E.MS.MRG.3IMERG.20140101-S000000-E002959.0000.V06B.HDF5.nc4", "HQprecipitation")
    raster = Raster.from_geotiff("/data/Sample/world.tif")
    print(raster.geotransform)
    # ethiopia = BoundingBox(32.75418, 3.22206, 47.98942, 15.15943)
    # # raster = raster.crop(bounds=ethiopia, resampling_algo=ReSample.BILINEAR)
    # # ethiopia = "/data/country_boundary/countries/ethiopia.shp"
    # ethiopia = "/data/woredas/Warder.shp"
    # raster = raster.crop(vector_file=ethiopia, resampling_algo=ReSample.BILINEAR)
    # raster.to_geotiff("/data/Sample/somali.tif")
