
#include "wrapper.hpp"

#include <optional>

#include "Luau/Frontend.h"

using namespace Luau;

struct WrapperFileResolver : FileResolver {
    std::optional<SourceCode> readSource(const ModuleName& name) override {
        FILE* f = fopen(name.c_str(), "r");
        if (!f)
            return std::nullopt;

        fseek(f, 0, SEEK_END);
        const size_t size = ftell(f);
        fseek(f, 0, SEEK_SET);

        SourceCode result;
        result.source.resize(size);
        fread(result.source.data(), size, 1, f);

        fclose(f);

        return result;
    }

    bool moduleExists(const ModuleName& name) const override {
        return true;
    }

    std::optional<ModuleName> fromAstFragment(AstExpr* expr) const override {
        return std::nullopt;
    }

    ModuleName concat(const ModuleName& lhs, std::string_view rhs) const override {
        return lhs + "/" + std::string{rhs};
    }

    std::optional<ModuleName> getParentModuleName(const ModuleName& name) const override {
        return std::nullopt;
    }

    std::optional<std::string> getHumanReadableModuleName_(const ModuleName& name) const {
        return name;
    }

    std::optional<std::string> getEnvironmentForModule(const ModuleName& name) const override {
        return std::nullopt;
    }
};

LuauWrapper::LuauWrapper()
    : configResolver(new NullConfigResolver)
    , fileResolver(new WrapperFileResolver)
    , fe(new Frontend(fileResolver, configResolver))
{
}

LuauWrapper::~LuauWrapper() {
    delete fe;
    delete configResolver;
    delete fileResolver;
}

void LuauWrapper::check(const char* filename) {
    auto result = fe->check(filename);

    for (const auto& e: result.errors) {
        printf("Error: %d,%d: %s\n", e.location.begin.line, e.location.begin.column, toString(e).c_str());
    }
}
