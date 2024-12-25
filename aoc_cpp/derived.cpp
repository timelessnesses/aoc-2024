import <string>

export module derived;

export template <typename T>
class Solution {
    public:
        virtual T solve_part_1(std::string input) = 0;
        virtual T solve_part_2(std::string input) = 0;
};