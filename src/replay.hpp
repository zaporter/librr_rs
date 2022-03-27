#pragma once

#include <string>
#include <iostream>
#include <memory>
#include <cstdint>

#include "rust/cxx.h"
#include "util.h"

extern "C++" {
#include "ReplayCommand.h"
#include "ReplaySession.h"
#include "GdbServer.h"
}
namespace rr {

class ReplayController : GdbServer {
public:
    ReplayController(rr::ReplaySession::shr_ptr session, const Target& target) : GdbServer(session,target) {
        this->is_replay_finished = false;
    };

    void print_test_controller() const;
    void setup();
    void test_run();
    bool can_continue_replay() const;
    /*
        
static size_t get_reg(const Registers& regs, const ExtraRegisters& extra_regs,
                      uint8_t* buf, GdbRegister regname, bool* defined) {

     */

private:
    bool is_replay_finished;
};
} // namespace rr

// I know that this is bad practice.
// I have this here because replay.rs.h requires ReplayController to be defined
// If you have a better way of solving this, please let me know.
#include "librr-rs/src/replay.rs.h"


int replay(ReplayingFlags flags, rust::String tracedir);
ReplayingFlags get_default_replay_flags();
void printmyval();
ReplayingFlags replay_flags_pipe_test(ReplayingFlags flags);

// This is down here because it uses ReplayingFlags defined in replay.rs.h
namespace rr {
std::unique_ptr<ReplayController> new_replay_controller(rust::String trace_dir, ReplayingFlags flags);

GdbRegisterValue new_register_value() ;
}
