#include "binary_interface.hpp"
#include <iostream>
#include <memory>
#include <unistd.h>
#include "util.h"
#include "core.h"
#include "main.h"



#include "Command.h"
#include "Flags.h"
#include "GdbServer.h"
#include "ReplaySession.h"
#include "core.h"
#include <memory>
#include <unistd.h>



#include <sys/prctl.h>
#include <sys/time.h>
#include <sys/wait.h>
#include <unistd.h>

#include <limits>
#include <iostream>

#include "Command.h"
#include "GdbServer.h"
#include "ReplaySession.h"
#include "ScopedFd.h"
#include "kernel_metadata.h"
#include "log.h"
#include "main.h"


using namespace std;

namespace rr {
/* static size_t get_reg(const Registers& regs, const ExtraRegisters& extra_regs, */
/*                       uint8_t* buf, GdbRegister regname, bool* defined) { */
/*   size_t num_bytes = regs.read_register(buf, regname, defined); */
/*   if (!*defined) { */
/*     num_bytes = extra_regs.read_register(buf, regname, defined); */
/*   } */
/*   return num_bytes; */
/* } */

static bool pid_exists(const string& trace_dir, pid_t pid) {
  TraceReader trace(trace_dir);

  while (true) {
    auto e = trace.read_task_event();
    if (e.type() == TraceTaskEvent::NONE) {
      return false;
    }
    if (e.tid() == pid) {
      return true;
    }
  }
}

static bool pid_execs(const string& trace_dir, pid_t pid) {
  TraceReader trace(trace_dir);

  while (true) {
    auto e = trace.read_task_event();
    if (e.type() == TraceTaskEvent::NONE) {
      return false;
    }
    if (e.tid() == pid && e.type() == TraceTaskEvent::EXEC) {
      return true;
    }
  }
}

static int find_pid_for_command(const string& trace_dir,
                                const string& command) {
  TraceReader trace(trace_dir);

  while (true) {
    TraceTaskEvent e = trace.read_task_event();
    if (e.type() == TraceTaskEvent::NONE) {
      return -1;
    }
    if (e.type() != TraceTaskEvent::EXEC) {
      continue;
    }
    if (e.cmd_line().empty()) {
      continue;
    }
    auto& cmd = e.cmd_line()[0];
    if (cmd == command ||
        (cmd.size() > command.size() &&
         cmd.substr(cmd.size() - command.size() - 1) == ('/' + command))) {
      return e.tid();
    }
  }
}
/* static bool set_reg(Task* target, const GdbRegisterValue& reg) { */
/*   if (!reg.defined) { */
/*     return false; */
/*   } */

/*   Registers regs = target->regs(); */
/*   if (regs.write_register(reg.name, reg.value, reg.size)) { */
/*     target->set_regs(regs); */
/*     return true; */
/*   } */

/*   ExtraRegisters extra_regs = target->extra_regs(); */
/*   if (extra_regs.write_register(reg.name, reg.value, reg.size)) { */
/*     target->set_extra_regs(extra_regs); */
/*     return true; */
/*   } */

/*   LOG(warn) << "Unhandled register name " << reg.name; */
/*   return false; */
/* } */
/* static GdbThreadId get_threadid(const Session& session, const TaskUid& tuid) { */
/*   Task* t = session.find_task(tuid); */
/*   pid_t pid = t ? t->tgid() : GdbThreadId::ANY.pid; */
/*   return GdbThreadId(pid, tuid.tid()); */
/* } */

/* static GdbThreadId get_threadid(Task* t) { */
/*   return GdbThreadId(t->tgid(), t->rec_tid); */
/* } */

/* static bool matches_threadid(const GdbThreadId& tid, */
/*                              const GdbThreadId& target) { */
/*   return (target.pid <= 0 || target.pid == tid.pid) && */
/*          (target.tid <= 0 || target.tid == tid.tid); */
/* } */

/* static bool matches_threadid(Task* t, const GdbThreadId& target) { */
/*   GdbThreadId tid = get_threadid(t); */
/*   return matches_threadid(tid, target); */
/* } */

/* static WatchType watchpoint_type(GdbRequestType req) { */
/*   switch (req) { */
/*     case DREQ_SET_HW_BREAK: */
/*     case DREQ_REMOVE_HW_BREAK: */
/*       return WATCH_EXEC; */
/*     case DREQ_SET_WR_WATCH: */
/*     case DREQ_REMOVE_WR_WATCH: */
/*       return WATCH_WRITE; */
/*     case DREQ_REMOVE_RDWR_WATCH: */
/*     case DREQ_SET_RDWR_WATCH: */
/*     // NB: x86 doesn't support read-only watchpoints (who would */
/*     // ever want to use one?) so we treat them as readwrite */
/*     // watchpoints and hope that gdb can figure out what's going */
/*     // on.  That is, if a user ever tries to set a read */
/*     // watchpoint. */
/*     case DREQ_REMOVE_RD_WATCH: */
/*     case DREQ_SET_RD_WATCH: */
/*       return WATCH_READWRITE; */
/*     default: */
/*       FATAL() << "Unknown dbg request " << req; */
/*       return WatchType(-1); // not reached */
/*   } */
/* } */

/* static void maybe_singlestep_for_event(Task* t, GdbRequest* req) { */
/*   if (!t->session().is_replaying()) { */
/*     return; */
/*   } */
/*   auto rt = static_cast<ReplayTask*>(t); */
/*   if (trace_instructions_up_to_event( */
/*           rt->session().current_trace_frame().time())) { */
/*     fputs("Stepping: ", stderr); */
/*     t->regs().print_register_file_compact(stderr); */
/*     fprintf(stderr, " ticks:%" PRId64 "\n", t->tick_count()); */
/*     *req = GdbRequest(DREQ_CONT); */
/*     req->suppress_debugger_stop = true; */
/*     req->cont().actions.push_back( */
/*         GdbContAction(ACTION_STEP, get_threadid(t->session(), t->tuid()))); */
/*   } */
/* } */

int64_t BinaryInterface::current_frame_time() const {
  return s.timeline.current_session().current_frame_time();
}

bool BinaryInterface::initialize(){
  ReplayResult result;
  int i = 0;
  do {
    ++i;
    result = s.timeline.replay_step_forward(RUN_CONTINUE);
    if (result.status == REPLAY_EXITED) {
      //LOG(info) << "Debugger was not launched before end of trace";
      return false;
    }
  } while (!s.at_target(result));
  cout << "Did " << i << "iterations" << endl; 
  return true;
}
  /* Task* t = timeline.current_session().current_task(); */
  /* ScopedFd listen_fd = open_socket(flags.dbg_host.c_str(), &port, probe); */
  /* if (flags.debugger_params_write_pipe) { */
  /*   DebuggerParams params; */
  /*   memset(&params, 0, sizeof(params)); */
  /*   strncpy(params.exe_image, t->vm()->exe_image().c_str(), */
  /*           sizeof(params.exe_image) - 1); */
  /*   strncpy(params.host, flags.dbg_host.c_str(), sizeof(params.host) - 1); */
  /*   params.port = port; */

