# Intro

The `data` submodule provides functionality for handling of various underlying 
data sources and types required for high fidelity astrodynamics modeling. 
This data includes, but is not limited to time system data, Earth 
orientation data, and space weather data.

## Package Data

To allow RAstro to be easy and quick to use , the package is distributed 
batteries-included with a number of important data files packaged with the 
distribution. The presence of these data files allows for users to quickly 
start building on-top of the library without requiring the user to undertake 
additional setup steps to find and download additional core external data 
files.

The data files distributed as part of the package are:

| Data File                | Type                         | Description                                                                     |
|--------------------------|------------------------------|---------------------------------------------------------------------------------|
| `EGM2008_90.gfc`         | Gravity Model                | EGM 2008 Gravity Model                                                          |
| `GGM05C.gfc`             | Gravity Model                | 2005 Grace Gravity Model                                                        |
| `iau2000A_c04_14.txt`    | Earth Orientation Parameters | IERS C04 (final) EOP parameter solution for IAU 2010 Precession/Nutation Models |
| `iau2000A_finals_ab.txt` | Earth Orientation Parameters | IERS rapid EOP parameter solution for IAU 2010 Precession/Nutation Models       |