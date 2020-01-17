import pickle

curr_dir = "./"

infile = curr_dir + "GLDAS_NOAH025_3H.A20080101.0000.021.nc4"
varname = "precipitation_flux"
drepr_file = curr_dir + "gldas.yml"


def get_data():
    from drepr import DRepr, outputs
    sm = outputs.ArrayBackend.from_drepr(drepr_file, infile)
    mint = sm.ns("https://mint.isi.edu/")
    rdf = sm.ns(outputs.Namespace.RDF)
    mint_geo = sm.ns("https://mint.isi.edu/geo")

    result = []

    for c in sm.c(mint.Variable).filter(outputs.FCondition(mint.standardName, "==", varname)):
        for raster_id, sc in c.group_by(mint_geo.raster):
            data = sc.p(rdf.value).as_ndarray([sc.p(mint_geo.lat), sc.p(mint_geo.long)])
            gt_info = sm.get_record_by_id(raster_id)

            if data.index_props[0].size > 1 and data.index_props[0][1] > data.index_props[0][0]:
                # create north-up image
                data.data = data.data[::-1]
                data.index_props[0] = data.index_props[0][::-1]

            result.append({
                "gt": dict(x_min=gt_info.s(mint_geo.x_min),
                          y_max=gt_info.s(mint_geo.y_min) + gt_info.s(mint_geo.dy) * data.data.shape[0],
                          dx=gt_info.s(mint_geo.dx), dy=-gt_info.s(mint_geo.dy)),
                "data": [data.data, int(gt_info.s(mint_geo.epsg)),
               data.nodata.value if data.nodata is not None else None]
            })
            # gt = GeoTransform(x_min=gt_info.s(mint_geo.x_min),
            #                   y_max=gt_info.s(mint_geo.y_min) + gt_info.s(mint_geo.y) * data.data.shape[0],
            #                   dx=gt_info.s(mint_geo.dx), dy=-gt_info.s(mint_geo.y))
            # raster = Raster(data.data, gt, int(gt_info.s(mint_geo.epsg)),
            #        data.nodata.value if data.nodata is not None else None)
            # raster.to_geotiff(curr_dir + f"{varname}.drepr.tif")
    assert len(result) > 0
    return result

data = get_data()
with open("./tmp.pkl", "wb") as f:
    pickle.dump(data, f)
exit(0)

with open("./tmp.pkl", "rb") as f:
    data = pickle.load(f)

from raster import Raster, GeoTransform, EPSG, BoundingBox, ReSample
for x in data:
    gt = GeoTransform(**x['gt'])
    print(gt)
    raster = Raster(x['data'][0], gt, x['data'][1],
           x['data'][2])
    raster.to_geotiff(curr_dir + f"{varname}.drepr.tif")
