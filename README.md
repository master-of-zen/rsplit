![](https://cdn.discordapp.com/attachments/829353018943602708/880923409041743903/rsync.png)
# rsplit
Splits video into chunks by size (Like if you have discord limit in 8MB)

## Behaviour
Rsplit will create chunks with numerated naming and maximum size of S until end of file.

## Usage
```
    -i, --input  Input file
    -o, --output Ouput file
    
    -s, --size   Chunk size, allowed different units [KB,KiB,MB,MiB,GB,GiB]
                 Example:  -s "5 MiB" , -s "800000 KB"
                
