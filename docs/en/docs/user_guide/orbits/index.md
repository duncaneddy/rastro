# Orbits Intro

The `orbits` submodule provides functions for the analysis of satellite orbits 
and their properties. The functions of this module frequently represent 
common fundamental astrodynamics concepts or are useful in quick analysis of 
specific orbital scenarios.

## Osculating Orbital Elements

The properties of a satellite orbit most commonly described in terms of its
_**osculating orbital elements**_. The osculating orbital elements, 
frequently just referred to as the _orbital elements_[^1], describe the 
state of an object in space at an instantaneous moment in 
time with respect to trajectory that it would have around its central body if 
no perturbations were present. That is, the osculating elements are the 
orbital elements that fit a Keplerian orbit trajectory of the object with 
respect to its central body for a given moment in time.

Since Keplerian orbits assume no other forces are present, they do not occur in 
reality. However, because in many cases the conservative (higher-order 
gravity, third-body gravity, etc.) and non-conservative (drag, solar radiation 
pressure, propulsion, etc.) perturbations present are an order-of-magnitude 
smaller than the point-mass gravitational force or more, present as 
higher-order perturbations. This makes the orbital element of a trajectory a 
useful tool in understanding the general motion of an object.

The _orbit_ submodule provides functions to perform computations on orbital 
element representations of satellite trajectories to gain useful insight 
into a trajectory's properties.

[^1]: In addition to osculating orbital elements there are also _mean 
orbital elements_. Mean elements average the effect of higher-order 
perturbations to capture the secular (constant) and long-periodic trends 
affecting orbital trajectories.