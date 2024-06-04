#Preston Engler (ppengler@mtu.edu)

flags = -Wall -Wextra -O1

all: 1/day1.exe 1/day1_2.exe 2/day2.exe 2/day2_2.exe 3/day3.exe 3/day3_2.exe 4/day4.exe

1/day1.exe: 1/day1.cpp

1/day1_2.exe: 1/day1_2.cpp

2/day2.exe: 2/day2.cpp

2/day2_2.exe: 2/day2_2.cpp

3/day3.exe: 3/day3.cpp

3/day3_2.exe: 3/day3_2.cpp

4/day4.exe: 4/day4.rs

%.exe: %.cpp
	g++ $^ $(flags) -o $@

%.exe: %.rs
	rustc $^ -o $@

debug: 1/day1.cpp
	g++ $^ -g -o 1/day1.exe
