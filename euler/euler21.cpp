#include <iostream>
#include <vector>

// range-based アルゴリズム使いたかった
#include <boost/range/algorithm.hpp>
#include <boost/range/algorithm_ext.hpp>
#include <boost/range/numeric.hpp>

inline int sum_of_divisors(int const n)
{
    std::vector<int> v(n-1);
    return boost::accumulate(boost::remove_erase_if(boost::iota(v, 1), [&](int i){return n%i!=0;}), 0);
}

bool is_not_amicable_num(int const n)
{
    auto tmp = sum_of_divisors(n);
    return n == tmp || n != sum_of_divisors(tmp);
}

int main()
{
    std::vector<int> v(10000);
    std::cout << boost::accumulate(boost::remove_erase_if(boost::iota(v, 2), is_not_amicable_num), 0) << std::endl;
    return 0;
}
