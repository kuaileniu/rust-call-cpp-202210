#include <iostream>

extern "C" {
  void hello() {
    std::cout << "Hello, world from CPP!" << std::endl;
  }
}
