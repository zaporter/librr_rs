#pragma once
#include <string>
#include <iostream>
#include <memory>
#include <cstdint>

#include "rust/cxx.h"

#include "Dwarf.h"

extern "C++" {
#include "foo.h"
#include "util.h"
#include "main.h"
#include "Flags.h"
#include "RecordCommand.h"
#include "RecordSession.h"
}

struct Zags {
    int date;
    float count;
};

Zags createZags();
void printZags(Zags zags);

int32_t testCPPFunction();
