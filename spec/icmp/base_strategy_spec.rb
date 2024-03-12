require "spec_helper"

RSpec.describe Icmp::BaseStrategy do
  let(:a) { Icmp::Image.from_file(File.expand_path("../../fixtures/a.png", __FILE__)) }
  let(:b) { Icmp::Image.from_file(File.expand_path("../../fixtures/b.png", __FILE__)) }

  describe "without include/exclude" do
    subject(:strategy) { described_class.new(a, b) }

    it { expect(strategy.bounds.xyrb).to eq([0, 0, 479, 431]) }
    it { expect(strategy.regions.size).to eq(1) }
    it { expect(strategy.bounds.xyrb).to eq(strategy.regions[0].xyrb) }
  end

  describe "with include/exclude" do
    subject(:strategy) do
      described_class.new(a, b, exclude_rect: Icmp::Rectangle.new(5, 5, 10, 10))
    end

    it { expect(strategy.regions.size).to eq(4) }
    it { expect(strategy.regions[0].xyrb).to eq([0, 0, 479, 4]) }
    it { expect(strategy.regions[1].xyrb).to eq([0, 5, 4, 9]) }
    it { expect(strategy.regions[2].xyrb).to eq([10, 5, 479, 9]) }
    it { expect(strategy.regions[3].xyrb).to eq([0, 10, 479, 431]) }
  end
end
