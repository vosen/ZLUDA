# Precompiling:

Consider precompiling the GPU code with `zluda_precompile` if you are trying to run a large application.
`zluda_precompile` scans the location, extracts all the GPU code, compiles it, and saves it to the cache.
This way, the GPU code is already in the cache when the application is launched for the first time.

This process uses all the threads on the machine (making it faster than leaving it to the application) and may compile more code than is necessary for your application (making it potentially slower than leaving it to the application). Your mileage might vary.

## Usage:

Windows:
```
zluda_precompile.exe <PATH>
```
Linux:
```
zluda_precompile <PATH>
```


where <PATH> is the path to the directory or file.