require "spec_helper"

RSpec.describe Icmp do
  let(:a) { Icmp::Image.from_file(File.expand_path("../fixtures/a.png", __FILE__)) }
  let(:b) { Icmp::Image.from_file(File.expand_path("../fixtures/b.png", __FILE__)) }

  [
    Icmp::RubyDiffStrategy,
    Icmp::RubyScoreOnlyStrategy,
    Icmp::CDiffStrategy,
    Icmp::CScoreOnlyStrategy,
    Icmp::CRawDiffStrategy,
    Icmp::CRawScoreOnlyStrategy
    # Icmp::CNormalBceStrategy,
    # Icmp::RustNormalStrategy
  ].each do |s|
    describe s, "#compare" do
      context "when a is compared to a" do
        subject(:strategy) { described_class.new(a, a) }

        it { expect(strategy.compare).to eq(0) }
      end

      context "when a is compared to b" do
        subject(:strategy) { described_class.new(a, b) }

        it { expect(strategy.compare.round(4)).to eq(0.0167) }
      end
    end
  end
end
