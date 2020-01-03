# Introduction

Beside the RDF that D-REPR can produce, it also support outputting data in a in-memory data structure, called data access layer (DAL), that enables users to easily work with heterogeneous datasets. Below are goals of the DAL.
1. Users can iterate through all records and links between records
2. Users can select a subset of the data (filtering)
3. Users can group the data based on some key.

At the high level, DAL displays to users the schema, which comprises classes as tables. There can be multiple classes of same name (e.g., two svo:Variable classes) in DAL, and they are belong to two different tables.

# Access the data

## Access individual records

Users can choose to iterate through records of a class. Each iteration, it yields a proxy record, which acts as a row in a table, hiding the detailed implementation from the end-users. For example, for netcdf dataset, the proxy record contains references to different arrays that it will extract data from only fly. 

The proxy record also allows users to navigate to other records in other tables that it is linked to.

## Access groups of records as arrays

For high volume data, which was original stored in NetCDF or Numpy, users may want to get all of the data of a property as array. The DAL class provides an interface to achieve it. Implementing this feature require almost nothing if the data is already in array format (netcdf datasets), but in case the data is not in array format (json, xml, etc.), then we can add an extra steps that copy the values to an array.

One problem that occurs when the data is presented to users in the array form is that we lose the way to access to other records in other classes. For example, we may have a class precipitation and a class Geo2D that every records in the precipitation class are linked to one Geo2D instance saying each observation value is one point in the 2D grid, hence users need to know the grid of the precipitation array so that users can do some operations such as cropping. 

Note that in these cases, values of the array that users want to obtain should belong to one group in some conditions that users defined, and there should be more than one values, otherwise, they can just 
work with individual records using the first method.

Therefore, to obtain the links, users can group records of a class based on a property. Each group contains records and the groupping property, so it encodes the link. We can do groupping further if we need to obtain more links.

# Select a subset of data

The DAL support filtering a class based on some properties. For example: `RDFSClass.filter(RDFSClass.property == "Something")`. We can simply loop through each record to obtain a subset. This filtering is going to be fast if the selected property is static property, or the property that has less number of values.

For the array format, we need to maintain original linking values so that we can link them faster:
    1. We rank the filtered properties based on number of values
    2. For each filtered property, (even if it is property of another link class, we can convert to the subject property), we select its index. turns it to subject index, and filter the subject. We keep the subject index as the reuslt of the filter.


Filter based on has_link: howver, has link may not what they want..

# Groupping data

Group based on a property. we can use the index method above.


class RDFSClass:
    id: int
    name: str
    data_props: List[DataProperty]
    link_props: List[ObjectProperty]

    def iter_records(self) -> Iterable[ProxyRecord]:
        pass