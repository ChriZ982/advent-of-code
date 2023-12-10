#include <algorithm>
#include <cassert>
#include <fstream>
#include <ranges>
#include <string_view>

auto read_file(const std::string_view path) -> std::string {
    std::ifstream file(path.data());
    std::string input{std::istreambuf_iterator<char>(file), std::istreambuf_iterator<char>()};
    return input.substr(0, input.find_last_not_of('\n') + 1);
}

auto first_digit(const auto line) -> int {
    return *std::ranges::find_if(line, [](auto c) { return std::isdigit(c); }) - '0';
}

auto solve(const std::string_view input) -> int {
    return std::ranges::fold_left(std::views::split(input, '\n'), 0, [](auto sum, auto line) -> int {
        return sum + 10 * first_digit(line) + first_digit(std::views::reverse(line));
    });
}

constexpr auto test_input = R"(1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet)";

auto main() -> int {
    assert(solve(test_input) == 142 && "Solving with test input failed!");

    printf("Solution: %d\n", solve(read_file("day-01/input.txt")));
    return EXIT_SUCCESS;
}
