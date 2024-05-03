#Preston Engler (ppengler@mtu.edu)

flags = -Wall -Wextra -O1

All: 1/day1.exe

1/day1.exe: 1/day1.cpp
	clang $^ $(flags) -o $@

debug: 1/day1.cpp
	clang $^ -g -o 1/day1.exe
