#include <stdio.h>
// #include <emscripten/emscripten.h>

void hi_my_js_lib(void);
void my_wait_input(int);

void hello(int i)
{
    printf("worker: wasm: Hello from wasm on worker: %d\n", i);
    my_wait_input(5000);
    hi_my_js_lib();
}

// int main()
// {
//     printf("worker: wasm: main!\n");
//     return 0;
// }
