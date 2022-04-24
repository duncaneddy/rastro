# Earth Orientation Data

The `eop` module provides data structures and functions for the updating, 
loading, and use of Earth orientation parameter (EOP) data.

Earth Orientation Parameters are empirically observed, estimated 
parameters that describe the irregularities in Earth's rotation in space. 
When combined with their specific related models they provide the mechanism 
to transform between an Earth-Centered Earth-Fixed (ECEF) reference frame 
and an inertial reference frame.

## IERS

The [International Earth Rotation Service (IERS)](https://www.iers.org/) was 
established in 1987 by the International Astronomical Union and the 
International Union of Geodesy and Geophysics. The IERS provides data on Earth 
orientation, on the International Celestial Reference System/Frame, and on the 
International Terrestrial Reference System/Frame. The IERS also maintains 
conventions containing models, constants and standards used for modeling Earth orientation.

The IERS deals with reference _systems_ and reference _frames_. A _reference 
system_ is an idealized mathematical concept for defining a reference used 
to represent the state of objects in that system. The two primary reference 
systems developed by the IERS are the International Celestial Reference 
System (ICRS) and International Terrestrial Reference System (ITRS).

A reference system is a concept and cannot be used directly, therefore the 
IERS develops _reference frames_, which are specific realizations of a given 
reference system. A reference frame realization defines the models, 
standards, and associated data products for users to actually interact and use
that reference system. The primary reference frames of the IERS are 
the International Celestial Reference Frame (ICRF) and International 
Terrestrial Reference Frame (ITRF).

The ICRS and ITRS models are defined with respect to the solar system 
barycenter[^1]. However, for many satellite-specific engineering 
applications we are primarily concerned with _geocentric_ references, 
centered at Earth. Therefore, RAstro primarily deals with the Geocentric 
Celestial Reference Frame (GCRF) and Geocentric Terrestrial Reference Frame 
(GTRF). For most intents and purposes the international and geocentric 
references are identical as there is no rotation component between ICRS and 
GCRS (or ITRF and GCRF)[^2]. The transformation between the two reference 
systems and frames is simply a pure translation.

For a more detailed discussion of reference frames and systems please read 
[IERS Technical Note 36](https://www.iers.org/SharedDocs/Publikationen/EN/IERS/Publications/tn/TechnNote36/tn36_174.pdf?__blob=publicationFile&v=1)
provides an in-depth discussion of the concepts presented and discussed here.

## Earth Orientation Products

The IERS provides various Earth orientation products which are derived from
Very Long Baseline Interferometry (VLBI) or a network of terrestrial GPS[^3] 
reference stations. The continual observations made by these stations are 
combined with specific reference frame realizations (e.g. the IAU 2010 
conventions) to model Earth orientation and enable the transformation between 
inertial and Earth-fixed reference frames.

The Earth orientation parameter products come in multiple variations, all of 
which can be found at the [IERS data products site](https://www.iers.org/IERS/EN/DataProducts/EarthOrientationData/eop.html). 
These variations arise from the selection of precession-nutation model, ITRF 
realization, the data sources, and data processing time span. There are two 
precession-nutation models widely in use today: IAU 1980 nutation theory and
the IAU2006/2000A precession-nutation model. The ITRF 2014 realization is 
the most recent realization and preferred in most cases.

For data products there are two primary distinctions: standard products and 
long term products.
Standard products, which are produced daily, to provide a daily estimate of the 
past Earth orientation along with forward-looking predictions available for 
use in planning. Long term data products are only available for past days, 
and are produced less frequently, but provider higher accurate estimates of 
Earth orientation. 

For most purposes the standard products provide sufficient accuracy along with 
the benefit of having fairly accurate forward-looking predictions. Therefore, 
RAstro defaults to using standard Earth Orientation data products wherever 
possible. Unless otherwise stated or specified, RAstro uses IERS standard 
product generated with respect to IAU 2006/2000A precession-nutation model and 
consistent with ITRF2014.

## Earth Orientation Parameters

Rastro provides the `EarthOrientationProvider` structure to handle loading, storing, and 
providing Earth orientation data for use. The package also includes default data files 
for ease of use that are sufficient for most purposes.

The `EarthOrientationProvider` structure can be used to handle storing and accessing the Earth
orientation data directly, however it is generally preferable to use the static instance of this
object provided by the crate. This static instance of `EarthOrientationProvider` is a global
source of Earth orientation which is loaded once, then utilized by other RAstro function which
need Earth orientation data. This prevents having to load the data multiple times enables
consistent API design between the Rust and Python implementations. 

??? info

    Explicitly passing Earth orientation was tried in an early implementation, however 
    the Rust brorrow checker, prevented python wrapper functions from directly passing
    the underlying Rust reference between them. This would have then required the
    Python wrapper to perform a clone of the Earth orientation data for most time operations
    leading to unacceptable overhead. Having a single static instance of Earth orientation
    data for the crate, protected by a read-write lock and atomic reference counting
    enabled the creation of a consistent API between both Rust and Python implementations
    without sacrifing performance or safety.

### Loading Data Data Sets

Most software using this library requires upfront, explicit initialization of the 
static Earth orientation data. Earth orientation data is loaded globally by calling one
of the provided loading methods: `set_global_eop_from_zero`, `set_global_eop_from_static_values`,
`set_global_eop_from_c04_file`, `set_global_eop_from_default_c04`, `set_global_eop_from_standard_file`,
or `set_global_eop_from_default_standard`. These methods can be called multiple times
to reset or override the currently loaded data.

The `set_global_eop_from_zero` will initialize the global data with zeroed values. This enables
usage of other module functionality, but does not provide the most accurate modeling of Earth
or time systems. It should be used when a quick, approximately correct answer is needed.
`set_global_eop_from_static_values` is a similar initialization method which configures the
module with a single set of Earth orientation data used for all transformations.

To configure more accurate Earth orientation data to use in the module, `set_global_eop_from_c04_file`
can be used to load long-term IERS C04 products and `set_global_eop_from_standard_file` to load either
Bulletin A or Bulletin B data from the IERS standard file product format.

RAstro distributions also include packaged IERS C04 and Bulletin A/B data. These can be
configured using `set_global_eop_from_default_c04` or `set_global_eop_from_default_standard`,
respectively. While not updated regularly.

For the most accurate Earth orientation data modeling in scripts, you should download the
latest available Earth orientation data for the desired model and the using
the file-based loading methods (`set_global_eop_from_c04_file` or `set_global_eop_from_standard_file`)
to initialize the Earth orientation data based on the file.

When creating any new Earth Orientation data instance there are two parameters that are set at 
loading time which will determine how the EOP instances handles data returns for certain cases.
The first parameter is the `extrapolate` parameter, which can have a value of `Zero`, `Hold`, or 
`Error`. This value will determine how requests for data points beyond the end of the loaded 
data are handled. The possible behaviors are
- `Zero`: Returned values will be `0.0` where data is not available
- `Hold`: Will return the last available returned value when data is not available
- `Error`: Data access attempts where data is not present will panic and terminate the program

The second parameter the `interpolate` setting. When `interpolate` is set to true and data 
requests made for a point that wasn't explicitly loaded as part of the input data set will be 
linearly interpolated to the desired time. When set to `false`, the function call will return 
the last value prior to the requested data.

Below is an example of loading C04 data

=== "Rust"

    ``` rust
    --8<-- "../examples/eop_c04_loading.rs"
    ```

=== "Python"

    ``` python
    --8<-- "../examples/eop_c04_loading.py"
    ```

The process for loading standard data is similar. However, when loading standard files there is one 
other parameter which comes into play, the Earth Orientation Type. 
This type setting determines whether the Bulletin A or Bulletin B data is loaded into the object 
when parsing the file. In rust

=== "Rust"

    ``` rust
    --8<-- "../examples/eop_standard_loading.rs"
    ```

=== "Python"

    ``` python
    --8<-- "../examples/eop_standard_loading.py"
    ```

!!! note

    For applications where the time is in the future it is recommended to use standard EOP data 
    as standard files contain predictions for approximately 1 year into the future and will 
    increase accuracy of analysis by accounting for Earth orientation corrections.

    For analysis for scenarios in the past it is recommended to use the final C04 products as they
    contain the highest accress estimates of Earth orientation data.

### Accessing Earth Orientation Data

Most of the time the data stored by the Earth orientation object is not used directly. If your 
application calls for accessing the `EarthOrientationProvider` object provides a number of 
methods for accessing different Earth orientation Parameters stored by the object. However, in most
cases, it is best to use the data for the crate's loaded static Earth orientation data. In these 
cases the following methods can be used to access the loaded static Earth orientation data:
- `get_global_ut1_utc`
- `get_global_pm`
- `get_global_dxdy`
- `get_global_lod`
- `get_global_eop`

The following methods return information on the currently loaded Earth orientation data:
- `get_global_eop_initialization`
- `get_global_eop_len`
- `get_global_eop_type`
- `get_global_eop_extrapolate`
- `get_global_eop_interpolate`
- `get_global_eop_mjd_min`
- `get_global_eop_mjd_max`
- `get_global_eop_mjd_last_lod`
- `get_global_eop_mjd_last_dxdy`

=== "Rust"

    ``` rust
    --8<-- "../examples/eop_data_access.rs"
    ```

=== "Python"

    ``` python
    --8<-- "../examples/eop_data_access.py"
    ```

One example of using the Earth orientation data directly is plotting the evolution of the difference
between the UT1 and UTC timescales. The discontinuous jumps are when leap seconds were introduced.

--8<-- "./docs/figures/fig_ut1_utc_evolution.html"

??? "Plot Source"

    ``` python title="fig_ut1_utc_evolution.py"
    --8<-- "../figures/fig_ut1_utc_evolution.py"
    ```

### Downloading updated Earth Orientation Data

The final functionality that Rastro provides is the ability to download new Earth orientation 
parameter data files.

The functions `download_c04_eop_file` and `download_standard_eop_file` can be used to downloaded 
the latest product files from IERS servers and store them locally at the specified filepath. The 
download functions will attempt to create the necessary directory structure if required.

=== "Rust"

    ``` rust
    use rastro::eop::{download_c04_eop_file, download_standard_eop_file};

    fn main() {
        // Download latest C04 final product file
        download_c04_eop_file("./c04_file.txt").unwrap();
    
        // Download latest standard product file
        download_standard_eop_file("./standard_file.txt").unwrap();
    }
    ```

=== "Python"

    ``` python
    import rastro

    if __name__ == '__main__':
        # // Download latest C04 final product file
        rastro.eop.download_c04_eop_file("./c04_file_py.txt")
    
        # // Download latest standard product file
        rastro.eop.download_standard_eop_file("./standard_file_py.txt")
    ```

If using the RAstro CLI, product files can be download with

```bash
rastro eop download --product final final_c04_eop_file.txt
```

or 

```bash
rastro eop download --product standard standard_eop_file.txt
```


[^1]: A barycenter is the center of mass of two or more bodies. The solar 
system barycenter is the center of mass of the entire solar system. Due to 
significant mass contributions and distances of Jupiter and Saturn, the 
solar system barycenter evolves in time and is sometimes outside of the 
Sun's outer radius.
[^2]: For applications requiring the highest levels of fidelity, the 
equations of motion of an Earth satellite, with respect to the 
GCRS will contain a relativistic Coriolis force due to geodesic precession 
not present in the ICRS. 
[^3]: Now frequently GNSS receivers