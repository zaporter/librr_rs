#include "replay.hpp"

using namespace std;

int test_replay(){
    rr::ReplayFlags flags;
    string tracedir;
    rr::start_replaying(flags,tracedir);
    return 0;
}
ReplayingFlags replay_flags_to_rust(rr::ReplayFlags in) {
    ReplayingFlags out = {};
    out.dont_launch_debugger = in.dont_launch_debugger;
    out.goto_event = in.goto_event;
    out.singlestep_to_event = in.singlestep_to_event;
    out.target_process = in.target_process;
    out.dbg_port = in.dbg_port;
    out.dbg_host = rust::String(in.dbg_host);
    out.keep_listening = in.keep_listening;
    rust::Vec<rust::String> gdb_options;
    for (auto option : in.gdb_options){
        gdb_options.push_back(rust::String(option));
    }
    out.gdb_options = gdb_options;
    out.gdb_binary_file_path = rust::String(in.gdb_binary_file_path);
    out.redirect = in.redirect;
    out.cpu_unbound = in.cpu_unbound;
    out.share_private_mappings = in.share_private_mappings;
    out.dump_interval = in.dump_interval;
    out.serve_files = in.serve_files;
    out.tty = rust::String(in.tty);
    return out;
}
rr::ReplayFlags replay_flags_to_cpp(ReplayingFlags in){
    rr::ReplayFlags out;
    out.dont_launch_debugger = in.dont_launch_debugger;
    out.goto_event = in.goto_event;
    out.singlestep_to_event = in.singlestep_to_event;
    out.target_process = in.target_process;
    out.dbg_port = in.dbg_port;
    out.dbg_host = std::string(in.dbg_host);
    out.keep_listening = in.keep_listening;
    std::vector<std::string> gdb_options;
    for (auto option : in.gdb_options){
        gdb_options.push_back(std::string(option));
    }
    out.gdb_options = gdb_options;
    out.gdb_binary_file_path = std::string(in.gdb_binary_file_path);
    out.redirect = in.redirect;
    out.cpu_unbound = in.cpu_unbound;
    out.share_private_mappings = in.share_private_mappings;
    out.dump_interval = in.dump_interval;
    out.serve_files = in.serve_files;
    out.tty = std::string(in.tty);
    return out;
}
ReplayingFlags get_default_replay_flags(){
    rr::ReplayFlags flags;
    return replay_flags_to_rust(flags);
}
ReplayingFlags replay_flags_pipe_test(ReplayingFlags flags){
    rr::ReplayFlags temp = replay_flags_to_cpp(flags);
    return replay_flags_to_rust(temp);
}
