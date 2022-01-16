# Generate plot of mean anomaly versus true anomaly for a range of eccentricies.
# Highlights the effect of eccentricity on the difference of the two.


import os
import pathlib
import numpy as np
import plotly.graph_objects as go
import plotly.io as pio
import rastro

## Define Constants
SCRIPT_NAME = pathlib.Path(__file__).stem
OUTDIR = os.getenv("RASTRO_FIGURE_OUTPUT_DIR") # Build Environment Variable
OUTFILE = f"{OUTDIR}/{SCRIPT_NAME}.html"

## Create figure
fig = go.Figure()
fig.update_layout(dict(paper_bgcolor='rgba(0,0,0,0)', plot_bgcolor='rgba(0,0,0,0)'))
fig.update_yaxes(
    tickmode='linear', title_text="UT1-UTC offset [seconds]"
)

## Generate and plot data

# Load EOP Data
eop = rastro.EarthOrientationData.from_default_standard("Hold", True, "StandardBulletinA")

# Get range of dates stores in EOP data
days = np.arange(eop.mjd_min, eop.mjd_max, 1)

# Get UT1-UTC offsets
ut1_utc = [eop.get_ut1_utc(mjd) for mjd in days]

fig.add_trace(go.Scatter(x=days, y=ut1_utc))

# Update Axes
fig.update_xaxes(
    tickmode='linear', tick0=eop.mjd_min, dtick=300, tickformat='5f',
    title_text="Modified Julian Date"
)
fig.update_xaxes(
    showgrid=True, gridwidth=1, gridcolor='LightGrey', range=[eop.mjd_min, eop.mjd_max],
    showline=True, linewidth=2, linecolor='Grey'
)
fig.update_yaxes(
    showgrid=True, gridwidth=1, gridcolor='LightGrey',
    showline=True, linewidth=2, linecolor='Grey'
)

pio.write_html(fig, file=OUTFILE, include_plotlyjs='cdn', full_html=False, auto_play=False)