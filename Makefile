#Preston Engler (ppengler@mtu.edu)

flags = -Wall -Wextra -O1

all: 1/day1.exe 1/day1_2.exe 2/day2.exe 2/day2_2.exe 3/day3.exe

1/day1.exe: 1/day1.cpp
	g++ $^ $(flags) -o $@

1/day1_2.exe: 1/day1_2.cpp
	g++ $^ $(flags) -o $@

2/day2.exe: 2/day2.cpp
	g++ $^ $(flags) -o $@

2/day2_2.exe: 2/day2_2.cpp
	g++ $^ $(flags) -o $@

3/day3.exe: 3/day3.cpp
	g++ $^ $(flags) -o $@

debug: 1/day1.cpp
	g++ $^ -g -o 1/day1.exe
