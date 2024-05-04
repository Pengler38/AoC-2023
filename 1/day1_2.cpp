/* day1_2.cpp
 * https://adventofcode.com/2023/day/1
 *
 * Preston Engler
 * ppengler@mtu.edu
 *
 * MotD: actually on second thought this program is too easily parallelized,
 *  I'm hardly doing concurrent programming
 *
 */

#include <iostream>
#include <thread>
#include <string>
#include <cctype>
#include <vector>

#define NUM_THREADS 8

int inputs;
int* num;
std::vector<std::string> inputList = std::vector<std::string>();

//handle
//param:
//  int thread: which thread it is, expected in the range [0, NUM_THREADS)
//return: void
//
//Takes care of inputs in the range [start, end), finding first and last num and 
//combining them to get the number, which is placed in int num[].
void handle(int start, int end)
{
  std::string s(2, 'x');


  for(int i = start; i < end; i ++)
  {
    bool first = true;
    for(unsigned int j = 0; j < inputList[i].size(); j++)
    {
      if( std::isdigit( static_cast<unsigned char>(inputList[i][j]) ))
      {
        if(first) {
          s[0] = inputList[i][j];
          s[1] = inputList[i][j];
          first = false;
        }
        else s[1] = inputList[i][j];
      }
    }

    num[i] = std::stoi(s);
  }

}

//Main Function, enough said
int main()
{
  //Get inputs from cin
  int a = 0;
  do{
    std::string s;
    getline(std::cin, s);
    inputList.push_back(s);
    a++;
  }while(inputList[a-1].size() != 0);

  //subtract 1 from the inputs since it has 1 extra string which is empty
  inputs = inputList.size()-1;
  num = new int[inputs];

  if(inputs < NUM_THREADS)
  {
    //Don't bother with threads, just do it yourself
    handle(0, inputs);
  }

  //Divvy up the input chunks for each thread to do and start them off
  std::thread* t[NUM_THREADS];
  for(int i = 0; i < NUM_THREADS; i++)
  {
    t[i] = new std::thread(handle, i*inputs/NUM_THREADS, (i+1)*inputs/NUM_THREADS);
  }


  //Join the threads, waiting for them to finish
  for(int i = 0; i < NUM_THREADS; i++)
  {
    t[i]->join();
  }


  //Add up all the numbers
  int ret = 0;
  for(int i = 0; i < inputs-1; i++)
  {
    ret += num[i];
    //std::cout << num[i] << std::endl;
  }

  std::cout << ret << std::endl;

  //It's good practice to delete what I've allocated
  for(int i = 0; i < NUM_THREADS; i++)
  {
    delete t[i];
  }
  delete[] num;
}

//experiment w/ making a class like this, goal is to statically initialize arr?
//maybe just stick with vector
typedef struct my_string{
  int len;
  char* arr;
} str;


//Base function idea: have an array of strings, first string has all the first chars of digits (o, t, t, f, etc.)
//use that to check if the first char matches, then go down the word to see if the whole word matches
bool is_digit()
{

}
