module Icmp
  class BaseStrategy
    attr_accessor :a, :b, :result

    def initialize(a, b)
      self.a = a
      self.b = b
      self.result = []
    end

    def compare
      raise NotImplementedError
    end
  end
end
