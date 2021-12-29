# Orbital Elements [WIP]

Orbital elements are closely related to the concept of _state_ from Control 
Theory.  A properly defined state representation of a system provides enough 
information about the system to determine its future behaviour in the 
absence of any unknown or unmodeled forces affecting the system. Orbital 
elements are one means of representing the _state_ of a system for 
orbital trajectories that can provide unique insight into that trajectory's
properties.

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

[^1]: In addition to osculating orbital elements there are also _mean
orbital elements_. Mean elements average the effect of higher-order
perturbations to capture the secular (constant) and long-periodic trends
affecting orbital trajectories.

## Keplerian Orbital Elements

The most common orbital elements used are the Keplerian Orbital Elements

$$
\vec{x}_{oe} = (a, e, i, \Omega, \omega, M)
$$