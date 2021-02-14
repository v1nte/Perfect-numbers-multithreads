from multiprocessing import Pool
TOTAL = 31


def is_prime(n: int) -> bool:

    if n == 2:
        return True

    if n <= 1 or n % 2 == 0:
        return False

    md: int = int(n ** 0.5) + 1

    for i in range(3, md, 2):
        if n % i == 0:
            return False

    return True


def is_perfect(a: int) -> int:

    if is_prime(a):
        aux = (2**a) -1

        if is_prime(aux):
            return aux*2**(a-1)
    return 0


if __name__=="__main__":

    print("p\tnumber\n")
    with Pool() as p:
        log = p.map(is_perfect, range(1, TOTAL + 1))

    for i, n in enumerate(log):
        if n:
            print(i, "\t", n)
        else:
            pass
