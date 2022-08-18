require "spec_helper"

RSpec.describe do
  let(:a) { Icmp::Image.from_file(File.expand_path("../fixtures/a.png", __FILE__)) }
  let(:b) { Icmp::Image.from_file(File.expand_path("../fixtures/b.png", __FILE__)) }

  [
    Icmp::RubyStrategy,
    Icmp::RubySimplifiedStrategy,
    Icmp::CSimplifiedStrategy,
    Icmp::CBceStrategy,
    Icmp::CNormalStrategy, # rubocop:disable Style/TrailingCommaInArrayLiteral
  ].each do |strategy|
    describe strategy, "#compare" do
      context "a and a" do
        subject { described_class.new(a, a) }

        it { expect(subject.compare).to eq(0) }
      end

      context "a and b" do
        subject { described_class.new(a, b) }

        it { expect(subject.compare.round(4)).to eq(0.0167) }
      end
    end
  end

  describe Icmp::CNormalStrategy, "#regions" do
    subject { described_class.new(a, b, exclude_rect: Icmp::Rectangle.new(5, 5, 10, 10)) }

    it { expect(subject.regions.size).to eq(4) }
    it { expect(subject.regions[0].bounds).to eq([0, 0, 479, 4]) }
    it { expect(subject.regions[1].bounds).to eq([0, 5, 4, 9]) }
    it { expect(subject.regions[2].bounds).to eq([10, 5, 479, 9]) }
    it { expect(subject.regions[3].bounds).to eq([0, 10, 479, 431]) }
  end
end
