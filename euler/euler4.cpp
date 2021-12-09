#include <iostream>
#include <string>

bool is_palindrome(int const i)
{
    auto s = std::to_string(i);
    for(auto f=s.begin(), l=s.end()-1; f<=l; ++f, --l){
        if(*f!=*l) return false;
    }
    return true;
}

int main()
{
    int max_val = 0;
    for(int i=100; i<1000; ++i){
        for(int j=i; j<1000; ++j){
            if(is_palindrome(i*j)){
                max_val = std::max(max_val, i*j);
            }
        }
    }
    std::cout << max_val << std::endl;
    return 0;
}
