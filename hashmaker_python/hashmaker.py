import hashlib
from traceback import print_exc
import click


@click.command()
@click.argument('arquivo', type=click.STRING)
@click.argument("hash", type=click.STRING)
def setup(arquivo, hash):

    data = open(arquivo, 'rb').read()
    saida = ''
    try:
        saida = eval(f"hashlib.{hash}(data).hexdigest()", {"hashlib": hashlib, "data": data})

    except AttributeError:
        print("Hash não disponivel")

    print(f"Resultado = |{saida}|")


setup()
