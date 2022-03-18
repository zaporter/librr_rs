#pragma once

#include <string>
#include <iostream>
#include <memory>
#include <cstdint>

#include "rust/cxx.h"
#include "librr-rs/src/replay.rs.h"

#include "util.h"
extern "C++" {
#include "ReplayCommand.h"
#include "ReplaySession.h"
}


int test_replay();
ReplayingFlags get_default_replay_flags();
ReplayingFlags replay_flags_pipe_test(ReplayingFlags flags);
