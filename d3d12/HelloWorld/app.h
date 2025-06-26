#pragma once

#include <Windows.h>
#include <cstdint>

class App {
  public:
    App(uint32_t const width, uint32_t const height);
    ~App();
    void run();

  private:
    bool init_app();
    void term_app();
    bool init_window();
    void term_window();
    void main_loop();

    static LRESULT CALLBACK window_proc(HWND hwnd, UINT msg, WPARAM wp, LPARAM lp);

    HINSTANCE hinst_;
    HWND hwnd_;
    uint32_t width_;
    uint32_t height_;
};
