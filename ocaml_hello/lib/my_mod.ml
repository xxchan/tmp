module MyMod = struct
  type t = int
  let hello = print_endline "Hello, my mod!"
end

let hello = MyMod.hello