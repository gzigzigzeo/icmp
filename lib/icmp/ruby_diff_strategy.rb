module Icmp
  # Calculates difference of two images using Ruby (with diff)
  class RubyDiffStrategy < BaseDiffStrategy
    def initialize(*args, **kwargs)
      super(*args, **kwargs)
    end

    def compare
      regions.each do |region|
        compare_region(region) do |a_pixel, b_pixel, x, y|
          next if a_pixel == b_pixel
          update_diff(a_pixel, b_pixel, x, y)
        end
      end

      score
    end

    protected

    def update_diff(a, b, x, y)
      diff << [a, b, x, y]
    end
  end
end
