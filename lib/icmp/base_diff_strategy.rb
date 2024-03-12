module Icmp
  # Represents abstract comparison strategy with the resulting diff and score
  # based on diff data
  class BaseDiffStrategy < BaseStrategy
    attr_accessor :diff

    def initialize(*args, **kwargs)
      super(*args, **kwargs)

      self.diff = []
    end

    protected

    # Returns comparison scope based on the length of the resulting diff
    def score
      diff.length.to_f / compared_area
    end
  end
end
