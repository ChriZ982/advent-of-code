FLAGS=-std=gnu++23 -O3 -Wall -Wextra -Wconversion -s -flto -I$(shell pwd)/vcpkg/installed/x64-linux/include/ -L$(shell pwd)/vcpkg/installed/x64-linux/lib/

deps:
	./vcpkg/bootstrap-vcpkg.sh -disableMetrics
	# ./vcpkg/vcpkg install fmt

day-1-part1: day-1/part1.cpp
	g++ $(FLAGS) -o build/day-1-part1 day-1/part1.cpp -lfmt
	./build/day-1-part1

day-1-part2: day-1/part2.cpp
	g++ $(FLAGS) -o build/day-1-part2 day-1/part2.cpp
	./build/day-1-part2

day-2-part1: day-2/part1.cpp
	g++ $(FLAGS) -o build/day-2-part1 day-2/part1.cpp
	./build/day-2-part1

day-2-part2: day-2/part2.cpp
	g++ $(FLAGS) -o build/day-2-part2 day-2/part2.cpp
	./build/day-2-part2
