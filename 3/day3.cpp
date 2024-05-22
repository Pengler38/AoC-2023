//Preston Engler
//day3.cpp

//TODO: 
//bug 1: it is currently including the final digit, 975, which it should not
//bug 2: 482388 is the current answer
//  excepting bug 1 it would be 481413 BUT 482388 is already too low of an answer!
//  this means there is a hidden bug currently :(

#include <string>
#include <vector>
#include <iostream>
#include <fstream>

using namespace std;

bool isDigit(char c) 
{
  if(c >= '0' && c <= '9') return true;
  return false;
}

//isPartNumber takes the number ranging from [start,end], start and end are both inclusive
bool isPartNumber(vector<string> input, int row, int start, int end)
{
  //Directional bools
  bool top = (row > 0) ? true : false;
  bool bottom = (row < (int) input.size()-1) ? true : false;
  bool left = (start > 0) ? true : false;
  bool right = (end < (int) input[row].size()-1) ? true : false;

  //TODO can a number be adjacent to a number? May need to check if it's not '.' and not a digit!

  //4 corners
  if(top && left && input[row-1][start-1] != '.') return true;
  if(top && right && input[row-1][end+1] != '.') return true;
  if(bottom && left && input[row+1][start-1] != '.') return true;
  if(bottom && right && input[row+1][end+1] != '.') return true;

  //left/right
  if(left && input[row][start-1] != '.') return true;
  if(left && input[row][start-1] != '.') return true;

  //top
  for(int i = start; top && i <= end; i++)
  {
    if(input[row-1][i] != '.') return true;
  }

  //bottom
  for(int i = start; bottom && i <= end; i++)
  {
    if(input[row+1][i] != '.') return true;
  }

  return false;
}

//Parse takes the number ranging from [start,end], start and end are both inclusive
int parse(vector<string> input, int row, int start, int end)
{
  return stoi( input[row].substr(start, end-start+1) );
}

int main () {
  //Load everything from input.txt
  ifstream instream;
  instream.open("input.txt");

  int result = 0;

  vector<string> input;
  for(int i = 0; instream.peek() != EOF; i++)
  {
    string inputLine;
    getline(instream, inputLine);
    input.push_back(inputLine);
  }

  for(int i = 0; i < (int) input.size(); i++)
  {
    bool inDigitState = false;
    int start = 0;
    for(int j = 0; j < (int) input[i].size(); j++)
    {
      if(inDigitState && !isDigit(input[i][j]))
      {
        inDigitState = false;
        //Check if the digit is adjacent to a symbol and parse it in if so
        if(isPartNumber(input, i, start, j-1))
        {
          cout << '\t' << parse(input, i, start, j-1) << endl;
          result += parse(input, i, start, j-1);
        }
      }
      else if(!inDigitState && isDigit(input[i][j]))
      {
        start = j;
        inDigitState = true;
      }

    }
  }

  cout << result << endl;
   
  return 0;
}
