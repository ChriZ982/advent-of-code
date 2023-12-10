#include <algorithm>
#include <cassert>
#include <cstdint>
#include <fstream>
#include <ranges>
#include <string_view>

using namespace std::literals;

auto read_file(const std::string_view path) -> std::string {
    std::ifstream file(path.data());
    std::string input{std::istreambuf_iterator<char>(file), std::istreambuf_iterator<char>()};
    return input.substr(0, input.find_last_not_of('\n') + 1);
}

auto first_digit(const auto line) -> std::tuple<int64_t, int32_t> {
    const auto first = *std::ranges::find_if(std::views::enumerate(line), [](auto c) { return std::isdigit(std::get<1>(c)); });
    return {std::get<0>(first), std::get<1>(first) - '0'};
}

constexpr auto num_words =
    std::array{std::pair{"one"sv, 1}, std::pair{"two"sv, 2},   std::pair{"three"sv, 3}, std::pair{"four"sv, 4}, std::pair{"five"sv, 5},
               std::pair{"six"sv, 6}, std::pair{"seven"sv, 7}, std::pair{"eight"sv, 8}, std::pair{"nine"sv, 9}};

auto index_of_word(const auto line, const std::string_view word, const auto transform) -> int64_t {
    return std::get<0>(*std::begin(
        std::ranges::search(std::views::enumerate(transform(line)), transform(word), [](auto a, auto b) { return std::get<1>(a) == b; })));
}

auto first_word(const auto line, const auto transform) -> std::tuple<int64_t, int32_t> {
    const auto first_word =
        std::ranges::min_element(num_words, {}, [&](const auto word) { return index_of_word(line, word.first, transform); });
    const int64_t index = index_of_word(line, first_word->first, transform);
    return {index, first_word->second};
}

auto solve(const std::string_view input) -> int32_t {
    return std::ranges::fold_left(std::views::split(input, '\n'), 0, [](auto sum, auto line) -> int {
        const auto [fw_idx, fw_val] = first_word(line, std::identity{});
        const auto [lw_idx, lw_val] = first_word(line, std::views::reverse);
        const auto [fd_idx, fd_val] = first_digit(line);
        const auto [ld_idx, ld_val] = first_digit(std::views::reverse(line));
        return sum + 10 * (fw_idx < fd_idx ? fw_val : fd_val) + (lw_idx < ld_idx ? lw_val : ld_val);
    });
}

constexpr auto test_input = R"(two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen)";

auto main() -> int {
    assert(solve(test_input) == 281 && "Solving with test input failed!");

    printf("Solution: %d\n", solve(read_file("day-01/input.txt")));
    return EXIT_SUCCESS;
}
