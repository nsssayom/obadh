#ifndef TRANSLITERATOR_H
#define TRANSLITERATOR_H

#include <string>
#include <unordered_map>
#include <unordered_set>
#include <codecvt>
#include <locale>

class Transliterator {
public:
    Transliterator();
    std::string toBangla(const std::string& input);

private:
    std::unordered_map<std::string, std::wstring> transliterationMap;
    void initializeMap();
    bool isVowel(const std::string&) const;
    std::wstring getVowelModifier(const std::string&) const;
};

#endif // TRANSLITERATOR_H
