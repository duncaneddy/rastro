import typer
import rastro.cli.eop as eop

app = typer.Typer()
app.add_typer(eop.app, name="eop")

# Call the application (used by setup.py to create the entry hook)
def main():
    app()
