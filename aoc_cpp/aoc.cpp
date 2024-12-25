#include <iostream>
#include <fstream>
#include <cassert>
import day1;
import day2;

int main() {
    Day2 day2;
    auto stream = std::ifstream("../input/day1.txt");    
    std::string input((std::istreambuf_iterator<char>(stream)), std::istreambuf_iterator<char>());
    auto out = day2.solve_part_1(input);
    std::cout << out << std::endl;
    return 0;
}