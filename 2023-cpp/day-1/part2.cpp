#include <format>

int main() {
    puts(std::format("Hello, {} from {}!", "world", "part2").c_str());
}
