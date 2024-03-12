# frozen_string_literal: true

require "bundler/gem_tasks"
require "rspec/core/rake_task"
require "benchmark/ips"
require "rake/extensiontask"

RSpec::Core::RakeTask.new(:spec)

require "standard/rake"

task default: %i[compile standard spec]

Rake::ExtensionTask.new("icmp_c") do |ext|
  ext.name = "icmp_c"
  ext.ext_dir = "ext/icmp"
  ext.lib_dir = "lib/icmp"
  # ext.config_options = '--with-cflags="-std=c99"'
  ext.config_script = "extconf_c.rb"
end

Rake::ExtensionTask.new("icmp_rust") do |ext|
  ext.name = "icmp_rust"
  ext.ext_dir = "ext/icmp"
  ext.lib_dir = "lib/icmp"
  ext.config_script = "extconf_rust.rb"
end

namespace :performance do
  desc "Run the performance for plain compare"
  task :run do
    require "icmp"

    a = Icmp::Image.from_file(File.expand_path("../spec/fixtures/a.png", __FILE__))
    b = Icmp::Image.from_file(File.expand_path("../spec/fixtures/b.png", __FILE__))

    Benchmark.ips do |x|
      x.report "Ruby (diff)" do
        Icmp::RubyDiffStrategy.new(a, b).compare
      end

      x.report "Ruby (score only)" do
        Icmp::RubyScoreOnlyStrategy.new(a, b).compare
      end

      x.report "C (diff)" do
        Icmp::CDiffStrategy.new(a, b).compare
      end

      x.report "C (score only)" do
        Icmp::CScoreOnlyStrategy.new(a, b).compare
      end

      x.report "C (raw diff)" do
        Icmp::CRawDiffStrategy.new(a, b).compare
      end

      x.report "C (raw score only)" do
        Icmp::CRawScoreOnlyStrategy.new(a, b).compare
      end

      # x.report "CNormal" do
      #   Icmp::CNormalStrategy.new(a, b).compare
      # end

      # x.report "CNormalBce" do
      #   Icmp::CNormalBceStrategy.new(a, b).compare
      # end

      # x.report "RustNormal" do
      #   Icmp::RustNormalStrategy.new(a, b).compare
      # end

      x.compare!(order: :baseline)
    end
  end
end
