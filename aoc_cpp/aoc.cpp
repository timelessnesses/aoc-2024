#include <iostream>
#include <vector>
#include <sstream>
#include <algorithm>
#include <numeric>
#include <fstream>
#include <cassert>

std::vector<std::string> split(std::string input, char delim = '\n') {
    std::vector<std::string> output;
    std::stringstream stream(input);
    std::string line;
    while (std::getline(stream, line, delim)) {
        output.push_back(line);
    }
    return output;
}

class Day1 {
    public:
        std::tuple<std::vector<long long int>, std::vector<long long int>> parse(std::string input) {
            auto left = std::vector<long long int>();
            auto right = std::vector<long long int>();
            std::stringstream stream(input);
            std::string line;
            for (std::string line: split(input)) {
                auto splitted = split(line, ' ');
                left.push_back(std::stoi(splitted[0]));
                right.push_back(std::stoi(splitted.back()));
            }
            std::sort(left.begin(), left.end());
            std::sort(right.begin(), right.end());
            return std::make_tuple(left, right);
        }
        
        long long int solve_part_1(std::string input) {
            auto [left, right] = parse(input);
            auto sum = 0;
            for (auto i = 0; i < left.size(); i++) {
                sum += std::abs(left[i] - right[i]);
            }
            return sum;
        }
        Day1() {}
};



int main() {
    Day1 day1;
    auto stream = std::ifstream("../input/day1.txt");    
    std::string input((std::istreambuf_iterator<char>(stream)), std::istreambuf_iterator<char>());
    auto out = day1.solve_part_1(input);
    std::cout << out << std::endl;
    assert(out == 2192892);
    return 0;
}