from enum import Enum
import typer
import rastro

class ProductType(str, Enum):
    standard = "standard"
    final = "final"

app = typer.Typer()

@app.command()
def download(filepath:str = typer.Argument(..., help="Filepath to output data"),
             product:ProductType = typer.Option(..., help="Type of data product to download")):
    if product == ProductType.standard:
        rastro.eop.download_standard_eop_file(filepath)
    else:
        rastro.eop.download_c04_eop_file(filepath)