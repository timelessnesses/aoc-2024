#include <algorithm>
#include <iostream>
import derived;
import utils;
export module day2;
class Day2: public Solution<long long int> {
    public:
        std::vector<Data> parse(std::string input) {}
        long long int solve_part_1(std::string input) {
            auto parsed = parse(input);
            return parsed.size();
        }

        long long int solve_part_2(std::string input) {
            auto parsed = parse(input);
            return 0;
        }
};

class Data {
    public:
        std::vector<int> ints;
        Data(std::vector<int> ints): ints(ints) {}
        bool is_safe() {
            auto check_is_down = ints[0] > ints[1];
            auto available_choices = std::vector<int>({ints[0], ints[1], ints[2]});
            for (auto i: window(ints, 2)) {
                std::cout << i[0] << " " << i[1] << std::endl;
            }
        }
};

std::vector<std::vector<int>> window(std::vector<int> input, int size) {
    std::vector<std::vector<int>> output;
    for (auto i = 0; i < input.size(); i += size) {
        output.push_back(
            std::vector<int>(
                input.begin() + i,
                input.begin() + i + size));
    }
    return output;
}