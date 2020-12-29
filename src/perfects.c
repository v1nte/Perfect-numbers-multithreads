#include <stdio.h>
#include <stdbool.h>
#include <math.h>
#define TOTAL 61

bool is_prime(long);
unsigned long long int is_perfect(int);

int main()
{
    unsigned long long int value;
    printf("p \t Number\n\n");
    for (int i = 1; i < TOTAL + 1; i++)
        if ((value = is_perfect(i)) != 0)
            printf("%d \t %llu\n", i, value);
}

bool is_prime(long n)
{
    if (n == 2) return true;

    if (n <= 1 || n % 2 == 0) return false;

    long maxdiv = sqrt(n) + 1;

    for (long i = 3; i < maxdiv; i += 2)
        if (n % i == 0)
            return false;

    return true;
}

unsigned long long int is_perfect(int a)
{
    if (is_prime(a)) {
        unsigned long long int aux = powl(2, a) - 1;
        if (is_prime(aux))
            return aux * powl(2, a - 1);
    }

    return 0;
}
