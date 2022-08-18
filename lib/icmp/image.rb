require "chunky_png"

module Icmp
  # Imatcher::Image
  class Image < ::ChunkyPNG::Image
    def compare_each_pixel(other, area: nil)
      area = a.bounding_rect if area.nil?
      (area.top..area.bot).each do |y|
        range = (area.left..area.right)
        next if other.row(y).slice(range) == row(y).slice(range)
        (area.left..area.right).each do |x|
          yield(self[x, y], other[x, y], x, y)
        end
      end
    end

    def bounding_rect
      Rectangle.new(0, 0, width - 1, height - 1)
    end
  end
end
