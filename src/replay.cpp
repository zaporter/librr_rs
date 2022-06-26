#include "replay.hpp"
#include <iostream>

using namespace std;
using namespace rr;

ReplayingFlags replay_flags_to_rust(rr::ReplayFlags in) {
    ReplayingFlags out = {};
    out.dont_launch_debugger = in.dont_launch_debugger;
    out.goto_event = in.goto_event;
    out.singlestep_to_event = in.singlestep_to_event;
    out.target_process = in.target_process;
    out.dbg_port = in.dbg_port;
    out.dbg_host = rust::String(in.dbg_host);
    out.keep_listening = in.keep_listening;
    rust::Vec<rust::String> gdb_options;
    for (auto option : in.gdb_options){
        gdb_options.push_back(rust::String(option));
    }
    out.gdb_options = gdb_options;
    out.gdb_binary_file_path = rust::String(in.gdb_binary_file_path);
    out.redirect = in.redirect;
    out.cpu_unbound = in.cpu_unbound;
    out.share_private_mappings = in.share_private_mappings;
    out.dump_interval = in.dump_interval;
    out.serve_files = in.serve_files;
    out.tty = rust::String(in.tty);
    return out;
}
rr::ReplayFlags replay_flags_to_cpp(ReplayingFlags in){
    rr::ReplayFlags out;
    out.dont_launch_debugger = in.dont_launch_debugger;
    out.goto_event = in.goto_event;
    out.singlestep_to_event = in.singlestep_to_event;
    out.target_process = in.target_process;
    out.dbg_port = in.dbg_port;
    out.dbg_host = std::string(in.dbg_host);
    out.keep_listening = in.keep_listening;
    std::vector<std::string> gdb_options;
    for (auto option : in.gdb_options){
        gdb_options.push_back(std::string(option));
    }
    out.gdb_options = gdb_options;
    out.gdb_binary_file_path = std::string(in.gdb_binary_file_path);
    out.redirect = in.redirect;
    out.cpu_unbound = in.cpu_unbound;
    out.share_private_mappings = in.share_private_mappings;
    out.dump_interval = in.dump_interval;
    out.serve_files = in.serve_files;
    out.tty = std::string(in.tty);
    return out;
}
int replay(ReplayingFlags flags, rust::String tracedir){
    rr::ReplayFlags rrflags = replay_flags_to_cpp(flags);
    return rr::start_replaying(rrflags, std::string(tracedir));
}
ReplayingFlags get_default_replay_flags(){
    rr::ReplayFlags flags;
    return replay_flags_to_rust(flags);
}
ReplayingFlags replay_flags_pipe_test(ReplayingFlags flags){
    rr::ReplayFlags temp = replay_flags_to_cpp(flags);
    return replay_flags_to_rust(temp);
}


static ReplaySession::Flags session_flags(const rr::ReplayFlags& flags) {
  ReplaySession::Flags result;
  result.redirect_stdio = flags.redirect;
  result.redirect_stdio_file = flags.tty;
  result.share_private_mappings = flags.share_private_mappings;
  result.cpu_unbound = flags.cpu_unbound;
  return result;
}
namespace rr {
std::unique_ptr<ReplayController> new_replay_controller(rust::String trace_dir, ReplayingFlags rust_flags){
    rr::ReplayFlags flags = replay_flags_to_cpp(rust_flags);
    GdbServer::Target target;
    switch (flags.process_created_how) {
    case ReplayFlags::CREATED_EXEC:
      target.pid = flags.target_process;
      target.require_exec = true;
      break;
    case ReplayFlags::CREATED_FORK:
      target.pid = flags.target_process;
      target.require_exec = false;
      break;
    case ReplayFlags::CREATED_NONE:
      break;
    }
    target.event = flags.goto_event;
    ReplaySession::shr_ptr replay_session =
        ReplaySession::create(std::string(trace_dir), session_flags(flags));
    return std::unique_ptr<ReplayController>(new ReplayController(replay_session, target));
}

void ReplayController::print_test_controller() const{
    std::cout << "THEE CRITICAL BEATDOWN" << std::endl;
}
void ReplayController::test_run() {
  while(true) {
    ReplayResult result =
        timeline.replay_step_forward(RUN_CONTINUE);
    if (result.status == REPLAY_EXITED) {
        std::cout << "Debugger was not launched before end of trace" << std::endl;
      return;
    }
  }
}
void ReplayController::setup() {
  ReplayResult result;
  do {
    result =
        timeline.replay_step_forward(RUN_CONTINUE);
    if (result.status == REPLAY_EXITED) {
      // TODO LOG(info) << "Debugger was not launched before end of trace";
      this->is_replay_finished = true;
      return;
    }
  } while (!at_target(result));
  Task* t = timeline.current_session().current_task();
  FrameTime first_run_event = std::max(t->vm()->first_run_event(),
    t->thread_group()->first_run_event());
  if (first_run_event) {
    timeline.set_reverse_execution_barrier_event(first_run_event);
  }
}
bool ReplayController::can_continue_replay() const {
    return !(this->is_replay_finished);
}

GdbRegisterValue new_register_value() {
    GdbRegisterValue reg;
    memset(&reg, 0, sizeof(reg));
    reg.size = 9;
    return reg;
}

} // namespace rr
void printmyval(){
  std::cout << "FYCJ YA" << std::endl;
}
