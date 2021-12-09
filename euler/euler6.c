#include <stdio.h>

int main()
{
    int i,j;
    int result = 0;
    for(i=1; i<=100; ++i){
        for(j=1; j<=100; ++j){
            if(i!=j) result += i*j;
        }
    }

    printf("%d\n",result);
    return 0;
}