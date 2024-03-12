module Icmp
  class RubyScoreOnlyStrategy < BaseStrategy
    def compare
      diff_counter = 0

      regions.each do |region|
        compare_region(region) do |a_pixel, b_pixel, x, y|
          next if a_pixel == b_pixel
          diff_counter += 1
        end
      end

      diff_counter.to_f / compared_area.to_f
    end
  end
end
