# Generate plot of evolution and

import os
import pathlib
import numpy as np
import matplotlib.pyplot as plt
import rastro

SCRIPT_NAME = pathlib.Path(__file__).stem

# RASTRO_FIGURE_OUTPUT_DIR is an environment variable set at build time
# to determine where generated figures are located
OUTDIR = os.getenv("RASTRO_FIGURE_OUTPUT_DIR")

if __name__ == '__main__':
    # Create figure
    ax = plt.subplot(111)

    # Generate range of true anomalies
    nu = [x for x in range(0, 360)]

    # Compute and plot eccentric anomaly for range of true anomalies
    for e in [0.0, 0.1, 0.3, 0.5, 0.7, 0.9]:
        # Take output mod 360 to wrap from 0 to 2pi
        ecc = [rastro.anomaly_true_to_eccentric(x, e, True) % 360 for x in nu]
        ax.plot(nu, ecc, linewidth=2, label=f'e={e:.1f}')

    # Adjust figure style
    ax.set_xlim([0, 360])
    ax.set_ylim([0, 360])
    ax.spines['right'].set_visible(False)
    ax.spines['top'].set_visible(False)
    plt.xticks(np.arange(0, 390, step=30))
    plt.yticks(np.arange(0, 390, step=30))
    plt.xlabel(r"True Anomaly, $\nu$ [deg]")
    plt.ylabel(r"Eccentric Anomaly, $E$ [deg]")
    plt.grid(color='grey', linestyle='--', linewidth=0.5)

    # Save figure
    ax.legend()
    OUTFILE = f"{OUTDIR}/{SCRIPT_NAME}.svg"
    plt.savefig(OUTFILE, transparent=True)