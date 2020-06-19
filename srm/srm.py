def srm(path, passes=5):

    from os import remove, urandom

    with open(path, 'rb+') as arquivo:

        arquivo.seek(0, 2)
        tamanho = arquivo.tell()
        
        for A in range(passes):
            arquivo.seek(0)
            arquivo.write(urandom(tamanho))

    remove(path)
