#!/usr/bin/env fish
# print the readme example in each script

set fonts superscript subscript script script-bold fullwidth gothic gothic-bold sans sans-italic monospace sans-bold sans-bold-italic serif-bold small-caps circled circled-negative squared squared-negative double-struck inverted reversed faux-cyrillic

set max_length 0
for i in $fonts
  if test (string length $i) -gt $max_length
    set max_length (string length $i)
  end
end

for i in $fonts
  set padding (math $max_length - (string length $i))
  set spaces (string repeat -n $padding " ")
  set s (./target/debug/runi $i 'abcdefghijklmnopqrstuvwxyz')

  echo $i: $spaces $s

end