  /*   ssize_t nwritten = */
  /*       write(*flags.debugger_params_write_pipe, &params, sizeof(params)); */
  /*   DEBUG_ASSERT(nwritten == sizeof(params)); */
  /* } else { */
  /*   fputs("Launch gdb with\n  ", stderr); */
  /*   print_debugger_launch_command(t, flags.dbg_host, port, flags.serve_files, */
  /*                                 flags.debugger_name.c_str(), stderr); */
  /* } */

  /* if (flags.debugger_params_write_pipe) { */
  /*   flags.debugger_params_write_pipe->close(); */
  /* } */
  /* debuggee_tguid = t->thread_group()->tguid(); */

  /* FrameTime first_run_event = std::max(t->vm()->first_run_event(), */
  /*   t->thread_group()->first_run_event()); */
  /* if (first_run_event) { */
  /*   timeline.set_reverse_execution_barrier_event(first_run_event); */
  /* } */

  /* do { */
  /*   LOG(debug) << "initializing debugger connection"; */
  /*   dbg = await_connection(t, listen_fd, GdbConnection::Features()); */
  /*   activate_debugger(); */

  /*   GdbRequest last_resume_request; */
  /*   while (debug_one_step(last_resume_request) == CONTINUE_DEBUGGING) { */
  /*   } */

  /*   timeline.remove_breakpoints_and_watchpoints(); */
  /* } while (flags.keep_listening); */

