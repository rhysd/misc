#include <iostream>
#include <utility>
#include <cstddef>

#include <boost/mpl/string.hpp>
#include <boost/mpl/insert_range.hpp>
#include <boost/mpl/end.hpp>
#include <boost/mpl/size_t.hpp>
#include <boost/mpl/char.hpp>
#include <boost/mpl/if.hpp>
#include <boost/mpl/eval_if.hpp>
#include <boost/mpl/identity.hpp>
#include <boost/mpl/copy.hpp>
#include <boost/mpl/back_inserter.hpp>

using std::size_t;
using namespace boost;

typedef mpl::string<'f', 'i', 'z', 'z'> fizz;
typedef mpl::string<'b', 'u', 'z', 'z'> buzz;

struct fizzbuzz
    : mpl::insert_range<fizz, typename mpl::end<fizz>::type, buzz>
{};

template<size_t N>
struct to_string_impl;

template<size_t N>
struct push_back_lazy
    : mpl::push_back<
        typename to_string_impl<N/10>::type,
        mpl::char_<'0' + (N % 10)>
      >
{};

template<size_t N>
struct to_string_impl
    : mpl::eval_if_c< (N < 10), mpl::string<'0' + N>, push_back_lazy<N>
    >::type
{};

template<class T>
struct to_string;

template<std::size_t N>
struct to_string<mpl::size_t<N>>
    : to_string_impl<N>
{};

template<size_t N>
struct to_fizzbuzz
    : mpl::eval_if_c<N % 15 == 0,
        fizzbuzz,
        mpl::eval_if_c<N % 3 == 0,
            fizz,
            mpl::eval_if_c<N % 5 == 0, buzz, to_string<mpl::size_t<N>>>
        >
    >::type
{};

template<size_t N>
struct fizzbuzz_string;

template<size_t N>
struct fizzbuzz_string_impl {
    typedef typename mpl::copy<
        typename to_fizzbuzz<N>::type,
        mpl::back_inserter<
            typename fizzbuzz_string<N-1>::type
        >
    > type;
};

template<size_t N>
struct fizzbuzz_string
    : mpl::copy<
        typename to_fizzbuzz<N>::type,
        mpl::back_inserter<
            typename mpl::eval_if_c<
                N==1,
                mpl::string<>,
                fizzbuzz_string<N-1>
            >::type
        >
    >::type
{};

template<size_t N>
struct fizzbuzz_string2
    : mpl::copy<
        typename to_fizzbuzz<N>::type,
        mpl::back_inserter<
            fizzbuzz_string2<N-1>
        >
    >::type
{};

template<>
struct fizzbuzz_string2<0>
    : mpl::string<>
{};

template<class String>
void println()
{
    std::cout << mpl::c_str<typename String::type>::value << std::endl;
}


int main()
{
    println<fizzbuzz>();
    // println<typename mpl::push_back<mpl::string<'hoge'>, mpl::char_<'!'>>::type>();

    println<typename to_string_impl<1234>::type>();
    println<typename fizzbuzz_string<10>::type>();
    return 0;
}
