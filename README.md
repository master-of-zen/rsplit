# rsplit
Splits video into chunks

## Behaviour
Rsplit will create chunks with numerated naming and maximum size of S until end of file.

## Usage
```
    -i, --input Input file
    -s, --size  Chunk size, allowed different units [KB,KiB,MB,MiB,GB,GiB]
                Example:  -s "5 MiB" , -s "800000 KB"