#include <windows.h>
#include <mmsystem.h>
#include <iostream>

#pragma comment(lib, "winmm.lib")

HHOOK hHook = nullptr;

LRESULT CALLBACK KeyboardProc(int nCode, WPARAM wParam, LPARAM lParam) {
    if (nCode == HC_ACTION) {
        if (wParam == WM_KEYDOWN) {
            auto *p = (KBDLLHOOKSTRUCT *)lParam;
            std::cout << p->vkCode << std::endl;
            PlaySound(TEXT("F:\\key-tone\\key-tone\\ding.wav"), nullptr, SND_FILENAME | SND_ASYNC);
        }
    }
    return CallNextHookEx(hHook, nCode, wParam, lParam);
}

int main() {
    hHook = SetWindowsHookEx(WH_KEYBOARD_LL, KeyboardProc, GetModuleHandle(nullptr), 0);
    MSG msg;
    while (GetMessage(&msg, nullptr, 0, 0)) {
        TranslateMessage(&msg);
        DispatchMessage(&msg);
    }
    UnhookWindowsHookEx(hHook);
    return 0;
}