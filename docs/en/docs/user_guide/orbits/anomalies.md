# True, Eccentric, and Mean Anomaly

This section deals with the conversion between true, eccentric, and mean 
anomaly. 

True anomaly, frequently denoted $\nu$, is the angular parameter that defines 
the position of an object moving along a Keplerian orbit. It is the angle 
between the eccentricity vector (vector pointing from the main pericenter to 
the periapsis) and the current position of the body in the orbital plane itself.

The eccentric anomaly is another angular parameter that defines the position 
of an object moving along a Keplerian orbit if viewed from the center of the 
ellipse. 

Finally, the mean anomaly defines the fraction of an orbital period that has 
elapsed since the orbiting object has passed its periapsis. It is the angle 
from the pericenter an object moving on a fictitious circular orbit with the 
same semi-major axis would have progressed through in the same time as the 
body on the true elliptical orbit.

Conversion between true, eccentric, and mean anomaly is done converting 
between true and eccentric anomaly, and eccentric and mean anomaly.

## True and Eccentric Anomaly Conversions

To convert from true anomaly to eccentric anomaly, you can use the function 
`anomaly_eccentric_to_true`. To perform the reverse conversion use 
`anomaly_true_to_eccentric`.

=== "Rust"

    ``` rust
    --8<-- "../examples/anomaly_true_and_eccentric.rs"
    ```

=== "Python"

    ``` python
    --8<-- "../examples/anomaly_true_and_eccentric.py"
    ```

??? "Plot Source"
    
    ``` python title="fig_anomaly_true_eccentric.py"
    --8<-- "../figures/fig_anomaly_true_eccentric.py"
    ```

## Eccentric and Mean Anomaly Conversions

To convert from true anomaly to eccentric anomaly, you can use the function
`anomaly_eccentric_to_mean`. To perform the reverse conversion use
`anomaly_mean_to_eccentric`. 

There is no known closed-form solution to 
convert from mean anomaly to eccentric anomaly. Instead, a numerical 
algorithm that iteratively refines and initial guess to converge on the 
eccentric anomaly is used. It is possible that in some cases, usually for 
highly elliptic orbits, this process does not converge within the fixed 
number of iterations. Therefore, `anomaly_mean_to_eccentric` returns a 
`Result`, forcing the user to explicitly handle this case. However, in 
almost all cases the algorithm will converge so simply unwrapping the type 
and receiving a runtime error will generally work.

=== "Rust"

    ``` rust
    ```

=== "Python"

    ``` python
    ```

## True and Mean Anomaly Conversions

Methods to convert from true anomaly to mean anomaly are 
provided for convenience. These methods simply wrap successive calls to two 
`anomaly_true_to_mean`. To perform the reverse conversion use
`anomaly_mean_to_true`.

=== "Rust"

    ``` rust
    ```

=== "Python"

    ``` python
    ```