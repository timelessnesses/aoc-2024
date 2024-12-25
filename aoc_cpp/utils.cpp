import <vector>
import <string>
import <sstream>

export module utils;

export std::vector<std::string> split(std::string input, char delim = '\n') {
    std::vector<std::string> output;
    std::stringstream stream(input);
    std::string line;
    while (std::getline(stream, line, delim)) {
        output.push_back(line);
    }
    return output;
}