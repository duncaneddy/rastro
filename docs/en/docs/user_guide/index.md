# Introduction

This user guide shows you how to use RAstro with a walk through of its 
modules, step by step.

Like RAstro, each section builds on previous ones but are each treated as 
separate topics allowing you to go directly to any specific topic or 
application of interest. This section is meant to provide a practical walk 
through of the library.

!!! check "Validation"

    A Validation section provides information on the validation and 
    verification steps used to provide indepdent confirmation of the 
    correctness of the RAstro library implementation.

## Library Architecture

Any astrodynamics library is a complex system that has to balance software 
implementation considerations with the underlying scientific models. RAstro is 
built based off of an astrodynamics-first principle with the belief that 
the software architecture should reflect the development of physics-based 
concepts at play. It is hoped that it will be easier for new users to 
navigate and learn software by logically grouping the software with respect 
to similar concepts taught in books and in schools. This enables users to 
apply their physics-based intuition in understanding the software.

With this development approach in mind RAstro is built starting from 
fundamental concepts like fixed-constants, time, and reference frames. Using 
these foundational functions higher level abstractions for more complex 
analysis like orbit dynamics and orbit propagation are built.

RAstro provides the following major modules and capabilities:

| Module            | Description                                                                                                        |
|-------------------|--------------------------------------------------------------------------------------------------------------------|
| `constants`       | Defined fixed mathematical and physicals constants.                                                                |
| `data`            | Provides functionality for loading fixed and time-varying empirical modeling data.                                 | 
| `time`            | Data structures and functions for handling time representation, including transformation between time systems.     |
| `orbits`          | Functions for representing and analyzing orbital trajectories in terms of orbital elements and related parameters. |
| `coordinates`     | Transformations between different coordinate represtations of object state.                                        |
| `refsys`          | Implementation of different reference systems and transformation between them.                                     |
| `orbit_dynamics`  | Models of orbital dynamics. Includes both conservative and non-conservative forces.                                |
| `tle`             | Functions for handling Two-Line Elements and associate SGP propagators.                                            |
| `attitude`        | Object orientation representation.                                                                                 |
| `propagators`     | Implementation of specific dynamics propagators                                                                    |
| `relative_motion` | Orbit and coordinate transforms for representation and analysis of satellite relative motion                       |
| `math`            | Mathematical support functions including numerical integrators used for propagation.                               | 


## Design Tenets

RAstro has a few design tenets that have guided the implementation to improve 
overall usability and safety. Safety in this case means that the user should 
be protected from using models incorrectly and from making unintentional 
assumptions based on internal implementation details.

The first such tenet 
is that all parameter values and function specifications of RAstro 
assume that the inputs and outputs are in [SI base units](https://en.wikipedia.org/wiki/SI_base_unit).
This is made so that at no point does the user have to check if inputs or 
outputs are of the right type. This allows for functions and parameters of 
RAstro to be directly composable.

The second tenet is that the default values for frequent operations and 
functions should be chosen to be consistent with the most common use cases 
as well as with the highest fidelity models possible. Users should not have 
to spend significant effort to build accurate, high-fidelity models. That work 
should be provided by the library whenever possible.

The third tenet is that it is better to be explicit rather than implicit
whenever possible. This ensures that the user must make conscious decisions about
which parameter values to use, which functions to call. This tenet is
sometimes in conflict with the second and in these cases the second tenent 
of usability is the preferred option with implementation assumptions 
captured in documentation and in code.