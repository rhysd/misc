#include <cstddef>

using std::size_t;

namespace detail {

    constexpr double power(double base, size_t n)
    {
        return n==0 ? 1 : base * power(base, n - 1);
    }

    constexpr double abs(double value)
    {
        return value>0.0 ? value : -value;
    }


    constexpr double nth_root_impl(double value, 
            size_t n,
            double base,
            double epsilon)
    {
        return abs( value - power(base,n) ) < epsilon ?
            base : 
            nth_root_impl( value, n, ((n-1) * base + value / power(base, n-1)) / n, epsilon );
    }

}

constexpr double nth_root(double value, size_t n)
{
    return detail::nth_root_impl(value, n, 1.0, 1e-7);
}

#include <iostream>

#define CHECK(v,n) static_assert( detail::power( nth_root( v, n ), n ) - v < 1e-7, "")

int main()
{
    CHECK(8816, 44);
    CHECK(6366, 31);
    CHECK(3643, 73);
    CHECK(5163, 8);
    CHECK(2592, 79);
    CHECK(4996, 67);
    CHECK(4947, 6);
    CHECK(3519, 60);
    CHECK(2940, 5);
    CHECK(6773, 2);
    CHECK(1826, 21);

    return 0;
}
