#pragma once

#include "GdbConnection.h"
#include "BinaryInterface.h"
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
std::unique_ptr<BinaryInterface> new_binary_interface(int64_t,rust::String);
} // end namespace rr
