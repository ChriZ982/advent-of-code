#include <algorithm>
#include <cassert>
#include <fstream>
#include <map>
#include <ranges>
#include <string>
#include <string_view>

auto read_file(const std::string_view path) -> std::string {
    std::ifstream file(path.data());
    std::string input{std::istreambuf_iterator<char>(file), std::istreambuf_iterator<char>()};
    return input.substr(0, input.find_last_not_of('\n') + 1);
}

template <std::size_t N, typename R, typename T>
[[nodiscard]] auto split(R&& range, T&& delimiter) -> std::array<std::string_view, N> {
    auto split = std::views::split(std::forward<R>(range), std::forward<T>(delimiter)) | std::views::transform([](auto&& rng) {
                     return std::string_view{std::begin(rng), std::end(rng)};
                 });
    std::array<std::string_view, N> result;
    std::ranges::copy(split, result.begin());
    return result;
}

const auto max_colors = std::map<std::string_view, int32_t>{
    {"red", 12},
    {"green", 13},
    {"blue", 14},
};

auto check_single_round(std::ranges::subrange<std::string_view::const_iterator> cubes) -> bool {
    return std::ranges::all_of(std::views::split(cubes, ','), [](auto cube) {
        const auto [count_str, color_str] = split<2>(cube | std::views::drop(1), ' ');
        return std::stoi(count_str.data()) <= max_colors.at(color_str);
    });
}

auto solve(const std::string_view input) -> int32_t {
    return std::ranges::fold_left(std::views::split(input, '\n'), 0, [](auto sum, auto line) -> int32_t {
        const auto [game_str, subset_str] = split<2>(line, ':');
        if (std::ranges::all_of(std::views::split(subset_str, ';'), check_single_round)) {
            return sum + std::stoi(split<2>(game_str, ' ')[1].data());
        }
        return sum;
    });
}

constexpr auto test_input = R"(Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green)";

auto main() -> int {
    assert(solve(test_input) == 8 && "Solving with test input failed!");
    printf("Test passed!\n");

    printf("Solution: %d\n", solve(read_file("day-02/input.txt")));
    return EXIT_SUCCESS;
}
