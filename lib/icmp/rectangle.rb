module Icmp
  # Imatcher::Rectangle
  class Rectangle
    attr_accessor :left, :top, :right, :bot

    def initialize(l, t, r, b)
      @left = l
      @top = t
      @right = r
      @bot = b
    end

    def area
      (right - left + 1) * (bot - top + 1)
    end

    def xyrb
      [left, top, right, bot]
    end

    def contains_point?(x, y)
      x.between?(left, right) && y.between?(top, bot)
    end

    def empty?
      left == right || top == bot
    end
  end
end
