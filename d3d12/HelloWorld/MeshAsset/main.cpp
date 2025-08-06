#include "App.h"
#if defined(DEBUG) || defined(_DEBUG)
#define _CRTDBG_MAP_ALLOC
#include <crtdbg.h>
#endif
#include <cstdio>

int wmain(int argc, wchar_t **argv, wchar_t **envp) {
#if defined(DEBUG) || defined(_DEBUG)
    _CrtSetDbgFlag(_CRTDBG_ALLOC_MEM_DF | _CRTDBG_LEAK_CHECK_DF);
    // For example, when runtime reports the 148th allocation leaked, the following line sets the break point on the allocation
    // _CtrSetBreakAlloc(148);
#endif
    App app(960, 540);
    if (!app.run()) {
        return 1;
    }
    return 0;
}
