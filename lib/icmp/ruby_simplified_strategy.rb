module Icmp
  class RubySimplifiedStrategy < BaseStrategy
    def compare
      diff_counter = 0

      area = a.bounding_rect
      (area.top..area.bot).each do |y|
        range = (area.left..area.right)
        next if b.row(y).slice(range) == a.row(y).slice(range)
        (area.left..area.right).each do |x|
          a_pixel = a[x, y]
          b_pixel = b[x, y]

          next if a_pixel == b_pixel
          diff_counter += 1
        end
      end

      diff_counter.to_f / area.area
    end
  end
end
