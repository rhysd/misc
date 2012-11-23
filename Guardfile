# This is Guardfile to compile LaTeX files automatically

guard :shell do
  watch(%r{(.+)\.tex$}) do |m|
    base = File.basename(m[0], '.tex')
    `platex -interaction=nonstopmode #{base}`
    `platex -interaction=nonstopmode #{base}`
    `pbibtex #{base}`
    `dvipdfmx #{base}`
  end
end
