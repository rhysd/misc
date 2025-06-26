#include "App.h"

int wmain(int argc, wchar_t **argv, wchar_t **envp) {
    App app(960, 540);
    app.run();
    return 0;
}
