#include <iostream>

extern "C"
{
  void hello()
  {
    std::cout << "Hello, world from CPP!" << std::endl;
  }

  int sum(int num1, int num2)
  {
    return num1 + num2;
  }
}
