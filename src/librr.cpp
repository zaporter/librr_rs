#include "librr.hpp"

namespace rr {
void test_replay_cpp(){
  std::cout <<  "test replay cpp called" << std::endl;
  std::vector<std::string> v = { "-a" , "-g", "1"};
  auto command = ReplayCommand::get();
  command->run(v);
  std::cout << "DONE CONNAMD : " << std::endl;
  std::cout <<  "test replay cpp exiting" << std::endl;

}

} // end namespace rr
