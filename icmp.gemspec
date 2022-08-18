# frozen_string_literal: true

require_relative "lib/icmp/version"

Gem::Specification.new do |spec|
  spec.name = "icmp"
  spec.version = Icmp::VERSION
  spec.authors = ["Victor Sokolov"]
  spec.email = ["gzigzigzeo@gmail.com"]

  spec.summary = "Image comparsion test library"
  spec.description = "Image comparsion test library"
  spec.homepage = "https://github.com/gzigzigzeo/icmp"
  spec.license = "MIT"
  spec.required_ruby_version = ">= 3.1.0"

  spec.metadata["allowed_push_host"] = "https://rubygems.org"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = "https://github.com/gzigzigzeo/icmp"

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  spec.files = Dir.chdir(__dir__) do
    `git ls-files -z`.split("\x0").reject do |f|
      (f == __FILE__) || f.match(%r{\A(?:(?:bin|test|spec|features)/|\.(?:git|travis|circleci)|appveyor)})
    end
  end
  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]
  spec.extensions = ["ext/icmp/extconf.rb"]
end
