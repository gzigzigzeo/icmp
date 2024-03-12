require "mkmf"
require "rb_sys/mkmf"

create_rust_makefile("icmp/icmp_rust") do |ext|
  ext.profile = ENV.fetch("RB_SYS_CARGO_PROFILE", :dev).to_sym
  ext.ext_dir = "rust"
end
