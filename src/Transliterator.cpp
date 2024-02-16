#include "Transliterator.h"

#include <iostream>

Transliterator::Transliterator() {
    initializeMap();
}

void Transliterator::initializeMap() {
    // Vowels
    transliterationMap["o"] = L"অ";
    transliterationMap["a"] = L"আ";
    transliterationMap["i"] = L"ই";
    transliterationMap["I"] = L"ঈ";
    transliterationMap["u"] = L"উ";
    transliterationMap["U"] = L"ঊ";
    transliterationMap["ri"] = L"ঋ";
    transliterationMap["e"] = L"এ";
    transliterationMap["oi"] = L"ঐ";
    transliterationMap["O"] = L"ও";
    transliterationMap["ou"] = L"ঔ";
    // consonants
    transliterationMap["k"] = L"ক";
    transliterationMap["kh"] = L"খ";
    transliterationMap["g"] = L"গ";
    transliterationMap["gh"] = L"ঘ";
    transliterationMap["Ng"] = L"ঙ";
    transliterationMap["c"] = L"চ";
    transliterationMap["ch"] = L"ছ";
    transliterationMap["j"] = L"জ";
    transliterationMap["J"] = L"জ";
    transliterationMap["jh"] = L"ঝ";
    transliterationMap["NG"] = L"ঞ";
    transliterationMap["T"] = L"ট";
    transliterationMap["Th"] = L"ঠ";
    transliterationMap["D"] = L"ড";
    transliterationMap["Dh"] = L"ঢ";
    transliterationMap["N"] = L"ণ";
    transliterationMap["t"] = L"ত";
    transliterationMap["th"] = L"থ";
    transliterationMap["d"] = L"দ";
    transliterationMap["dh"] = L"ধ";
    transliterationMap["n"] = L"ন";
    transliterationMap["p"] = L"প";
    transliterationMap["ph"] = L"ফ";
    transliterationMap["f"] = L"ফ";
    transliterationMap["b"] = L"ব";
    transliterationMap["bh"] = L"ভ";
    transliterationMap["v"] = L"ভ";
    transliterationMap["m"] = L"ম";
    transliterationMap["z"] = L"য";
    transliterationMap["r"] = L"র";
    transliterationMap["l"] = L"ল";
    transliterationMap["sh"] = L"শ";
    transliterationMap["S"] = L"শ";
    transliterationMap["SH"] = L"ষ";
    transliterationMap["s"] = L"স";
    transliterationMap["h"] = L"হ";
    transliterationMap["R"] = L"ড়";
    transliterationMap["Rh"] = L"ঢ়";
    transliterationMap["y"] = L"য়";
    transliterationMap["Y"] = L"য়";
    transliterationMap["tth"] = L"ৎ";
    transliterationMap["ng"] = L"ং";
    transliterationMap[":"] = L"ঃ";
    transliterationMap["^"] = L" ঁ";
}

bool Transliterator::isVowel(const std::string& sequence) const {
    // List of single-character and multi-character vowels
    static const std::unordered_set<std::string> vowels = {
        "a", "i", "I", "u", "U", "e",
        "O"  // Single-character vowels
        "ri",
        "oi", "ou"  // Multi-character vowels
    };

    return vowels.find(sequence) != vowels.end();
}

// Adjusted to accept std::string for compatibility with multi-character strings
std::wstring Transliterator::getVowelModifier(const std::string& vowel) const {
    std::unordered_map<std::string, std::wstring> vowelModifiers = {
        {"a", L"া"},
        {"i", L"ি"},
        {"I", L"ী"},
        {"u", L"ু"},
        {"U", L"ূ"},
        {"ri", L"ৃ"},
        {"e", L"ে"},
        {"oi", L"ৈ"},
        {"O", L"ো"},
        {"ou", L"ৌ"},
    };

    auto it = vowelModifiers.find(vowel);
    if (it != vowelModifiers.end()) {
        return it->second;
    }
    return L"";  // Return empty if not found
}

std::string Transliterator::toBangla(const std::string& input) {
    std::wstring result;
    bool isNewSound = true;  // Indicates the start of a new sound segment or after a sound-breaker 'o'.

    for (size_t i = 0; i < input.length();) {
        std::string currentKey = (i + 1 < input.length()) ? input.substr(i, 2) : input.substr(i, 1);

        if (transliterationMap.find(currentKey) == transliterationMap.end()) {
            // If two-character key not found or not valid, use single character.
            currentKey = input.substr(i, 1);
        }

        if (currentKey == "o") {
            if (isNewSound) {
                // Treat 'o' at the start of a new sound or word as a standalone vowel.
                result += transliterationMap[currentKey];
                isNewSound = false;  // No longer at the start of a new sound.
            } else {
                // 'o' acts as a sound break, indicating a new sound segment.
                isNewSound = true;
            }
            i++;
            continue;
        }

        if (isNewSound) {
            // Handle any character at the start of a new sound as a standalone character.
            if (transliterationMap.find(currentKey) != transliterationMap.end()) {
                result += transliterationMap[currentKey];
                isNewSound = false;  // A new sound has been processed.
            }
            i += currentKey.length();
            continue;
        } else {
            // Process vowels (including "O" as a modifier) following consonants.
            std::wstring modifier;
            if (isVowel(currentKey) && transliterationMap.find(currentKey) != transliterationMap.end()) {
                modifier = getVowelModifier(currentKey);
                if (!modifier.empty()) {
                    result += modifier;
                    i += currentKey.length();
                    continue;
                }
            }
        }

        // For consonants or unhandled characters when not starting a new sound.
        /* if (transliterationMap.find(currentKey) != transliterationMap.end()) {
            result += transliterationMap[currentKey];
            i += currentKey.length();
        } else {
            i++;  // Skip unrecognized characters or move to the next character.
        } */

        if (transliterationMap.find(currentKey) != transliterationMap.end()) {
            if (!isNewSound && currentKey == "O") {
                // Special handling for 'O' after a consonant as a modifier
                result += getVowelModifier(currentKey);
            } else {
                result += transliterationMap[currentKey];
            }
            i += currentKey.length();
        } else {
            i++;  // Move to the next character for unrecognized sequences
        }

        isNewSound = false;  // Reset for the next iteration unless explicitly set.
    }

    // Conversion from wstring to UTF-8 string for output.
    std::wstring_convert<std::codecvt_utf8<wchar_t>, wchar_t> converter;
    return converter.to_bytes(result);
}
