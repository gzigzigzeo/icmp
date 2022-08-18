module Icmp
  class CNormalStrategy < BaseStrategy
    attr_accessor :bounds, :include_rect, :exclude_rect, :regions

    # Imatcher::Matcher guarantess the following:
    #   - a dimensions equal b dimensions
    #   - exclude_rect is within those dimensions
    #   - exclude_rect is within include_rect bounds
    def initialize(a, b, include_rect: nil, exclude_rect: nil)
      super(a, b)
      self.exclude_rect = exclude_rect
      self.include_rect = include_rect
      self.bounds = Icmp::Rectangle.new(*(include_rect&.bounds || a.bounding_rect.bounds))

      # We do not exclude anything: there's only one region which equals bounds
      if exclude_rect.nil?
        self.regions = [self.bounds]
      else
        self.regions = [
          Rectangle.new(bounds.left, bounds.top, bounds.right, exclude_rect.top - 1),
          Rectangle.new(bounds.left, exclude_rect.top, exclude_rect.left - 1, exclude_rect.bot - 1),
          Rectangle.new(exclude_rect.right, exclude_rect.top, bounds.right, exclude_rect.bot - 1),
          Rectangle.new(bounds.left, exclude_rect.bot, bounds.right, bounds.bot),
        ].filter { |r| !r.empty? }
      end
    end
  end
end
