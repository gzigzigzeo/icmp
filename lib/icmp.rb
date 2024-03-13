# frozen_string_literal: true

require_relative "icmp/version"

require_relative "icmp/rectangle"
require_relative "icmp/image"

require_relative "icmp/base_strategy"
require_relative "icmp/base_diff_strategy"
require_relative "icmp/base_score_only_strategy"

require_relative "icmp/ruby_diff_strategy"
require_relative "icmp/ruby_score_only_strategy"

require_relative "icmp/c_diff_strategy"
require_relative "icmp/c_score_only_strategy"
require_relative "icmp/c_raw_diff_strategy"
require_relative "icmp/c_raw_score_only_strategy"

require_relative "icmp/rust_diff_strategy"
require_relative "icmp/rust_score_only_strategy"
require_relative "icmp/rust_semi_raw_score_only_strategy"

require_relative "icmp/icmp_c"
require_relative "icmp/icmp_rust"

module Icmp
end
