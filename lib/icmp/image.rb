require "chunky_png"

module Icmp
  class Image < ::ChunkyPNG::Image
    def bounding_rect
      @bounding_rect ||= Rectangle.new(0, 0, width - 1, height - 1)
    end
  end
end
