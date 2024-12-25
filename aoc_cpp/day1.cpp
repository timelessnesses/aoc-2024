#include <algorithm>
import derived;
import utils;
export module day1;
class Day1: public Solution<long long int> {
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

        long long int solve_part_2(std::string input) {
            auto [left, right] = parse(input);
            auto count = 0;
            for (auto i: left) {
                auto filtered = 0;
                for (auto j: right) {
                    if (i == j) {
                        filtered += 1;
                    }
                }
                count += i * filtered;
            }
            return count;
        }
};
