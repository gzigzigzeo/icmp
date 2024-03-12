module Icmp
  # BaseStrategy represents abstract comparison strategy
  class BaseStrategy
    attr_accessor :a, :b, :bounds, :regions
    attr_accessor :include_rect, :exclude_rect

    def initialize(a, b, include_rect: nil, exclude_rect: nil)
      if (a.width != b.width) || (a.height != b.height)
        raise ArgumentError, "a and b must have equal dimensions"
      end

      self.a = a
      self.b = b
      self.include_rect = include_rect
      self.exclude_rect = exclude_rect

      # If include_rect is set, we'll use it, otherwise we'll use the whole region
      bounds = include_rect&.xyrb || a.bounding_rect.xyrb
      bounds = Icmp::Rectangle.new(*bounds)
      self.bounds = bounds

      # If there are no exclude rect - than, there's only one region
      if exclude_rect.nil?
        self.regions = [bounds]
        return
      end

      # If exclude_rect is set, there are 2..4 subregions to compare
      self.regions = [
        Rectangle.new(bounds.left, bounds.top, bounds.right, exclude_rect.top - 1),
        Rectangle.new(bounds.left, exclude_rect.top, exclude_rect.left - 1, exclude_rect.bot - 1),
        Rectangle.new(exclude_rect.right, exclude_rect.top, bounds.right, exclude_rect.bot - 1),
        Rectangle.new(bounds.left, exclude_rect.bot, bounds.right, bounds.bot)
      ].filter { |r| !r.empty? } # Remove zero area regions
    end

    def compare
      raise NotImplementedError
    end

    protected

    # compare_region compares region of given x and y and yields a block.
    # Used in Ruby*Strategy.
    def compare_region(area)
      (area.top..area.bot).each do |y|
        range = (area.left..area.right)
        next if b.row(y).slice(range) == a.row(y).slice(range)
        (area.left..area.right).each do |x|
          yield(a[x, y], b[x, y], x, y)
        end
      end
    end

    # Returns actual compared area square
    def compared_area
      @compared_area ||= begin
        return bounds.area if exclude_rect.nil?

        (bounds.area - exclude_rect.area).to_f
      end
    end
  end
end
