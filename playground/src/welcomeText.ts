const WELCOME_TEXT = `
-- Welcome to the χ playground!

-- Small example program
-- Note: meta variables and assignments
-- (let name = <some expr>;) are supported 
let add = rec add = \\x. \\y. case x of
{ Zero() -> y
; Suc(n) -> Suc(add n y)
};

let zero = Zero();

let three = Suc(Suc(Suc(Zero())));

let equals = rec equals = \\m. \\n. case m of
{ Zero() -> case n of
  { Zero() -> True()
  ; Suc(n) -> False()
  }
; Suc(m) -> case n of
  { Zero() -> False()
  ; Suc(n) -> equals m n
  }
};

equals (add zero three) three
-- the value of the last expression is printed in the right window
-- (each time the contents of the editor changes).


-- if you want to save your program you can create
-- a Github Gist and open it by adding ?gist=<gist id> to the URL.
`;

export default WELCOME_TEXT;