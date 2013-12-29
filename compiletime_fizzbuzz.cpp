#include <cstddef>
#include <array>
#include <iostream>
#include <type_traits>

#include <boost/preprocessor/repetition/enum.hpp>
#include <boost/preprocessor/facilities/intercept.hpp>
#include <boost/preprocessor/config/limits.hpp>

using std::size_t;

template< class CharT, CharT... Chars >
struct basic_string{

    static const CharT value[sizeof...(Chars)+1];

    constexpr std::array<CharT, sizeof...(Chars)+1>
    to_array() const
    {
        return {{Chars..., '\0'}};
    }
};

template< class CharT, CharT... Chars >
const CharT basic_string<CharT, Chars...>::value[sizeof...(Chars)+1] = {Chars..., '\0'};

template<char... Chars>
using string = basic_string<char, Chars...>;

template<wchar_t... Chars>
using wstring = basic_string<wchar_t, Chars...>;


template< class CharT,
          size_t Value,
          class Acc = basic_string<CharT>,
          bool = (Value < 10) >
struct to_string_from_size_t;

template< class CharT,
          size_t Value,
          CharT... Chars >
struct to_string_from_size_t< CharT, Value, basic_string<CharT, Chars...>, false >
        : to_string_from_size_t< CharT, Value / 10, basic_string< CharT, Value % 10 + '0', Chars... > >{};

template< class CharT,
          size_t Value,
          CharT... Chars >
struct to_string_from_size_t< CharT, Value, basic_string<CharT, Chars...>, true >{
    typedef basic_string< CharT, Value + '0', Chars... > type;
};

template<class T, T Value, class CharT = char>
struct to_string;

template<size_t Value, class CharT>
struct to_string<size_t, Value, CharT> : to_string_from_size_t<CharT, Value>{};


template< class T, class U >
struct joint_;

template< class CharT, CharT... C1, CharT... C2 >
struct joint_< basic_string<CharT, C1...>, basic_string<CharT, C2...> >{
    typedef basic_string<CharT, C1..., C2...> type;
};

template< class Str1, class Str2 >
using joint = typename joint_<Str1, Str2>::type;


template<class T>
struct addnl_;

template< class CharT, CharT... Chars >
struct addnl_< basic_string<CharT, Chars...> >{
    typedef basic_string<CharT, Chars..., '\n'> type;
};

template<class Str>
using addnl = typename addnl_<Str>::type;


template<class CharT, CharT C, class T>
struct cons_;

template<class CharT, CharT C, CharT... Chars>
struct cons_<CharT, C, basic_string<CharT, Chars...>>{
    typedef basic_string<CharT, C, Chars...> type;
};

template<class CharT, CharT C, class T>
using cons = typename cons_<CharT, C, T>::type;


template<size_t N, class T>
struct remove_trailing_null_chars_;

template<class CharT, CharT Head, CharT... Tail>
struct remove_trailing_null_chars_<1, basic_string<CharT, Head, Tail...>>{
    typedef basic_string<CharT> type;
};

template<class CharT, size_t N, CharT Head, CharT... Tail>
struct remove_trailing_null_chars_<N, basic_string<CharT, Head, Tail...>>{
    typedef cons<CharT, Head, typename remove_trailing_null_chars_<N-1, basic_string<CharT, Tail...>>::type> type;
};

template<size_t N, class T>
using remove_trailing_null_chars = typename remove_trailing_null_chars_<N, T>::type;


template<class T>
constexpr T min(T a, T b)
{
    return a < b ? a : b;
}

#define FROM_STRING_LITERAL(lit) FROM_STRING_LITERAL_I(lit)
#define FROM_STRING_LITERAL_I(lit) \
    remove_trailing_null_chars< \
        sizeof(lit) / sizeof(*lit), \
        basic_string< \
            std::decay<decltype(*lit)>::type, \
            BOOST_PP_ENUM(BOOST_PP_LIMIT_REPEAT, FROM_STRING_LITERAL_M, lit) \
        > \
    >
#define FROM_STRING_LITERAL_M(z, i, lit) lit[min((size_t)i, sizeof(lit) / sizeof(*lit) - 1)]


using fizz = FROM_STRING_LITERAL("fizz");
using buzz = FROM_STRING_LITERAL("buzz");


template< size_t Start,
          size_t Last,
          class CharT = char,
          class Acc = basic_string<CharT>,
          size_t Mod3 = Start%3,
          size_t Mod5 = Start%5,
          bool Finish = (Start>=Last) >
struct fizzbuzz{
    typedef Acc type;
};

template< class CharT,
          size_t Start,
          size_t Last,
          class Acc >
struct fizzbuzz<Start, Last, CharT, Acc, 0, 0, false>
        : fizzbuzz<Start+1, Last, CharT, addnl<joint<joint<Acc, fizz>, buzz>> >{};

template< class CharT,
          size_t Start,
          size_t Last,
          class Acc,
          size_t Mod5 >
struct fizzbuzz<Start, Last, CharT, Acc, 0, Mod5, false>
        : fizzbuzz<Start+1, Last, CharT, addnl<joint<Acc, fizz>>>{};

template< class CharT,
          size_t Start,
          size_t Last,
          class Acc,
          size_t Mod3 >
struct fizzbuzz<Start, Last, CharT, Acc, Mod3, 0, false>
        : fizzbuzz<Start+1, Last, CharT, addnl<joint<Acc, buzz>> >{};

template< class CharT,
          size_t Start,
          size_t Last,
          class Acc,
          size_t Mod3,
          size_t Mod5 >
struct fizzbuzz<Start, Last, CharT, Acc, Mod3, Mod5, false>
        : fizzbuzz<Start+1, Last, CharT, addnl<joint<Acc, typename to_string<size_t, Start, CharT>::type>> >{};


int main()
{
    ::setlocale(LC_ALL, "");
    // static_assert(typename fizzbuzz<1, 100, wchar_t>::type().to_array().size() == 480, "");
    std::cout << fizzbuzz<1, 100>::type::value;


    return 0;
}
