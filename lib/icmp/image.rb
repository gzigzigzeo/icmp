require "chunky_png"
require "fiddle"

module Icmp
  class Image < ::ChunkyPNG::Image
    def bounding_rect
      @bounding_rect ||= Rectangle.new(0, 0, width - 1, height - 1)
    end
  end

  class ImageBuffer
    attr_accessor :buffer, :width, :height, :bounding_rect, :raw_box_ptr

    def initialize(buffer, width, height)
      self.buffer = buffer
      self.width = width
      self.height = height
      self.raw_box_ptr = new_box_image_from_buffer

      ObjectSpace.define_finalizer(self, self.class.create_finalizer(buffer, raw_box_ptr))
    end

    # NOTE: This cop returns false positive
    # rubocop:disable Lint/DuplicateMethods
    def bounding_rect
      @bounding_rect ||= Rectangle.new(0, 0, width - 1, height - 1)
    end
    # rubocop:enable Lint/DuplicateMethods

    def new_box_image_from_buffer
      raise NotImplementedError
    end

    class << self
      def from_file(path)
        png_header = ChunkyPNG::Datastream.from_file(path).header_chunk
        width, height = png_header.width, png_header.height

        File.open(path, "rb") do |file|
          file_size = file.size
          buffer = Fiddle::Pointer.malloc(file_size)

          file.each_byte.with_index do |byte, index|
            buffer[index] = byte
          end

          return new(buffer, width, height)
        end
      end

      def destroy_raw_box_ptr(raw_box_ptr)
        raise NotImplementedError
      end

      def create_finalizer(buffer, raw_box_ptr)
        proc {
          buffer.free
          destroy_raw_box_ptr(raw_box_ptr)
        }
      end
    end
  end
end
