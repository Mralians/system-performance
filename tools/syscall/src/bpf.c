BPF_PERF_OUTPUT(output);

struct data_t {
  u32 pid;
  u32 uid;
  char command[16];
};

int execve_counter(void *ctx) {
  struct data_t data = {};
  data.pid = bpf_get_current_pid_tgid() >> 32;
  data.uid = bpf_get_current_uid_gid() & 0XFFFFFFFF;
  bpf_get_current_comm(data.command, sizeof(data.command));
  output.perf_submit(ctx, &data, sizeof(data));
  return 0;
}
