# frozen_string_literal: true

require "bundler/gem_tasks"
require "rspec/core/rake_task"
require "benchmark/ips"
require "rake/extensiontask"

RSpec::Core::RakeTask.new(:spec)

require "standard/rake"

task default: %i[compile standard spec]

Rake::ExtensionTask.new("icmp") do |ext|
  ext.lib_dir = "lib/icmp"
  ext.config_options = '--with-cflags="-std=c99"'
end

namespace :performance do
  desc "Run the performance for plain compare"
  task :run do
    require "icmp"

    a = Icmp::Image.from_file(File.expand_path("../spec/fixtures/a.png", __FILE__))
    b = Icmp::Image.from_file(File.expand_path("../spec/fixtures/b.png", __FILE__))

    Benchmark.ips do |x|
      x.report "Ruby" do
        Icmp::RubyStrategy.new(a, b).compare
      end

      x.report "RubySimplified" do
        Icmp::RubySimplifiedStrategy.new(a, b).compare
      end

      x.report "CSimplified" do
        Icmp::CSimplifiedStrategy.new(a, b).compare
      end

      x.report "CBce" do
        Icmp::CBceStrategy.new(a, b).compare
      end

      x.compare!
    end
  end
end
