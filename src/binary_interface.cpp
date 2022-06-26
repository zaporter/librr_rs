#include "binary_interface.hpp"
#include <iostream>
#include <memory>
#include <unistd.h>


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


/* bool BinaryInterface::initialize(){ */
/*   ReplayResult result; */
/*   do { */
/*     result = timeline.replay_step_forward(RUN_CONTINUE); */
/*     cout << "hehe" << endl; */
/*     cout << "target.event" << target.event << endl; */
/*     if (result.status == REPLAY_EXITED) { */
/*       LOG(info) << "Debugger was not launched before end of trace"; */
/*       return false; */
/*     } */
/*   } while (!at_target(result)); */

/*   Task* t = timeline.current_session().current_task(); */
/*   debuggee_tguid = t->thread_group()->tguid(); */

/*   FrameTime first_run_event = std::max(t->vm()->first_run_event(), */
/*     t->thread_group()->first_run_event()); */
/*   if (first_run_event) { */
/*     timeline.set_reverse_execution_barrier_event(first_run_event); */
/*   } */

/*   cout << "initialized" << endl; */

/*   return true; */

/* } */

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
std::unique_ptr<BinaryInterface> new_binary_interface(rust::String trace_dir_rust) {
  rr::ReplayFlags flags;// = replay_flags_to_cpp(rust_replay_flags);
  flags.goto_event = 1;
  flags.dont_launch_debugger = true;
  string trace_dir = std::string(trace_dir_rust);

  /* cout << "TRACE_DIR : "<<trace_dir << endl; */
  /* cout << "goto_event : "<<flags.goto_event << endl; */
  /* cout << "singlestep_to_event : "<<flags.singlestep_to_event << endl; */
  /* cout << "target_process : "<<flags.target_process << endl; */
  /* cout << "created_how : "<<flags.process_created_how << endl; */
  /* cout << "target_process : "<<flags.target_process << endl; */
  /* cout << "target_command : "<<flags.target_command << endl; */
  /* cout << "dont_launch_debugger : "<<flags.dont_launch_debugger << endl; */
  /* cout << "keep_listening : "<<flags.keep_listening << endl; */
  /* cout << "redirect : "<<flags.redirect << endl; */
  /* cout << "keep_listening : "<<flags.keep_listening << endl; */
  /* cout << "cpu_unbound : "<<flags.cpu_unbound << endl; */
  /* cout << "share_private_mappings : "<<flags.share_private_mappings << endl; */

  /* cout << "tty : "<<flags.tty << endl; */
  /* cout << "share_private_mappings : "<<flags.share_private_mappings << endl; */

  /* cout << "serve_files : "<<flags.serve_files << endl; */
  /* { */
  /*   std::vector<std::string> v = { "-a" , "-g", "1"}; */
  /*   auto command = ReplayCommand::get(); */
  /*   command->run(v); */
  /*   cout << "DONE CONNAMD : " << endl; */
  /* } */

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
  /* cout << "creating server" << endl; */
  /* auto server = new GdbServer(session, target); */
  /* cout << "running server" << endl; */
  /* server->serve_replay(conn_flags); */
  /* cout << "Finished gdb server" << endl; */
  /* auto bin_interface = new BinaryInterface(session, target); */
  /* bin_interface->initialize(); */
  /* cout << "ENDED" << endl; */
  return std::unique_ptr<BinaryInterface>(new BinaryInterface());
}
/* void sayHi(){ */
/* cout << "FUCK THIS INTEROP" << endl; */
/* } */

} // end namespace rr
  //
