# Specification of D-REPR (v2)

## Resources

1. NetCDF:

Represented as a tree:

```
{
  "@": {
    // global metadata of a netcdf 
    "filename": <name of the file> 
  },
  <variable_id>: {
    "data": <data of variable since netcdf allow storing not only numbers>,
    "@": {
       // metadata of a variable
       <key>: <value>
    }   
  }
}
```

## Expression

This is a mechanism to obtain a number from the data itself (for example, missing values are coming from the metadata)
 