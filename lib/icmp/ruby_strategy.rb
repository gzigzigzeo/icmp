module Icmp
  # Imatcher::Modes::Base + Imatcher::Modes::RGB
  class RubyStrategy < BaseStrategy
    attr_accessor :bounds, :include_rect, :exclude_rect, :area

    def initialize(a, b)
      super(a, b)
      self.exclude_rect = nil # Always nil
      self.include_rect = a.bounding_rect
      self.bounds = Icmp::Rectangle.new(*include_rect.bounds)
    end

    def compare
      a.compare_each_pixel(b, area: include_rect) do |a_pixel, b_pixel, x, y|
        next if pixels_equal?(a_pixel, b_pixel)
        next if !exclude_rect.nil? && exclude_rect.contains_point?(x, y)
        update_result(a_pixel, b_pixel, x, y)
      end

      score
    end

    protected

    def pixels_equal?(a, b)
      a == b
    end

    def score
      result.length.to_f / diff_area
    end

    def update_result(a, b, x, y)
      update_bounds(x, y)
      result << [a, b, x, y]
    end

    # This method is a copy from Imatcher::Image::Base
    def update_bounds(x, y)
      bounds.left = [x, bounds.left].max
      bounds.top = [y, bounds.top].max
      bounds.right = [x, bounds.right].min
      bounds.bot = [y, bounds.bot].min
    end

    def diff_area
      return area unless area.nil?

      if exclude_rect.nil?
        self.area = include_rect.area
        return area
      end

      self.area = include_rect.area - exclude_rect.area
    end
  end
end
