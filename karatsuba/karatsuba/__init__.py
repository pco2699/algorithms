__version__ = '0.1.0'

def main():
    # print(485*12)
    # print(karatsuba("485", "12"))

    print(karatsuba("3141592653589793238462643383279502884197169399375105820974944592", "2718281828459045235360287471352662497757247093699959574966967627"))
 
def karatsuba(x: str, y: str):
    n = max(len(x), len(y))
    if n == 1:
        return int(x) * int(y)
    
    if len(x) > len(y):
        y = y.zfill(n)
    elif len(y) > len(x):
        x = x.zfill(n)

    if n % 2 != 0:
        y = '0' + y
        x = '0' + x
        n += 1
    

    print("{}, {}, {}".format(x, y, n))
    a = x[0:n//2]; b = x[n//2:n]; c = y[0:n//2]; d = y[n//2:n]
    print("{}, {}, {}, {}".format( a, b, c, d))
    ac = karatsuba(a, c)
    bd = karatsuba(b, d)
    a_b = str(int(a) + int(b))
    c_d = str(int(c) + int(d))
    abcd = karatsuba(a_b, c_d)
    ad_bc = abcd - ac - bd

    return (10 ** n) * ac + (10 ** (n // 2)) * ad_bc + bd
