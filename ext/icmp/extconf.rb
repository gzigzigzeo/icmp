require "mkmf"
$CFLAGS << " -Wall" # rubocop:disable Style/GlobalVars
create_makefile("icmp/icmp")
