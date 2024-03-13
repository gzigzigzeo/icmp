# Icmp

This project mocks `imatcher` gem, but with comparison logic written in rust/C.

`bundle exec rake compile RB_SYS_CARGO_PROFILE=release && bundle exec rspec spec/icmp_spec.rb && bundle exec rake performance:run`