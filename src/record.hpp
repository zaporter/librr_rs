#pragma once
#include <string>
#include <iostream>
#include <memory>
#include <cstdint>

#include "rust/cxx.h"
#include "librr-rs/src/record.rs.h"

#include "util.h"
extern "C++" {
#include "RecordCommand.h"
#include "RecordSession.h"
}



int record(rust::Vec<rust::String> args, RecordingFlags flags);
RecordingFlags get_default_record_flags();
RecordingFlags record_flags_pipe_test(RecordingFlags flags);


