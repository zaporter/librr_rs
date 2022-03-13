#pragma once
#include <string>
#include <iostream>
#include <memory>
#include <cstdint>

#include "rust/cxx.h"
#include "librr-rs/src/main.rs.h"

#include "Dwarf.h"

#include "util.h"
extern "C++" {
#include "foo.h"
#include "main.h"
#include "Flags.h"
#include "RecordCommand.h"
#include "RecordSession.h"
}

struct Zags {
    int date;
    float count;
};
//namespace librr {
/*struct RecordingFlags {
    int ignore_sig;
    int print_trace_dir;
};*/
//}; // end namespace

Zags createZags();
void printZags(Zags zags);

void tryRecordCommand();
int record(rust::Vec<rust::String> args, RecordingFlags flags);

RecordingFlags getDefaultRecordFlags();
RecordingFlags recordFlagsPipeTest(RecordingFlags flags);


int32_t testCPPFunction();
