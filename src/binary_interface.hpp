#pragma once

#include "GdbConnection.h"
#include <iostream>
#include <sys/types.h>
#include "GdbServer.h"

#include <sys/prctl.h>
#include <sys/time.h>
#include <sys/wait.h>
#include <unistd.h>
#include "rust/cxx.h"

#include <limits>
#include "ReplaySession.h"
#include "ReplayCommand.h"
#include "Session.h"
#include "replay.hpp"

#include "Command.h"
#include "Flags.h"
#include "GdbServer.h"
#include "ReplaySession.h"
#include "core.h"
#include <memory>
#include <unistd.h>
namespace rr {


/**
 * This class is a giant wrapper over the functionality provided in GdbServer. 
 * Sadly that functionality is exposed only though a message passing API so 
 * this class creates and passes message to a tricked GdbServer.
 */
class BinaryInterface  {
public: 
  BinaryInterface(std::shared_ptr<ReplaySession> session, const GdbServer::Target& target)
    : s(session, target)
    /*GdbServer( session, target), state(REPORT_NORMAL)*/
  {};

  GdbServer s;
  bool initialize();
  int64_t current_frame_time() const;
  /* ReportState state; */
  /* /1* bool set_query_thread(GdbThreadId); *1/ */
  /* GdbThreadId get_current_thread() const; */
/* rust::String get_exec_file(GdbThreadId request_target) const; */
  /* /1* const std::vector<uint8_t>& get_auxv(); *1/ */
  /* /1* const std::string& get_exec_file(); *1/ */
  /* /1* bool get_is_thread_alive(); *1/ */
  /* /1* const std::string& info get_thread_extra_info(); *1/ */
  /* /1* bool select_thread(); *1/ */
  /* /1* const std::vector<uint8_t>& get_mem(); *1/ */
  /* /1* bool set_mem(); *1/ */
  /* /1* remote_ptr<void> search_mem(); // todo @zack multivariate *1/ */
  /* /1* void get_offsets(); // rr-todo *1/ */
  /* /1* GdbRegisterValue& get_reg(); *1/ */
  /* rust::Vec<GdbRegisterValue> get_regs(pid_t tid) const; */
  /* /1* bool set_reg(); *1/ */
  /* /1* int get_stop_reason(); // todo @ zack multivariate *1/ */
  /* rust::Vec<GdbThreadId> get_thread_list() const; */
  /* /1* bool watchpoint_request(); *1/ */
  /* /1* detach(); *1/ */ 
  /* /1* const std::vector<uint8_t>& read_siginfo(); *1/ */
  /* /1* void write_siginfo(); // rr-todo *1/ */

}; // end class
std::unique_ptr<rr::BinaryInterface> new_binary_interface(int64_t,rust::String);
void beta_test_me(); // replay
void gamma_test_me(); // ?
void delta_test_me();
/* void sayHi(); */

} // end namespace
