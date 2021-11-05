#pragma once

namespace Luau {
    struct ConfigResolver;
    struct FileResolver;
    struct Frontend;
}

struct LuauWrapper {
    LuauWrapper();
    ~LuauWrapper();

    void check(const char* filename);

private:
    Luau::ConfigResolver* const configResolver;
    Luau::FileResolver* const fileResolver;

    Luau::Frontend* const fe;
};
