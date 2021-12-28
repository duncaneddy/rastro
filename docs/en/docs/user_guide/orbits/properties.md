# Orbital Properties

The `orbits` module also provides functions to help derive useful parameters
from the orbital trajectory.

## Periapsis Properties

There are a few properties of an orbit that can be derived from the periapsis of an orbit.

??? info

    Periapsis is formed by combination of the Greek words "_peri-_", meaning around, about and 
    "_apsis_"
    meaning "arch or vault". An apsis is the farthest or nearest point in the orbit of a 
    planetary body about its primary body. 

    Thereforce _periapsis_ of an orbit is the point of closest approach of the orbiting body with 
    respect to its central body. The suffix can be modified to indicate the point of 
    closest approach to a specific celestical body. The _perigee_ is the point of cloest approach to
    an object orbiting Earth. The _perihelion_ is the point of closest approach to the Sun.


For convenience, `perigee_*` functions are provided which wrap the general 
periapsis functions and specifically pass the `GM_EARTH` constants so that 
it does not need to be done by the user.

## Apoapsis Properties

??? info

    Apoapsis is formed by combination of the Greek words "_apo-_", meaning away from; separate, 
    apart from and "_apsis_".

    Thereforce _apoapsis_ of an orbit is the farthest point of an orbiting body with 
    respect to its central body. The suffix can be modified to indicate the farthest point
    with respect to a specific primary celestical body. The _apogee_ is furthest point away for 
    an object orbiting Earth. The _aphelion_ is the furthest away from an object orbiting the Sun.

## Mean Motion

## Orbital Period

## Sun-Synchronous Inclination

