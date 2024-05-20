#include <iostream>
#include <fstream>
#include <regex>
#include <string>

using namespace std;

#ifdef DEBUG
  #define debug(...) __VA_ARGS__
#else
  #define debug(...)
#endif

//const int red = 12;
//const int green = 13;
//const int blue = 14;


int minCubesPossible(const string in, regex r, string s)
{
  (void) s;
  debug(cout << endl << s << " ");

  string search(in);

  int min = 0;

  for(smatch m; regex_search(search, m, r);)
  {
    debug(cout << '\t' << m[1]);
    if(stoi(m[1]) > min) min = stoi(m[1]);
    search = m.suffix(); //this might cause issues? memory shenanigans when put before the stoi! corrupts m[1]! happens w/ clang and gcc
  }

  return min;
}



int main()
{
  ifstream inputStream;
  inputStream.open("input.txt");
  string input;

  int result = 0;

  regex blueRegex("([0-9]+) blue");
  regex redRegex("([0-9]+) red");
  regex greenRegex("([0-9]+) green");

  //One failing here:
  //potentially could have been more ideal to have id be taken from the input rather than assumed to increment 1 per line.
  for(int id = 1; inputStream.peek() != EOF; id++)
  {
    getline(inputStream, input);
    debug(cout << input << endl);

    //Find min cubes, multiply into pow
    int pow = 1;
    pow *= minCubesPossible(input, blueRegex, "blue");
    pow *= minCubesPossible(input, redRegex, "red");
    pow *= minCubesPossible(input, greenRegex, "green");

    result += pow;
  }


  cout << result << endl;


  inputStream.close();
  return 0;
}

