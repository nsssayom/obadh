#include <iostream>

#include "Transliterator.h"

int main() {
    Transliterator transliterator;
    // Test inputs
    std::string inputs[] = {"a", "i", "u", "k", "g",
                            "ka", "ki", "ku", "ko", "kO", "Ol", "kol", "kOl",
                            "koO", "ila", "nila", "nala", "kina", "kena", "mIna",
                            "kakku", "Taka", "chagol", "banor", "boka", "bOka",
                            "kOl", "kol", "pOl", "mol"};
    for (const auto& input : inputs) {
        std::string output = transliterator.toBangla(input);
        std::cout << "Romanized: " << input << " => Bangla: " << output << std::endl;
    }
    return 0;
}
