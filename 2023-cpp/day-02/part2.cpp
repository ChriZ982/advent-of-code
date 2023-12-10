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

auto get_max_counts(auto cubes) -> std::array<int32_t, 3> {
    return std::ranges::fold_left(std::views::split(cubes, ','), std::array{0, 0, 0}, [](auto max, auto cube) -> std::array<int32_t, 3> {
        const auto [count, color] = split<2>(cube | std::views::drop(1), ' ');
        return {std::max(max[0], color == "red" ? std::stoi(count.data()) : 0),
                std::max(max[1], color == "green" ? std::stoi(count.data()) : 0),
                std::max(max[2], color == "blue" ? std::stoi(count.data()) : 0)};
    });
}

auto solve(const std::string_view input) -> int32_t {
    return std::ranges::fold_left(std::views::split(input, '\n'), 0, [](auto sum, auto line) {
        return sum + std::ranges::fold_left(
                         std::ranges::fold_left(split<2>(line, ':')[1] | std::views::split(';'), std::array{0, 0, 0},
                                                [](auto max, auto cubes) -> std::array<int32_t, 3> {
                                                    auto [red, green, blue] = get_max_counts(cubes);
                                                    return {std::max(max[0], red), std::max(max[1], green), std::max(max[2], blue)};
                                                }),
                         1, std::multiplies{});
    });
}

constexpr auto test_input = R"(Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green)";

auto main() -> int {
    assert(solve(test_input) == 2286 && "Solving with test input failed!");
    printf("Test passed!\n");

    printf("Solution: %d\n", solve(read_file("day-02/input.txt")));
    return EXIT_SUCCESS;
}
