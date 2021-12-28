import os
import pathlib
import plotly.express as px
import pandas as pd
import plotly.io as pio

SCRIPT_NAME = pathlib.Path(__file__).stem

# RASTRO_FIGURE_OUTPUT_DIR is an environment variable set at build time
# to determine where generated figures are located
OUTDIR = os.getenv("RASTRO_FIGURE_OUTPUT_DIR")

df = pd.DataFrame(dict(
    x = [1, 3, 2, 4],
    y = [1, 2, 3, 4]
))
fig = px.line(df, x="x", y="y", title="Unsorted Input")

df = df.sort_values(by="x")
fig = px.line(df, x="x", y="y", title="Sorted Input")

OUTFILE = f"{OUTDIR}/{SCRIPT_NAME}.html"
# pio.write_html(fig, file=OUTFILE, include_plotlyjs='cdn', full_html=True)
print(pio.write_html(fig, file=OUTFILE, include_plotlyjs='cdn',
                     full_html=False, auto_play=False))