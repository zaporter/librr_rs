#pragma once
#include <string>
#include <iostream>
#include <memory>
#include <cstdint>

#include "rust/cxx.h"
#include "librr-rs/src/main.rs.h"


#include "util.h"
extern "C++" {
#include "Dwarf.h"
#include "foo.h"
#include "main.h"
#include "Flags.h"
#include "RecordCommand.h"
#include "RecordSession.h"
}



void tryRecordCommand();
int record(rust::Vec<rust::String> args, RecordingFlags flags);

RecordingFlags getDefaultRecordFlags();
RecordingFlags recordFlagsPipeTest(RecordingFlags flags);


int32_t testCPPFunction();
