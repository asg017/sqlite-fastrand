require "version"

module SqliteFastrand
  class Error < StandardError; end
  def self.fastrand_loadable_path
    File.expand_path('../fastrand0', __FILE__)
  end
  def self.load(db)
    db.load_extension(self.fastrand_loadable_path)
  end
end
