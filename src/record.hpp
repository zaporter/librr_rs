#pragma once
#include <string>
#include <iostream>
#include <memory>
#include <cstdint>

#include "rust/cxx.h"

#include "util.h"
extern "C++" {
#include "RecordCommand.h"
#include "RecordSession.h"
}


namespace rr {
int record(rust::Vec<rust::String> args);
/* RecordingFlags get_default_record_flags(); */
/* RecordingFlags record_flags_pipe_test(RecordingFlags flags); */

} // end namespace rr
