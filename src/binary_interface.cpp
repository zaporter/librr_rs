#include "binary_interface.hpp"
namespace rr {

/* static ReplaySession::Flags session_flags(const ReplayFlags& flags) { */
/*   ReplaySession::Flags result; */
/*   result.redirect_stdio = flags.redirect; */
/*   result.redirect_stdio_file = flags.tty; */
/*   result.share_private_mappings = flags.share_private_mappings; */
/*   result.cpu_unbound = flags.cpu_unbound; */
/*   return result; */
/* } */
std::unique_ptr<BinaryInterface> new_binary_interface(int64_t target_event,rust::String trace_dir_rust) {
  std::string trace_dir = std::string(trace_dir_rust);
  return new_binary_interface_librr(target_event,trace_dir);
}
rust::Vec<GdbThreadId> get_thread_list_from_rust(const BinaryInterface& interface){
  rust::Vec<GdbThreadId> tids;
  auto cpp_tids = interface.get_thread_list();
  for (auto tid : cpp_tids){
    tids.push_back(tid);
  }

  return tids;
}

}//end namespace rr
