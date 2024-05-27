//Preston Engler
//day3_2.cpp

#include <string>
#include <vector>
#include <iostream>
#include <fstream>

using namespace std;

vector<string> input;

bool isDigit(char c) 
{
  if(c >= '0' && c <= '9') return true;
  return false;
}

int getNumber(int row, int col)
{
  int begin = 0;
  int end = 0;
  for(int i = col; isDigit(input[row][i]) && i < (int) input[row].size(); i++)
  {
    end = i;
  }

  for(int i = col; isDigit(input[row][i]) && i >= 0; i--)
  {
    begin = i;
  }

  /* if(begin == end)
  {
    cout << "Error! getNumber begin == end, row " << row << " col " << col << endl;
    cout << input[row][col] << endl;
  } */

  return stoi( input[row].substr(begin,end-begin+1) );
}

int gearRatio(int i, int j)
{
  vector<int> nums;

  //Weird logic below is necessary since one digit can be multiple slots on the top/bottom 
  //yet there can be 2 digits on the top/bottom if there's a space in between

  //top right
  if(isDigit(input[i-1][j+1]) )
  {
    nums.push_back(getNumber(i-1, j+1));
    //no top and top left
    if(!isDigit(input[i-1][j]) && isDigit(input[i-1][j-1])) nums.push_back(getNumber(i-1, j-1));
  }
  //top left
  else if(isDigit(input[i-1][j-1])) nums.push_back(getNumber(i-1, j-1));
  //top
  else if(isDigit(input[i-1][j])) nums.push_back(getNumber(i-1, j));

  //bottom right
  if(isDigit(input[i+1][j+1]) )
  {
    nums.push_back(getNumber(i+1, j+1));
    //no bottom and bottom left
    if(!isDigit(input[i+1][j]) && isDigit(input[i+1][j-1])) nums.push_back(getNumber(i+1, j-1));
  }
  //bottom left
  else if(isDigit(input[i+1][j-1])) nums.push_back(getNumber(i+1, j-1));
  //bottom
  else if(isDigit(input[i+1][j])) nums.push_back(getNumber(i+1, j));

  if(isDigit(input[i][j-1]) ) nums.push_back(getNumber(i, j-1)); //left
  if(isDigit(input[i][j+1]) ) nums.push_back(getNumber(i, j+1)); //right


  if(nums.size() > 2)
  {
    cout << "gear nums > 2 at: row " << i << ", col " << j << endl;
    return 0;
  }

  if(nums.size() < 2)
  {
    cout << "Not enough part nums, row " << i << ", col " << j << endl;
    return 0;
  }

  cout << nums[0] << ", " << nums[1] << endl;
  return nums[0] * nums[1];
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

  long int result = 0;

  for(int i = 0; instream.peek() != EOF; i++)
  {
    string inputLine;
    getline(instream, inputLine);
    input.push_back(inputLine);
  }

  for(int i = 0; i < (int) input.size(); i++)
  {
    for(int j = 0; j < (int) input[i].size(); j++)
    {
      if(input[i][j] == '*')
      {
        cout << "\tFound gear: " << i << ", " << j << endl;
        result += gearRatio(i, j);
      }

    }
  }

  cout << result << endl;
   
  return 0;
}
