#pragma once
#include <string>
#include <iostream>
#include <memory>
#include <cstdint>

#include "rust/cxx.h"

#include "util.h"
#include "main.h"

extern "C++" {
#include "Flags.h"
#include "RecordCommand.h"
#include "ReplayCommand.h"
#include "RecordSession.h"
}

namespace rr {
void test_replay_cpp();
}
