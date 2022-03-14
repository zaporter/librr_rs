#pragma once
#include <string>
#include <iostream>
#include <memory>
#include <cstdint>
//#include "librr-rs/src/zags.rs.h"

struct Zags {
    int date;
    float count;
};
Zags createZags();
void printZags(Zags zags);
