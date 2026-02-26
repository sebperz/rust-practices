import math

def function(num):
    return math.exp(-num)
    # return 0.4 * math.exp(num ** 2)

def calc_error(x0, x1):
    return abs((x1 - x0) / x1)

def main():
    x = 0.0
    target_error = 0.01

    while True:
        x_plus_one = function(x)
        print(f"Value of Xi: \t{x}")
        print(f"Value of Xi+1: \t{x_plus_one}")
        error = calc_error(x, x_plus_one)
        print(f"Error: {error}\n")

        if error <= target_error:
            print("END of process")
            break
        x = x_plus_one

if __name__ == "__main__":
    main()