  /* LOG(debug) << "debugger server exiting ..."; */

/* GdbThreadId BinaryInterface::get_current_thread() const{ */
/*     BinaryInterface* me = const_cast<BinaryInterface*>(this); */
/*     return get_threadid(me->current_session(), last_continue_tuid); */
/* } */
/* rust::Vec<GdbThreadId> BinaryInterface::get_thread_list() const{ */
/*     BinaryInterface* me = const_cast<BinaryInterface*>(this); */
/*     rust::Vec<GdbThreadId> tids; */
/*     if (state != REPORT_THREADS_DEAD) { */
/*       for (auto& kv : me->current_session().tasks()) { */
/*         tids.push_back(get_threadid(me->current_session(), kv.second->tuid())); */
/*       } */
/*     } */
/*     return tids; */

/* } */
/* rust::String BinaryInterface::get_exec_file(GdbThreadId request_target) const { */

/*     string exec_file; */
/*     BinaryInterface* me = const_cast<BinaryInterface*>(this); */

/*     Task* t = nullptr; */
/*     cout << "1" << endl; */
/*     if (request_target.tid) { */
/*       ThreadGroup* tg = me->current_session().find_thread_group(request_target.tid); */
/*       cout << "2" << endl; */
/*       if (tg) { */
/*         t = *tg->task_set().begin(); */
/*         cout << "3" << endl; */
/*       } */
/*     } */ 
/*     if (t) { */
/*       cout << "4" << endl; */
/*       return t->vm()->exe_image(); */
/*     } else { */
/*       return string(""); */
/*     } */
/* } */
/* /1* bool set_query_thread(GdbThreadId query_thread){ *1/ */

/* /1*   bool is_query = false; *1/ */
/* /1*   Task* target = *1/ */
/* /1*       query_thread.tid > 0 *1/ */
/* /1*           ? current_session().find_task(query_thread.tid) *1/ */
/* /1*           : current_session().find_task(is_query ? last_query_tuid : last_continue_tuid); *1/ */

/* /1*   if (target) { *1/ */
/* /1*     if (is_query) { *1/ */
/* /1*       last_query_tuid = target->tuid(); *1/ */
/* /1*     } else { *1/ */
/* /1*       last_continue_tuid = target->tuid(); *1/ */
/* /1*     } *1/ */
/* /1*   } *1/ */
/* /1*   return true; *1/ */
/* /1* } *1/ */

/* rust::Vec<GdbRegisterValue> BinaryInterface::get_regs(pid_t tid) const{ */
/*     BinaryInterface* me = const_cast<BinaryInterface*>(this); */

/*   if (tid==0){ */
/*     // TODO throw error */
/*     // */
/*   } */
/*   bool is_query = true; */
/*   Task* target = */
/*       tid > 0 */
/*           ? me->current_session().find_task(tid) */
/*           : me->current_session().find_task(is_query ? last_query_tuid : last_continue_tuid); */
/*   const Registers& regs = target->regs(); */
/*   const ExtraRegisters& extra_regs = target->extra_regs(); */
/*   GdbRegister end; */
/*   // Send values for all the registers we sent XML register descriptions for. */
/*   // Those descriptions are controlled by GdbConnection::cpu_features(). */
/*   bool have_PKU = dbg->cpu_features() & GdbConnection::CPU_PKU; */
/*   bool have_AVX = dbg->cpu_features() & GdbConnection::CPU_AVX; */
/*   switch (regs.arch()) { */
/*     case x86: */
/*       end = have_PKU ? DREG_PKRU : (have_AVX ? DREG_YMM7H : DREG_ORIG_EAX); */
/*       break; */
/*     case x86_64: */
/*       end = have_PKU ? DREG_64_PKRU : (have_AVX ? DREG_64_YMM15H : DREG_GS_BASE); */
/*       break; */
/*     case aarch64: */
/*       end = DREG_FPCR; */
/*       break; */
/*     default: */
/*       FATAL() << "Unknown architecture"; */
/*   } */
/*   rust::Vec<GdbRegisterValue> rs; */
/*   for (GdbRegister r = GdbRegister(0); r <= end; r = GdbRegister(r + 1)) { */
/*     rs.push_back(get_reg(regs, extra_regs, r)); */
/*   } */
/*     return rs; */


/* } */

static ReplaySession::Flags session_flags(const ReplayFlags& flags) {
  ReplaySession::Flags result;
  result.redirect_stdio = flags.redirect;
  result.redirect_stdio_file = flags.tty;
  result.share_private_mappings = flags.share_private_mappings;
  result.cpu_unbound = flags.cpu_unbound;
  return result;
}

static pid_t waiting_for_child;


static void handle_SIGINT_in_parent(int sig) {
 // DEBUG_ASSERT(sig == SIGINT);
  // Just ignore it.
}

static GdbServer* server_ptr = nullptr;

static void handle_SIGINT_in_child(int sig) {
  //DEBUG_ASSERT(sig == SIGINT);
  if (server_ptr) {
    server_ptr->interrupt_replay_to_target();
  }
}

static int replay(const string& trace_dir, const ReplayFlags& flags) {
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

  // If we're not going to autolaunch the debugger, don't go
  // through the rigamarole to set that up.  All it does is
  // complicate the process tree and confuse users.
  if (flags.dont_launch_debugger) {
    if (target.event == numeric_limits<decltype(target.event)>::max()) {
      //serve_replay_no_debugger(trace_dir, flags);
    } else {
      auto session = ReplaySession::create(trace_dir, session_flags(flags));
      GdbServer::ConnectionFlags conn_flags;
      conn_flags.dbg_port = flags.dbg_port;
      conn_flags.dbg_host = flags.dbg_host;
      conn_flags.debugger_name = flags.gdb_binary_file_path;
      conn_flags.keep_listening = flags.keep_listening;
      conn_flags.serve_files = flags.serve_files;
      GdbServer(session, target).serve_replay(conn_flags);
    }

    // Everything should have been cleaned up by now.
    //check_for_leaks();
    return 0;
  }

  int debugger_params_pipe[2];
  /* if (pipe2(debugger_params_pipe, O_CLOEXEC)) { */
  /*   FATAL() << "Couldn't open debugger params pipe."; */
  /* } */
  if (0 == (waiting_for_child = fork())) {
    // Ensure only the parent has the read end of the pipe open. Then if
    // the parent dies, our writes to the pipe will error out.
    close(debugger_params_pipe[0]);

    {
      prctl(PR_SET_PDEATHSIG, SIGTERM, 0, 0, 0);

      ScopedFd debugger_params_write_pipe(debugger_params_pipe[1]);
      auto session = ReplaySession::create(trace_dir, session_flags(flags));
      GdbServer::ConnectionFlags conn_flags;
      conn_flags.dbg_port = flags.dbg_port;
      conn_flags.dbg_host = flags.dbg_host;
      conn_flags.debugger_params_write_pipe = &debugger_params_write_pipe;
      conn_flags.serve_files = flags.serve_files;
      if (target.event == -1 && target.pid == 0) {
        // If `replay -e` is specified without a pid, go to the exit
        // of the first process (rather than the first exit of a process).
        target.pid = session->trace_reader().peek_frame().tid();
      }
      GdbServer server(session, target);

      server_ptr = &server;
      struct sigaction sa;
      memset(&sa, 0, sizeof(sa));
      sa.sa_flags = SA_RESTART;
      sa.sa_handler = handle_SIGINT_in_child;
      if (sigaction(SIGINT, &sa, nullptr)) {
        FATAL() << "Couldn't set sigaction for SIGINT.";
      }

      server.serve_replay(conn_flags);
    }
    // Everything should have been cleaned up by now.
    check_for_leaks();
    return 0;
  }
  // Ensure only the child has the write end of the pipe open. Then if
  // the child dies, our reads from the pipe will return EOF.
  close(debugger_params_pipe[1]);
  LOG(debug) << getpid() << ": forked debugger server " << waiting_for_child;

  struct sigaction sa;
  memset(&sa, 0, sizeof(sa));
  sa.sa_flags = SA_RESTART;
  sa.sa_handler = handle_SIGINT_in_parent;
  if (sigaction(SIGINT, &sa, nullptr)) {
    FATAL() << "Couldn't set sigaction for SIGINT.";
  }

  {
    ScopedFd params_pipe_read_fd(debugger_params_pipe[0]);
    GdbServer::launch_gdb(params_pipe_read_fd, flags.gdb_binary_file_path,
                          flags.gdb_options,
                          flags.serve_files);
  }

  // Child must have died before we were able to get debugger parameters
  // and exec gdb. Exit with the exit status of the child.
  while (true) {
    int status;
    int ret = waitpid(waiting_for_child, &status, 0);
    int err = errno;
    LOG(debug) << getpid() << ": waitpid(" << waiting_for_child << ") returned "
               << errno_name(err) << "(" << err << "); status:" << HEX(status);
    if (waiting_for_child != ret) {
      if (EINTR == err) {
        continue;
      }
      FATAL() << getpid() << ": waitpid(" << waiting_for_child << ") failed";
    }
    if (WIFEXITED(status) || WIFSIGNALED(status)) {
      LOG(info) << ("Debugger server died.  Exiting.");
      exit(WIFEXITED(status) ? WEXITSTATUS(status) : 1);
    }
  }

  return 0;
}
std::unique_ptr<BinaryInterface> new_binary_interface(int64_t target_event,rust::String trace_dir_rust) {
  rr::ReplayFlags flags;// = replay_flags_to_cpp(rust_replay_flags);
  flags.goto_event = 320;
  /* flags.dont_launch_debugger = true; */
  string trace_dir = std::string(trace_dir_rust);


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
  auto session = ReplaySession::create(trace_dir, session_flags(flags));
  return std::unique_ptr<BinaryInterface>(new BinaryInterface(session, target));
}
void beta_test_me(){
  cout << "BETA" << endl;
  vector<string> args;
  /* args.push_back("-g 1"); */
  /* args.push_back("-a"); */
  auto command = ReplayCommand::get();
  command->run(args);
}

int start_replaying_2(ReplayFlags flags, string trace_dir){
  if (!flags.target_command.empty()) {
    flags.target_process =
        find_pid_for_command(trace_dir, flags.target_command);
    if (flags.target_process <= 0) {
      fprintf(stderr, "No process '%s' found. Try 'rr ps'.\n",
              flags.target_command.c_str());
      return 2;
    }
  }
  if (flags.process_created_how != ReplayFlags::CREATED_NONE) {
    if (!pid_exists(trace_dir, flags.target_process)) {
      fprintf(stderr, "No process %d found in trace. Try 'rr ps'.\n",
              flags.target_process);
      return 2;
    }
    if (flags.process_created_how == ReplayFlags::CREATED_EXEC &&
        !pid_execs(trace_dir, flags.target_process)) {
      fprintf(stderr, "Process %d never exec()ed. Try 'rr ps', or use "
                      "'-f'.\n",
              flags.target_process);
      return 2;
    }
  }
  if (flags.dump_interval > 0 && !flags.dont_launch_debugger) {
    fprintf(stderr, "--stats requires -a\n");
    // TODO ZACK:
    //print_help(stderr);
    return 2;
  }

  assert_prerequisites();

  if (running_under_rr()) {
    if (!Flags::get().suppress_environment_warnings) {
      fprintf(stderr, "rr: rr pid %d running under parent %d. Good luck.\n",
              getpid(), getppid());
    }
    if (trace_dir.empty()) {
      fprintf(stderr,
              "rr: No trace-dir supplied. You'll try to replay the "
              "recording of this rr and have a bad time. Bailing out.\n");
      return 3;
    }
  }

  if (flags.keep_listening && flags.dbg_port == -1) {
    fprintf(stderr,
            "Cannot use --keep-listening (-k) without --dbgport (-s).\n");
    return 4;
  }

  return replay(trace_dir, flags);
}
void gamma_test_me(){
  cout << "GAMMA" << endl;
  string trace_dir;
  ReplayFlags flags;
  
  
  start_replaying(flags, trace_dir);
  //replay(trace_dir,flags);

}
void delta_test_me(){
  cout << "DELTA" << endl;
  /* vector<string> args; */
  /* /1* args.push_back("-g 1"); *1/ */
  /* /1* args.push_back("-a"); *1/ */
  /* auto command = ReplayCommand::get(); */
  /* command->run(args); */
}

/* void sayHi(){ */
/* cout << "FUCK THIS INTEROP" << endl; */
/* } */

} // end namespace rr
  //
