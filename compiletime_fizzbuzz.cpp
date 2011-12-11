template< char ... Str >
struct fb_string{
  static char const* value()
    {
      static char const str[] = {Str..., '\0'};
      return str;
    }
};

template<>
struct fb_string<>{
  static char *value()
    {
      static char const str = {'\0'};
      return str;
    }
};

template<class T1, class T2>
struct joint;

template<char ... S, char ... T>
struct joint<fb_string<S...>, fb_string<T...> > {
  typedef fb_string<S... , T...> result;
};

template<int N, int M>
struct int_to_str{
  typedef typename joint< typename int_to_str<M, M/10>::result,
                          fb_string<'0'+N%10>
                          >::result result;
};

template<int N>
struct int_to_str<N, 0>{
  typedef fb_string<'0'+N> result;
};

template<int N>
struct integer_string{
  typedef typename int_to_str<N, N/10>::result result;
};

template<int V, int Mod3, int Mod5>
struct fizzbuzz_value{
  typedef typename integer_string<V>::result value;
};

template<int V, int Mod5>
struct fizzbuzz_value<V, 0, Mod5>{
  typedef fb_string<'f','i','z','z'> value;
};

template<int V, int Mod3>
struct fizzbuzz_value<V, Mod3, 0>{
  typedef fb_string<'b','u','z','z'> value;
};

template<int V>
struct fizzbuzz_value<V, 0, 0>{
  typedef fb_string<'f','i','z','z','b','u','z','z'> value;
};

template<int N>
struct fizzbuzz_elem{
  typedef typename joint< typename fizzbuzz_value<N, N%3, N%5>::value,
                          fb_string<'\n'>
                          >::result result;
};

template<int N>
struct fizzbuzz{
  typedef typename joint<typename fizzbuzz<N-1>::result,
                         typename fizzbuzz_elem<N>::result
                         >::result result;
};

template<>
struct fizzbuzz<0>{
  typedef fb_string<> result;
};

#include <iostream>

int main()
{
  std::cout << fizzbuzz<4096>::result::value();
  return 0;
}
